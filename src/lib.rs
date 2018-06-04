// Copyright (c) 2015-2018 Vincent van Ingen <code@abitvin.net>
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>
// This file may not be copied, modified, or distributed except according to those terms.

// TODO Change repository names.
// TODO This is good: `!monkey+`, but this is weird: `"!monkey*"`.
// TODO Refactor.
// TODO There is a bug when using ranges in a not (!).
// TODO Update Cargo.toml, use online crate of Rule.
// TODO Remove most panics.
// TODO Analyze and optimize the AST.

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

type GrammarRules<T> = HashMap<String, GrammarRule<T>>;

pub struct Grammar<T> {
    compiled: bool,
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

    pub fn scan(&mut self, root_id: &str, code: &str) -> Result<Vec<T>, GrammarError> 
     {
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
                .map_err(|e| GrammarError::from(e))
        }
        else {
            return Err(GrammarError::from(format!("Rule \"{}\" not found.", root_id)));
        }   
    }
}

fn parse(parser: &Rule<ParseData>, expr: &str) -> Result<Vec<Clause>, Vec<RuleError>> {
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
                Rule::new(None)
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
                    
                    add_extra(&target, *not, *min, *max, |r: &Rule<T>| r.any_of(rules));
                },
                Clause::CharRanges { not, ref ranges, min, max } => {
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
                            let quantity = Rule::new(None);
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
    where F: FnOnce(&Rule<T>) -> &Rule<T>
{
    if min == 1 && max == 1 {
        if not {
            let rule = Rule::new(None);
            target.not(f(&rule));
        }
        else {
            f(target);
        }
    }
    else {
        if not {
            let rule = Rule::new(None);
            let quantity = Rule::new(None);
            quantity.between(min, max, f(&rule));
            target.not(&quantity);
        }
        else {
            let rule = Rule::new(None);
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
        &self.msg
    }
}

impl From<String> for GrammarError {
    fn from(err_msg: String) -> Self {
        GrammarError {
            msg: err_msg,
        }
    }
}

impl From<Vec<RuleError>> for GrammarError {
    fn from(err: Vec<RuleError>) -> Self {
        let msg = err
            .into_iter()
            .fold(String::from("Parse error\n"), |msg, err| format!("{}- index {}: {}\n", msg, err.index, err.msg));
        
        GrammarError { msg }
    }
}