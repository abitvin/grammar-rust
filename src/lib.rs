// TODO Remove any printf.
// TODO Remove Range::Not
// TODO Optimize Range::None, this can be Range::Quantity { min: 1, max: 1 }
// TODO This is good: `!monkey*`, but this is weird: `"!monkey+"`.
// TODO Refactor.
// TODO There is a bug when using ranges in a not (!).
// TODO Implement other "not"'s in the Pattern enum.
// TODO Update Cargo.toml

// Copyright (c) 2015-2018 Vincent van Ingen <code@abitvin.net>
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>
// This file may not be copied, modified, or distributed except according to those terms.

extern crate rule;

mod ast;
mod rules;

use ast::{ParseData, Pattern, Range};
use rule::{BranchFn, Rule, RuleError};
use rules::root;
use std::collections::HashMap;

// TODO This name!
struct PatternX<T> {
    rule: Rule<T>,
    pattern: Vec<Pattern>,
}

type Patterns<T> = HashMap<String, PatternX<T>>;

pub struct Grammer<T> {
    compiled: bool,
    patterns: Patterns<T>,
    parser: Rule<ParseData>,
    ws: PatternX<T>,
}

impl<T> Grammer<T> {
    pub fn new() -> Self {
        Self::new_("(\\ |\t|\n|\r)")
    }

    pub fn new_with_ws(pattern: &str) -> Self {
        Self::new_(pattern)
    }

    fn new_(ws_pattern: &str) -> Self {
        let parser = root();
        
        let ws = PatternX {
            rule: Rule::new(None),
            pattern: parse(&parser, &ws_pattern).unwrap(),
        };

        Self {
            compiled: false,
            patterns: HashMap::new(),
            ws,
            parser,
        }
    }

    pub fn add(&mut self, id: &str, expr: &str, branch_fn: BranchFn<T>) {
        if self.compiled {
            // TODO Improve, this panic is lazy. We can parse/compile anytime (because we cannot remove rules). 
            panic!("Cannot alter grammer when being used.");
        }

        match parse(&self.parser, expr) {
            Ok(pattern) => {
                let rule = Rule::new(branch_fn);

                if self.patterns.insert(String::from(id), PatternX{rule, pattern}).is_some() {
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
            self.ws.code_gen(&self.patterns, &dummy)?;

            for (_, patternx) in &self.patterns {
                patternx.code_gen(&self.patterns, &self.ws.rule)?;
            }

            self.compiled = true;
        }
        
        if let Some(root) = &self.patterns.get(root_id) {
            root.rule.scan(code)
                .map_err(|_| String::from("TODO ERROR"))        // TODO Error messages.
        }
        else {
            return Err(format!("Rule \"{}\" not found.", root_id));
        }   
    }
}

fn parse(parser: &Rule<ParseData>, expr: &str) -> Result<Vec<Pattern>, Vec<RuleError>> {
    parser.scan(expr)
        .map(|parse_data| parse_data.into_iter().map(|x| x.unwrap_pattern()).collect())
}

impl<T> PatternX<T> {
    fn code_gen(&self, all_patterns: &Patterns<T>, ws: &Rule<T>) -> Result<(), String> {
        let is_one = self.pattern.len() == 1;
        
        for p in &self.pattern {
            let target = if is_one {
                self.rule.clone()
            }
            else {
                Rule::new(None)
            };

            match p {
                Pattern::AlterChars { ref replacements, ref range } => {
                    let replacements = replacements.iter()
                        .map(|x| (x.find.clone(), x.replace.clone()))
                        .collect();

                    // TODO Refactor all these common ranges.
                    match range {
                        Range::None => {
                            target.alter_string(replacements);
                        },
                        Range::Quantity { min, max } => {
                            let rule = Rule::new(None);
                            rule.alter_string(replacements);
                            target.between(*min, *max, &rule);
                        },
                    }
                }
                Pattern::AnyChar(ref range) => {
                    match range {
                        Range::None => {
                            target.any_char();
                        },
                        Range::Quantity { min, max } => {
                            let rule = Rule::new(None);
                            rule.any_char();
                            target.between(*min, *max, &rule);
                        },
                    }
                },
                Pattern::AnyCharExcept { ref chars, ref range } => {
                    match range {
                        Range::None => {
                            target.any_char_except(chars.clone());
                        },
                        Range::Quantity { min, max } => {
                            let rule = Rule::new(None);
                            rule.any_char_except(chars.clone());
                            target.between(*min, *max, &rule);
                        },
                    }
                },
                Pattern::AnyOf { ref patterns, ref range } => {
                    println!("ANYOF");
                    let mut rules = vec![];

                    // TODO These names are ugly!
                    for pp in patterns {
                        let rrr = Rule::new(None);

                        for ppp in pp {
                            println!("PP: {:?}", pp);

                            let x = PatternX {
                                rule: Rule::new(None),
                                pattern: vec![ppp.clone()],     // TODO Improve.
                            };

                            x.code_gen(all_patterns, ws)?;
                            rrr.one(&x.rule);
                        }

                        rules.push(rrr);
                    }

                    let rules = rules.iter()
                        .map(|x| x)
                        .collect();
                    
                    match range {
                        Range::None => {
                            target.any_of(rules);
                        },
                        Range::Quantity { min, max } => {
                            let rule = Rule::new(None);
                            rule.any_of(rules);
                            target.between(*min, *max, &rule);
                        },
                    }
                },
                Pattern::CharRanges { ref ranges, ref range } => {
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
                    
                    match range {
                        Range::None => {
                            target.any_of(rules);
                        },
                        Range::Quantity { min, max } => {
                            let rule = Rule::new(None);
                            rule.any_of(rules);
                            target.between(*min, *max, &rule);
                        },
                    }
                },
                Pattern::Eof => {
                    target.eof();
                },
                Pattern::Id { ref name, ref range } => {
                    match all_patterns.get(name) {
                        Some(ref patx) => {
                            match range {
                                Range::None => target.one(&patx.rule),
                                Range::Quantity { min, max } => target.between(*min, *max, &patx.rule),
                            };
                        }
                        None => {
                            return Err(format!("Rule \"{}\" not found.", name));
                        }
                    }
                },
                Pattern::Literal { not /* TODO */, ref text, ref range } => {
                    match range {
                        Range::None => {
                            if *not {
                                let rule = Rule::new(None);
                                rule.literal_string(text.clone());    
                                target.not(&rule);
                            }
                            else {
                                target.literal_string(text.clone());
                            }
                        },
                        Range::Quantity { min, max } => {
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
                        },
                    }
                },
                Pattern::Whitespace(ref range) => {
                    match range {
                        Range::None => unreachable!(),
                        Range::Quantity { min, max } => target.between(*min, *max, &ws),
                    };
                },
            }
            
            if !is_one {
                self.rule.one(&target);
            }
        }

        Ok(())
    }
}