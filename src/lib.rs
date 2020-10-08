// Copyright (c) 2015-2020 Vincent van Ingen <code@abitvin.net>
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>
// This file may not be copied, modified, or distributed except according to those terms.

// TODO This is good: `!monkey+`, but this is weird: `"!monkey*"`.
// TODO There is a bug when using ranges in a not (!).

extern crate rule;

mod ast;
mod rules;

use ast::{Clause, ParseData};
use rule::{BranchFn, Rule, RuleError};
use rules::root;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;

struct GrammarRule<T> { 
    rule: Rule<T>,
    sentence: Vec<Clause>,
}

type CompiledGrammarRules<T> = HashMap<String, Rule<T>>;
type GrammarRules<T> = HashMap<String, GrammarRule<T>>;

pub struct CompiledGrammar<T> {
    rules: CompiledGrammarRules<T>,
}

pub struct Grammar<T> {
    rules: GrammarRules<T>,
    parser: Rule<ParseData>,
    ws: GrammarRule<T>,
}

#[derive(Debug)]
pub struct GrammarError {
    msg: String,
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
            rule: Rule::default(),
            sentence: parse(&parser, &ws_expr).unwrap(),
        };

        Self {
            rules: HashMap::new(),
            ws,
            parser,
        }
    }

    pub fn compile(self) -> Result<CompiledGrammar<T>, GrammarError> {
        let dummy = Rule::default();
        self.ws.code_gen(&self.rules, &dummy)?;
        
        for (_, r) in &self.rules {
            r.code_gen(&self.rules, &self.ws.rule)?;
        }
        
        let mut rules = HashMap::new();

        for (k, r) in self.rules {
            rules.insert(k, r.rule);
        }

        Ok(CompiledGrammar { rules })
    }

    pub fn map(&mut self, id: &str, expr: &str, branch_fn: BranchFn<T>) {
        self.add(id, expr, Some(branch_fn));
    }
    
    pub fn rule(&mut self, id: &str, expr: &str) {
        self.add(id, expr, None);
    }

    fn add(&mut self, id: &str, expr: &str, branch_fn: Option<BranchFn<T>>) {
        match parse(&self.parser, expr) {
            Ok(sentence) => {
                let rule = branch_fn
                    .map(|f| Rule::new(f))
                    .unwrap_or(Rule::default());
                
                let gram_rule = GrammarRule {
                    rule, sentence,
                };

                if self.rules.insert(String::from(id), gram_rule).is_some() {
                    panic!("The rule \"{}\" already used.", id);
                }
            },
            Err(err) => {
                panic!("Error parsing rule \"{}\": {:?}", id, err)
            },
        }
    }
}

impl<T> CompiledGrammar<T> {
    pub fn scan(&self, root_id: &str, code: &str) -> Result<Vec<T>, GrammarError> {
        if let Some(root) = &self.rules.get(root_id) {
            root.scan(code)
                .map_err(|e| GrammarError::from(e))
        }
        else {
            return Err(GrammarError::from(format!("Rule \"{}\" not found.", root_id)));
        }   
    }
}

fn parse(parser: &Rule<ParseData>, expr: &str) -> Result<Vec<Clause>, RuleError> {
    parser.scan(expr)
        .map(|parse_data| parse_data.into_iter().map(|x| x.unwrap_clause()).collect())
}

impl<T> GrammarRule<T> {
    fn code_gen(&self, all_rules: &GrammarRules<T>, ws: &Rule<T>) -> Result<(), GrammarError> {
        let is_one = self.sentence.len() == 1;
        
        for clause in &self.sentence {
            let target = if is_one {
                self.rule.clone()
            }
            else {
                Rule::default()
            };

            match clause {
                Clause::AlterTexts { ref replacements, min, max } => {
                    let replacements = replacements.iter()
                        .map(|x| (x.find.clone(), x.replace.clone()))
                        .collect();

                    add_extra(&target, false, *min, *max, |r: &Rule<T>| r.alter_string(replacements));
                }
                Clause::AnyChar { not, min, max } => {
                    add_extra(&target, *not, *min, *max, |r: &Rule<T>| r.any_char());
                },
                Clause::AnyCharExcept { not, ref chars, min, max } => {
                    add_extra(&target, *not, *min, *max, |r: &Rule<T>| r.any_char_except(chars.clone()));
                },
                Clause::AnyOf { not, ref sentences, min, max } => {
                    let mut rules = vec![];

                    for sentence in sentences {
                        let gram_rule = GrammarRule {
                            rule: Rule::default(),
                            sentence: sentence.clone(), // TODO Can we remove the clone?
                        };

                        gram_rule.code_gen(all_rules, ws)?;
                        rules.push(gram_rule.rule);
                    }

                    let rules = rules.iter()
                        .map(|x| x)
                        .collect();
                    
                    add_extra(&target, *not, *min, *max, |r: &Rule<T>| r.any_of(rules));
                },
                Clause::CharRanges { not, ref ranges, min, max } => {
                    let rules: Vec<Rule<_>> = ranges.iter()
                        .map(|r| {
                            let rule = Rule::default();
                            rule.char_in(r.start, r.end);
                            rule
                        })
                        .collect();
                    
                    let rules = rules.iter()
                        .map(|x| x)
                        .collect();

                    add_extra(&target, *not, *min, *max, |r: &Rule<T>| r.any_of(rules));
                },
                Clause::Eof => {
                    target.eof();
                },
                Clause::Id { not, ref name, min, max } => {
                    let rule = match all_rules.get(name) {
                        Some(ref r) => &r.rule,
                        None => return Err(GrammarError::from(format!("Rule \"{}\" not found.", name)))
                    };

                    if *min == 1 && *max == 1 {
                        if *not {
                            target.not(&rule);
                        }
                        else {
                            target.one(&rule);
                        }
                    }
                    else {
                        if *not {
                            let quantity = Rule::default();
                            quantity.between(*min, *max, &rule);
                            target.not(&quantity);
                        }
                        else {
                            target.between(*min, *max, &rule);
                        }
                    }
                },
                Clause::Literal { not, ref text, min, max } => {
                    add_extra(&target, *not, *min, *max, |r: &Rule<T>| r.literal_string(text.clone()));
                },
                Clause::NoBacktrack(ref err_msg) => {
                    target.no_backtrack(err_msg.clone());
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

fn add_extra<T, F>(target: &Rule<T>, not: bool, min: u64, max: u64, f: F) 
where F: FnOnce(&Rule<T>) -> &Rule<T> {
    if min == 1 && max == 1 {
        if not {
            let rule = Rule::default();
            target.not(f(&rule));
        }
        else {
            f(target);
        }
    }
    else {
        if not {
            let rule = Rule::default();
            let quantity = Rule::default();
            quantity.between(min, max, f(&rule));
            target.not(&quantity);
        }
        else {
            let rule = Rule::default();
            target.between(min, max, f(&rule));
        }
    }
}

impl fmt::Display for GrammarError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl Error for GrammarError {
    fn description(&self) -> &str {
        "Grammer error"
    }
}

impl From<String> for GrammarError {
    fn from(err_msg: String) -> Self {
        GrammarError {
            msg: err_msg,
        }
    }
}

impl From<RuleError> for GrammarError {
    fn from(err: RuleError) -> Self {
        GrammarError { 
            msg: format!("{}", err),
        }
    }
}