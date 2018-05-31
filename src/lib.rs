// TODO Remove Range::Not
// TODO Implement other "not"'s in the Pattern enum.
// TODO This is good: `!monkey*`, but this is weird: `"!monkey+"`.
// TODO Refactor.
// TODO There is a bug when using ranges in a not (!).
// TODO Update Cargo.toml, use online crate of Rule.
// TODO Error messages.
// TODO Remove most panics.

// Copyright (c) 2015-2018 Vincent van Ingen <code@abitvin.net>
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>
// This file may not be copied, modified, or distributed except according to those terms.

extern crate rule;

mod ast;
mod rules;

use ast::{Clause, ParseData};
use rule::{BranchFn, Rule, RuleError};
use rules::root;
use std::collections::HashMap;

struct GrammarRule<T> { 
    rule: Rule<T>,
    sentence: Vec<Clause>,
}

type GrammarRules<T> = HashMap<String, GrammarRule<T>>;

pub struct Grammar<T> {
    compiled: bool,
    rules: GrammarRules<T>,
    parser: Rule<ParseData>,
    ws: GrammarRule<T>,
}

impl<T> Grammar<T> {
    pub fn new() -> Self {
        Self::new_("(\\ |\t|\n|\r)")
    }

    pub fn new_with_ws(expr: &str) -> Self {
        Self::new_(expr)
    }

    fn new_(ws_expr: &str) -> Self {
        let parser = root();
        
        let ws = GrammarRule {
            rule: Rule::new(None),
            sentence: parse(&parser, &ws_expr).unwrap(),
        };

        Self {
            compiled: false,
            rules: HashMap::new(),
            ws,
            parser,
        }
    }

    pub fn add(&mut self, id: &str, expr: &str, branch_fn: BranchFn<T>) {
        if self.compiled {
            // TODO Improve, this panic is lazy. We can parse/compile anytime (because we cannot remove rules). 
            panic!("Cannot alter Grammar when being used.");
        }

        match parse(&self.parser, expr) {
            Ok(sentence) => {
                let rule = Rule::new(branch_fn);

                if self.rules.insert(String::from(id), GrammarRule { rule, sentence }).is_some() {
                    panic!("The rule \"{}\" already used.", id);
                }
            },
            Err(err) => {
                panic!("Error parsing rule \"{}\": {:?}", id, err)
            },
        }
    }

    pub fn scan(&mut self, root_id: &str, code: &str) -> Result<Vec<T>, String> {
        if !self.compiled {
            let dummy = Rule::new(None);
            self.ws.code_gen(&self.rules, &dummy)?;

            for (_, r) in &self.rules {
                r.code_gen(&self.rules, &self.ws.rule)?;
            }

            self.compiled = true;
        }
        
        if let Some(root) = &self.rules.get(root_id) {
            root.rule.scan(code)
                .map_err(|_| String::from("TODO ERROR"))        // TODO Error messages.
        }
        else {
            return Err(format!("Rule \"{}\" not found.", root_id));
        }   
    }
}

fn parse(parser: &Rule<ParseData>, expr: &str) -> Result<Vec<Clause>, Vec<RuleError>> {
    parser.scan(expr)
        .map(|parse_data| parse_data.into_iter().map(|x| x.unwrap_clause()).collect())
}

impl<T> GrammarRule<T> {
    fn code_gen(&self, all_rules: &GrammarRules<T>, ws: &Rule<T>) -> Result<(), String> {
        let is_one = self.sentence.len() == 1;
        
        for clause in &self.sentence {
            let target = if is_one {
                self.rule.clone()
            }
            else {
                Rule::new(None)
            };

            match clause {
                Clause::AlterTexts { ref replacements, min, max } => {
                    let replacements = replacements.iter()
                        .map(|x| (x.find.clone(), x.replace.clone()))
                        .collect();

                    if *min == 1 && *max == 1 {
                        target.alter_string(replacements);
                    }
                    else {
                        let rule = Rule::new(None);
                        rule.alter_string(replacements);
                        target.between(*min, *max, &rule);
                    }
                }
                Clause::AnyChar { min, max } => {
                    if *min == 1 && *max == 1 {
                        target.any_char();
                    }
                    else {
                        let rule = Rule::new(None);
                        rule.any_char();
                        target.between(*min, *max, &rule);
                    }
                },
                Clause::AnyCharExcept { ref chars, min, max } => {
                    if *min == 1 && *max == 1 {
                        target.any_char_except(chars.clone());
                    }
                    else {
                        let rule = Rule::new(None);
                        rule.any_char_except(chars.clone());
                        target.between(*min, *max, &rule);
                    }
                },
                Clause::AnyOf { ref sentences, min, max } => {
                    let mut rules = vec![];

                    for sentence in sentences {
                        let rule = Rule::new(None);

                        for clause in sentence {
                            let gram_rule = GrammarRule {
                                rule: Rule::new(None),
                                sentence: vec![clause.clone()],     // TODO Improve.
                            };

                            gram_rule.code_gen(all_rules, ws)?;
                            rule.one(&gram_rule.rule);
                        }

                        rules.push(rule);
                    }

                    let rules = rules.iter()
                        .map(|x| x)
                        .collect();
                    
                    if *min == 1 && *max == 1 {
                        target.any_of(rules);

                    }
                    else {
                        let rule = Rule::new(None);
                        rule.any_of(rules);
                        target.between(*min, *max, &rule);
                    }
                },
                Clause::CharRanges { ref ranges, min, max } => {
                    let rules: Vec<Rule<_>> = ranges.iter()
                        .map(|r| {
                            let rule = Rule::new(None);
                            rule.char_in(r.start, r.end);
                            rule
                        })
                        .collect();
                    
                    let rules = rules.iter()
                        .map(|x| x)
                        .collect();
                    
                    if *min == 1 && *max == 1 {
                        target.any_of(rules);
                    }
                    else {
                        let rule = Rule::new(None);
                        rule.any_of(rules);
                        target.between(*min, *max, &rule);
                    }
                },
                Clause::Eof => {
                    target.eof();
                },
                Clause::Id { ref name, min, max } => {
                    match all_rules.get(name) {
                        Some(ref patx) => {
                            if *min == 1 && *max == 1 {
                                target.one(&patx.rule);
                            }
                            else {
                                target.between(*min, *max, &patx.rule);
                            }
                        }
                        None => {
                            return Err(format!("Rule \"{}\" not found.", name));
                        }
                    }
                },
                Clause::Literal { not /* TODO */, ref text, min, max } => {
                    if *min == 1 && *max == 1 {
                        if *not {
                            let rule = Rule::new(None);
                            rule.literal_string(text.clone());    
                            target.not(&rule);
                        }
                        else {
                            target.literal_string(text.clone());
                        }
                    }
                    else {
                        if *not {
                            let rule = Rule::new(None);
                            rule.literal_string(text.clone());

                            let between = Rule::new(None);
                            between.between(*min, *max, &rule);

                            target.not(&between);
                        }
                        else {
                            let rule = Rule::new(None);
                            rule.literal_string(text.clone());
                            target.between(*min, *max, &rule);
                        }
                    }
                },
                Clause::Whitespace { min, max } => {
                    target.between(*min, *max, &ws);
                },
            }
            
            if !is_one {
                self.rule.one(&target);
            }
        }

        Ok(())
    }
}