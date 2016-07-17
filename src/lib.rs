// Copyright (c) 2015-2016 Abitvin <foss@abitvin.net>
// Licensed under the MIT license <LICENSE.md or http://opensource.org/licenses/MIT>
// This file may not be copied, modified, or distributed except according to those terms.

// TODO Put Cargo.lock in the .gitignore and remove it from git. Also do this for the Rule API.

extern crate rule;

use rule::BranchFn;
use rule::Rule;
use rule::RuleError;
use std::collections::BTreeMap;

enum RangeType {
    NoRangeType = 0,
    AtLeast,
    AtMost,
    Between,
    Exact,
    Not
}

// TODO Remove NoShared and add shared state to the grammer API. 
pub struct NoShared {} 

struct ParseContext<T> {
    arg1: u64,
    arg2: u64, 
    arg3: Option<String>,
    arg4: Option<(String, String)>,
    range_type: RangeType,
    rule: Option<Rule<T, NoShared>>,
}

// TODO Use HashMap. It's faster and maybe we can revert back from `*mut Rule` into `Rule` in the RuleExp struct.
// https://doc.rust-lang.org/std/collections/
type RuleExprMap<T> = BTreeMap<&'static str, RuleExpr<T>>;

struct GrammerShared<T> {
    rule_exps: *const RuleExprMap<T>,
    keep_ws: *const Rule<T, NoShared>,
}

// TODO TMeta class R<TB, TM> extends Rule<IParseContext<TB, TM>, IEmpty> {}
type R<T> = Rule<ParseContext<T>, GrammerShared<T>>;

// TODO Note: This was IRule, remove this comment later after porting.
struct RuleExpr<T> {
    //TODO Maybe remove? id: &'static str,
    is_defined: bool,
    rule: *mut Rule<T, NoShared>,   
}

impl<T> Drop for RuleExpr<T> 
{
    fn drop(&mut self) 
    {
        unsafe {
            let rule = Box::from_raw(self.rule);
        } 
    }
}

pub struct Grammer<T> /* <TBranch, TMeta> */
{
    grammer: R<T>,
    rule_exps: RuleExprMap<T>,
    
    keep_alter_tuple: Box<R<T>>,        // We need to keep rules defined in the `new` function alive.
    keep_integer: Box<R<T>>,            // ..
    keep_ranges: Box<R<T>>,             // ..
    keep_statement: Box<R<T>>,          // ..
    keep_ws: Rule<T, NoShared>,         // ..
}

impl<T> Grammer<T>
{
    pub fn new() -> Self
    {
        let rule_exps = RuleExprMap::new();

        let mut space = Rule::new(None); space.literal(" ");
        let mut tab = Rule::new(None); tab.literal("\t");
        let mut new_line = Rule::new(None); new_line.literal("\n");
        let mut carriage_return = Rule::new(None); carriage_return.literal("\r");
        let mut ws = Rule::new(None);
        ws.any_of(vec![space, tab, new_line, carriage_return]);

        let statement_fn = |mut b: Vec<ParseContext<T>>, _: &str, _: &mut GrammerShared<T>|
        {
            match b[0].range_type {
                RangeType::Not => {
                    let mut r: Rule<T, NoShared> = Rule::new(None);
                    r.not(b.pop().unwrap().rule.unwrap());    // TODO Test this, originally it was b[1]
                                                                    // Does this goes well together with ranges? 

                    vec![ParseContext{ 
                        arg1: 0,
                        arg2: 0,
                        arg3: None,
                        arg4: None,
                        range_type: RangeType::NoRangeType,
                        rule: Some(r), 
                    }]
                },
                _ => {
                    b
                }
            }
        };

        let boxed_ranges = Box::new(R::new(None));  // Allocate memory for the "ranges" rule.
        let ranges = Box::into_raw(boxed_ranges);   // Transform it into a raw pointer to be used by other rules.

        let boxed_statement = Box::new(R::new(Some(Box::new(statement_fn))));
        let statement = Box::into_raw(boxed_statement);
        
        let mut escaped_ctrl_chars: R<T> = R::new(None);
        escaped_ctrl_chars.alter(vec![
            ("\\<", "<"), 
            ("\\>", ">"), 
            ("\\{", "{"),
            ("\\}", "}"), 
            ("\\(", "("), 
            ("\\)", ")"), 
            ("\\[", "["), 
            ("\\]", "]"), 
            ("\\^", "^"),
            ("\\~", "~"),
            ("\\-", "-"),
            ("\\,", ","),
            ("\\|", "|"),
            ("\\+", "+"), 
            ("\\?", "?"), 
            ("\\*", "*"), 
            ("\\.", "."), 
            ("\\$", "$"),
            ("\\ ", " "), 
            ("\\_", "_"),
            ("\\!", "!"),
        ]);

        // Integer
        let integer_fn = |_: Vec<ParseContext<T>>, l: &str, _: &mut GrammerShared<T>|
        {
            vec![ParseContext { 
                arg1: l.parse::<u64>().unwrap(),
                arg2: 0,
                arg3: None,
                arg4: None,
                range_type: RangeType::NoRangeType,
                rule: None, 
            }]
        };

        let mut digit = R::new(None); digit.char_in('0', '9');
        let boxed_integer = Box::new(R::new(Some(Box::new(integer_fn))));
        let integer = Box::into_raw(boxed_integer); 
        unsafe { (*integer).at_least(1, digit); } 
        
        // Literal
        let literal_text_fn = |_: Vec<ParseContext<T>>, l: &str, _: &mut GrammerShared<T>|
        {
            vec![ParseContext { 
                arg1: 0,
                arg2: 0,
                arg3: Some(String::from(l)),
                arg4: None,
                range_type: RangeType::NoRangeType,
                rule: None, 
            }]
        };

        let mut literal_all_except: R<T> = R::new(None);
        literal_all_except.all_except(vec!['<', '{', '(', ')', '|', '[', '+', '?', '*', '.', '$', ' ', '_', '!']);

        let mut literal_char: R<T> = R::new(None);
        unsafe { literal_char.any_of(vec![escaped_ctrl_chars.shallow_clone(None), literal_all_except]); }
        
        let mut literal_text: R<T> = R::new(Some(Box::new(literal_text_fn)));
        literal_text.at_least(1, literal_char);

        let literal_fn = |b: Vec<ParseContext<T>>, _: &str, _: &mut GrammerShared<T>|
        {
            let mut rule = Rule::new(None);
            rule.literal_string(b[0].arg3.clone().unwrap());
            
            if b.len() == 2 {
                rule = Grammer::add_range(rule, &b[1]);
            }
            
            vec![ParseContext { 
                arg1: 0,
                arg2: 0,
                arg3: None,
                arg4: None,
                range_type: RangeType::NoRangeType,
                rule: Some(rule),
            }]
        };

        let mut literal: R<T> = R::new(Some(Box::new(literal_fn)));
        unsafe { literal.one(literal_text).maybe_raw(ranges); }
        
        // Any char
        let any_char_fn = |b: Vec<ParseContext<T>>, _: &str, _: &mut GrammerShared<T>|
        {
            let mut rule = Rule::new(None);
            rule.all();

            if b.len() == 1 {
                rule = Grammer::add_range(rule, &b[0]);
            }

            vec![ParseContext { 
                arg1: 0,
                arg2: 0,
                arg3: None,
                arg4: None,
                range_type: RangeType::NoRangeType,
                rule: Some(rule),
            }]
        };

        let mut any_char = R::new(Some(Box::new(any_char_fn)));
        unsafe { any_char.literal(".").maybe_raw(ranges); }

        // Any char except
        let any_char_except_chars_fn = |_: Vec<ParseContext<T>>, l: &str, _: &mut GrammerShared<T>|
        {
            vec![ParseContext { 
                arg1: 0,
                arg2: 0,
                arg3: Some(String::from(l)),
                arg4: None,
                range_type: RangeType::NoRangeType,
                rule: None,
            }]
        };

        let any_char_except_fn = |mut b: Vec<ParseContext<T>>, _: &str, _: &mut GrammerShared<T>|
        {
            let mut rule = Rule::new(None);

            if b.len() == 1 {
                rule.all_except(b.pop().unwrap().arg3.unwrap().chars().map(|c| c).collect());
            } 
            else {
                let last = b.pop().unwrap();
                rule.all_except(b.pop().unwrap().arg3.unwrap().chars().map(|c| c).collect());
                rule = Grammer::add_range(rule, &last);
            }

            vec![ParseContext { 
                arg1: 0,
                arg2: 0,
                arg3: None,
                arg4: None,
                range_type: RangeType::NoRangeType,
                rule: Some(rule),
            }]
        };
        
        let mut any_char_except_any_other = R::new(None); 
        any_char_except_any_other.all_except(vec![']']);

        let mut any_char_except_char = R::new(None); 
        unsafe { any_char_except_char.any_of(vec![escaped_ctrl_chars.shallow_clone(None), any_char_except_any_other]); }

        let mut any_char_except_chars = R::new(Some(Box::new(any_char_except_chars_fn))); 
        any_char_except_chars.at_least(1, any_char_except_char);

        let mut any_char_except = R::new(Some(Box::new(any_char_except_fn))); 
        unsafe { any_char_except.literal("[^").one(any_char_except_chars).literal("]").maybe_raw(ranges); }
        
        // Match character range
        let char_range_fn = |_: Vec<ParseContext<T>>, l: &str, _: &mut GrammerShared<T>|
        {
            let lower = l.chars().next().unwrap();
            let upper = l.chars().skip(2).next().unwrap();

            vec![ParseContext { 
                arg1: 0,
                arg2: 0,
                arg3: Some(char::to_string(&lower)),
                arg4: None,
                range_type: RangeType::NoRangeType,
                rule: None,
            },
            ParseContext { 
                arg1: 0,
                arg2: 0,
                arg3: Some(char::to_string(&upper)),
                arg4: None,
                range_type: RangeType::NoRangeType,
                rule: None,
            }]
        };

        let char_ranges_fn = |mut b: Vec<ParseContext<T>>, _: &str, _: &mut GrammerShared<T>|
        {
            let mut rule = Rule::new(None);
            
            if b.len() > 2 {
                let mut ranges = Vec::new();
                
                while b.len() > 1 {
                    let rest = b.split_off(2);
                    
                    let upper = b.pop().unwrap().arg3.unwrap().chars().next().unwrap();
                    let lower = b.pop().unwrap().arg3.unwrap().chars().next().unwrap();

                    let mut r = Rule::new(None);
                    r.char_in(lower, upper);

                    ranges.push(r);

                    b = rest; 
                }

                rule.any_of(ranges);
            }
            else {
                let upper = b.pop().unwrap().arg3.unwrap().chars().next().unwrap();
                let lower = b.pop().unwrap().arg3.unwrap().chars().next().unwrap();
                rule.char_in(lower, upper);
            }

            if let Some(ctx) = b.pop() {
                rule = Grammer::add_range(rule, &ctx);
            }

            vec![ParseContext { 
                arg1: 0,
                arg2: 0,
                arg3: None,
                arg4: None,
                range_type: RangeType::NoRangeType,
                rule: Some(rule),
            }]
        };

        let mut char_range_all_except = R::new(None);
        char_range_all_except.all_except(vec!['-', ']']);

        let mut char_range_char = R::new(None);
        unsafe { char_range_char.any_of(vec![escaped_ctrl_chars.shallow_clone(None), char_range_all_except]); }

        let mut char_range = R::new(Some(Box::new(char_range_fn)));
        unsafe { char_range.one(char_range_char.shallow_clone(None)).literal("-").one(char_range_char); }

        let mut char_ranges = R::new(Some(Box::new(char_ranges_fn)));
        unsafe { char_ranges.literal("[").at_least(1, char_range).literal("]").maybe_raw(ranges); }

        // EOF
        let eof_fn = |_: Vec<ParseContext<T>>, _: &str, _: &mut GrammerShared<T>|
        {
            let mut rule = Rule::new(None);
            rule.eof();

            vec![ParseContext { 
                arg1: 0,
                arg2: 0,
                arg3: None,
                arg4: None,
                range_type: RangeType::NoRangeType,
                rule: Some(rule),
            }]
        };

        let mut eof = R::new(Some(Box::new(eof_fn)));
        eof.literal("$");
        
        // One rule
        let rule_name_fn = |_: Vec<ParseContext<T>>, l: &str, _: &mut GrammerShared<T>|
        {
            vec![ParseContext { 
                arg1: 0,
                arg2: 0,
                arg3: Some(String::from(l)),
                arg4: None,
                range_type: RangeType::NoRangeType,
                rule: None,
            }]
        };

        let mut rule_name_all_except = R::new(None);
        rule_name_all_except.all_except(vec!['>']);

        let mut rule_name_char = R::new(None);
        unsafe { rule_name_char.any_of(vec![escaped_ctrl_chars.shallow_clone(None), rule_name_all_except]); }

        let mut rule_name = R::new(Some(Box::new(rule_name_fn)));
        rule_name.at_least(1, rule_name_char);

        let rule_fn = |mut b: Vec<ParseContext<T>>, _: &str, s: &mut GrammerShared<T>|
        {
            if b.len() == 1 {
                // TODO This note is not only meant for the code below, but could we just use `ref` instead of popping?
                let id = b.pop().unwrap().arg3.unwrap();

                match unsafe { (*s.rule_exps).get(id.as_str()) } {
                    None => {
                        panic!("Rule \"{}\" not found", id)
                    },
                    Some(r) => {
                        let mut rule = Rule::new(None);
                        unsafe { rule.one_raw(r.rule) };

                        vec![ParseContext { 
                            arg1: 0,
                            arg2: 0,
                            arg3: None,
                            arg4: None,
                            range_type: RangeType::NoRangeType,
                            rule: Some(rule),
                        }]
                    },
                }
            }
            else {
                let range = b.pop().unwrap();
                let id = b.pop().unwrap().arg3.unwrap();

                match unsafe { (*s.rule_exps).get(id.as_str()) } {
                    None => {
                        panic!("Rule \"{}\" not found", id)
                    },
                    Some(r) => {
                        vec![ParseContext { 
                            arg1: 0,
                            arg2: 0,
                            arg3: None,
                            arg4: None,
                            range_type: RangeType::NoRangeType,
                            rule: Some(Grammer::add_range_raw(r.rule, &range)),
                        }]
                    },
                }
            }
        };

        let mut rule = R::new(Some(Box::new(rule_fn)));
        unsafe { rule.literal("<").one(rule_name).literal(">").maybe_raw(ranges) };
        
        // At least
        let at_least_fn = |b: Vec<ParseContext<T>>, _: &str, _: &mut GrammerShared<T>|
        {
            vec![ParseContext { 
                arg1: b[0].arg1,
                arg2: 0,
                arg3: None,
                arg4: None,
                range_type: RangeType::AtLeast,
                rule: None,
            }]
        };

        let mut at_least = R::new(Some(Box::new(at_least_fn))); 
        unsafe { at_least.literal("{").one_raw(integer).literal(",}"); }

        // At least one
        let at_least_one_fn = |_: Vec<ParseContext<T>>, _: &str, _: &mut GrammerShared<T>|
        {
            vec![ParseContext { 
                arg1: 1,
                arg2: 0,
                arg3: None,
                arg4: None,
                range_type: RangeType::AtLeast,
                rule: None,
            }]
        };

        let mut at_least_one = R::new(Some(Box::new(at_least_one_fn))); 
        at_least_one.literal("+");

        // At most
        let at_most_fn = |b: Vec<ParseContext<T>>, _: &str, _: &mut GrammerShared<T>|
        {
            vec![ParseContext { 
                arg1: b[0].arg1,
                arg2: 0,
                arg3: None,
                arg4: None,
                range_type: RangeType::AtMost,
                rule: None,
            }]
        };

        let mut at_most = R::new(Some(Box::new(at_most_fn))); 
        unsafe { at_most.literal("{,").one_raw(integer).literal("}"); }

        // Between
        let between_fn = |b: Vec<ParseContext<T>>, _: &str, _: &mut GrammerShared<T>|
        {
            vec![ParseContext { 
                arg1: b[0].arg1,
                arg2: b[1].arg1,
                arg3: None,
                arg4: None,
                range_type: RangeType::Between,
                rule: None,
            }]
        };

        let mut between = R::new(Some(Box::new(between_fn))); 
        unsafe { between.literal("{").one_raw(integer).literal(",").one_raw(integer).literal("}"); }

        // Exact
        let exact_fn = |b: Vec<ParseContext<T>>, _: &str, _: &mut GrammerShared<T>|
        {
            vec![ParseContext { 
                arg1: b[0].arg1,
                arg2: 0,
                arg3: None,
                arg4: None,
                range_type: RangeType::Exact,
                rule: None,
            }]
        };

        let mut exact = R::new(Some(Box::new(exact_fn))); 
        unsafe { exact.literal("{").one_raw(integer).literal("}"); }

        // Maybe
        let maybe_fn = |_: Vec<ParseContext<T>>, _: &str, _: &mut GrammerShared<T>|
        {
            vec![ParseContext { 
                arg1: 0,
                arg2: 1,
                arg3: None,
                arg4: None,
                range_type: RangeType::Between,
                rule: None,
            }]
        };

        let mut maybe = Rule::new(Some(Box::new(maybe_fn)));
        maybe.literal("?");

        // None or many
        let none_or_many_fn = |_: Vec<ParseContext<T>>, _: &str, _: &mut GrammerShared<T>|
        {
            vec![ParseContext { 
                arg1: 0,
                arg2: 0,
                arg3: None,
                arg4: None,
                range_type: RangeType::AtLeast,
                rule: None,
            }]
        };

        let mut none_or_many = Rule::new(Some(Box::new(none_or_many_fn)));
        none_or_many.literal("*");
        
        // Not
        let not_fn = |_: Vec<ParseContext<T>>, _: &str, _: &mut GrammerShared<T>|
        {
            vec![ParseContext { 
                arg1: 0,
                arg2: 0,
                arg3: None,
                arg4: None,
                range_type: RangeType::Not,
                rule: None,
            }]
        };

        let mut not = Rule::new(Some(Box::new(not_fn)));
        not.literal("!");

        // Any of
        let any_of_fn = |mut b: Vec<ParseContext<T>>, _: &str, _: &mut GrammerShared<T>|
        {
            let last = b.pop().unwrap();
            let mut rule = Rule::new(None);
            let mut rules: Vec<Rule<T, NoShared>> = b.into_iter().map(|c| c.rule.unwrap()).collect();

            match last.range_type {
                RangeType::NoRangeType => {
                    rules.push(last.rule.unwrap());
                    rule.any_of(rules);
                },
                _ => {
                    rule.any_of(rules);
                    rule = Grammer::add_range(rule, &last);
                },
            }

            vec![ParseContext { 
                arg1: 0,
                arg2: 0,
                arg3: None,
                arg4: None,
                range_type: RangeType::NoRangeType,
                rule: Some(rule),
            }]
        };

        let statements_fn = |b: Vec<ParseContext<T>>, _: &str, _: &mut GrammerShared<T>|
        {
            if b.len() == 1 {
                b
            }
            else {
                let mut rule = Rule::new(None);

                for ctx in b {
                    rule.one(ctx.rule.unwrap());
                }

                vec![ParseContext { 
                    arg1: 0,
                    arg2: 0,
                    arg3: None,
                    arg4: None,
                    range_type: RangeType::NoRangeType,
                    rule: Some(rule),
                }]
            }                
        };

        let mut statements = R::new(Some(Box::new(statements_fn)));
        unsafe { statements.at_least_raw(1, statement); }

        let mut more = R::new(None);
        unsafe { more.literal("|").one(statements.shallow_clone(None)); }

        let mut any_of = R::new(Some(Box::new(any_of_fn)));
        unsafe { any_of.literal("(").one(statements).none_or_many(more).literal(")").maybe_raw(ranges); }

        // Alter
        let alter_fn = |mut b: Vec<ParseContext<T>>, _: &str, _: &mut GrammerShared<T>|
        {
            // TODO Can we use `ref` instead of pop and pushing?
            let last = b.pop().unwrap();

            match last.range_type {
                RangeType::NoRangeType => {
                    b.push(last);

                    let mut rule = Rule::new(None);
                    rule.alter_string(b.into_iter().map(|i| i.arg4.unwrap()).collect());
                    
                    vec![ParseContext { 
                        arg1: 0,
                        arg2: 0,
                        arg3: None,
                        arg4: None,
                        range_type: RangeType::NoRangeType,
                        rule: Some(rule),
                    }]
                },
                _ => {
                    let mut rule = Rule::new(None);
                    rule.alter_string(b.into_iter().map(|i| i.arg4.unwrap()).collect());

                    vec![ParseContext { 
                        arg1: 0,
                        arg2: 0,
                        arg3: None,
                        arg4: None,
                        range_type: RangeType::NoRangeType,
                        rule: Some(Grammer::add_range(rule, &last)),
                    }]
                },
            }        
        };

        let alter_left_text_fn = |_: Vec<ParseContext<T>>, l: &str, _: &mut GrammerShared<T>|
        {
            vec![ParseContext { 
                arg1: 0,
                arg2: 0,
                arg3: Some(String::from(l)),
                arg4: None,
                range_type: RangeType::NoRangeType,
                rule: None,
            }]
        };

        let alter_right_text_fn = |_: Vec<ParseContext<T>>, l: &str, _: &mut GrammerShared<T>|
        {
            vec![ParseContext { 
                arg1: 0,
                arg2: 0,
                arg3: Some(String::from(l)),
                arg4: None,
                range_type: RangeType::NoRangeType,
                rule: None,
            }]
        };

        let alter_tuple_fn = |mut b: Vec<ParseContext<T>>, _: &str, _: &mut GrammerShared<T>|
        {
            let to = b.pop().unwrap().arg3.unwrap();
            let from = b.pop().unwrap().arg3.unwrap();

            vec![ParseContext { 
                arg1: 0,
                arg2: 0,
                arg3: None,
                arg4: Some((from, to)),
                range_type: RangeType::NoRangeType,
                rule: None,
            }]
        };

        let mut alter_all_except_left_char = R::new(None);
        alter_all_except_left_char.all_except(vec![',']);
        
        let mut alter_left_char = R::new(None);
        unsafe { alter_left_char.any_of(vec![escaped_ctrl_chars.shallow_clone(None), alter_all_except_left_char]); }

        let mut alter_left_text = R::new(Some(Box::new(alter_left_text_fn)));
        alter_left_text.at_least(1, alter_left_char);

        let mut alter_all_except_right_char = R::new(None);
        alter_all_except_right_char.all_except(vec!['|', ')']);
        
        let mut alter_right_char = R::new(None);
        alter_right_char.any_of(vec![escaped_ctrl_chars, alter_all_except_right_char]);

        let mut alter_right_text = R::new(Some(Box::new(alter_right_text_fn)));
        alter_right_text.at_least(1, alter_right_char);

        let mut alter_tuple = R::new(Some(Box::new(alter_tuple_fn)));
        alter_tuple.one(alter_left_text).literal(",").one(alter_right_text);
        let alter_tuple = Box::into_raw(Box::new(alter_tuple));

        let mut alter_more = R::new(None);
        unsafe { alter_more.literal("|").one_raw(alter_tuple); }

        let mut alter = R::new(Some(Box::new(alter_fn)));
        unsafe { alter.literal("(~").one_raw(alter_tuple).none_or_many(alter_more).literal(")").maybe_raw(ranges); }

        // Whitespace
        let at_least_one_ws_fn = |_: Vec<ParseContext<T>>, _: &str, s: &mut GrammerShared<T>|
        {
            let mut r = Rule::new(None);
            unsafe { r.at_least_raw(1, s.keep_ws); }

            vec![ParseContext { 
                arg1: 0,
                arg2: 0,
                arg3: None,
                arg4: None,
                range_type: RangeType::NoRangeType,
                rule: Some(r),
            }]
        };

        let none_or_many_ws_fn = |_: Vec<ParseContext<T>>, _: &str, s: &mut GrammerShared<T>|
        {
            let mut r = Rule::new(None);
            unsafe { r.none_or_many_raw(s.keep_ws); }

            vec![ParseContext { 
                arg1: 0,
                arg2: 0,
                arg3: None,
                arg4: None,
                range_type: RangeType::NoRangeType,
                rule: Some(r),
            }]
        };

        let mut at_least_one_ws = R::new(Some(Box::new(at_least_one_ws_fn)));
        at_least_one_ws.literal("_");

        let mut none_or_many_ws = R::new(Some(Box::new(none_or_many_ws_fn)));
        none_or_many_ws.literal(" ");

        // Ranges and statements definitions
        unsafe {
            (*ranges).any_of(vec![at_least, at_least_one, at_most, between, exact, maybe, none_or_many]);
            (*statement).maybe(not).any_of(vec![any_char, none_or_many_ws, at_least_one_ws, eof, alter, any_char_except, char_ranges, rule, any_of, literal]);
        }
        
        let mut grammer = R::new(None);
        unsafe { grammer.none_or_many_raw(statement); }
        
        Grammer {
            grammer: grammer, 
            rule_exps: rule_exps,

            keep_alter_tuple: unsafe { Box::from_raw(alter_tuple) },    // We need to keep these rules alive because they are used in other rules as raw references.
            keep_integer: unsafe { Box::from_raw(integer) },            // ..
            keep_ranges: unsafe { Box::from_raw(ranges) },              // ..
            keep_statement: unsafe { Box::from_raw(statement) },        // ..
            keep_ws: ws,                                                // ..
        }
    }
    
    pub fn add(&mut self, id: &'static str, expr: &'static str, branch_fn: BranchFn<T, NoShared>)
    {
        {
            let rulexp = self.rule_exps.get(id);

            if rulexp.is_some() && rulexp.unwrap().is_defined {
                panic!("The rule \"{}\" already used.", id);    // TODO Return nice error
            }
        }

        let mut shared = GrammerShared {
            rule_exps: &self.rule_exps,
            keep_ws: &self.keep_ws,
        };

        let result = self.grammer.scan(&expr, &mut shared); 

        match result {
            Err(_) => {
                panic!("Error compiling rule expression.");    // TODO Return nice error
            },
            Ok(mut branches) => {
                let new_ruleexp = match self.rule_exps.get_mut(id) {
                    None => {
                        let mut compiled = Rule::new(branch_fn);
                        
                        for r in branches {
                            compiled.one(r.rule.unwrap());
                        }

                        Some(RuleExpr {
                            // TODO Maybe remove? id: id,
                            is_defined: true,
                            rule: Box::into_raw(Box::new(compiled)),
                        })
                    },
                    Some(rulexp) => {
                        rulexp.is_defined = true;
                        // TODO rulexp.rule.meta = meta;

                        unsafe {
                            (*rulexp.rule).branch_fn = branch_fn;

                            for r in branches {
                                (*rulexp.rule).one(r.rule.unwrap());
                            }
                        }

                        None
                    },
                };

                if let Some(r) = new_ruleexp {
                    self.rule_exps.insert(id, r);
                }
            },
        }
    }

    pub fn declare(&mut self, ids: Vec<&'static str>)
    {
        for id in ids {
            if self.rule_exps.contains_key(id) {
                panic!("The rule \"{}\" already used.", id);
            }

            let rule = Box::new(Rule::new(None));
            
            self.rule_exps.insert(id, RuleExpr {
                // TODO Maybe remove? id: id,
                is_defined: false,
                rule: Box::into_raw(rule),
            });
        }
    }
    
    // TODO Make a type of this return type.
    pub fn scan(&self, root_id: &str, code: &str) -> Result<Vec<T>, Vec<RuleError>>
    {
        let mut dummy = NoShared {};

        match self.rule_exps.get(root_id) {
            Some(r) => {
                unsafe {
                    (*r.rule).scan(code, &mut dummy)
                }
            },
            None => panic!("Rule with id \"{}\" not found.", root_id),
        }
    }
    
    pub fn ws(&mut self, expr: &str) 
    {
        let mut shared = GrammerShared {
            rule_exps: &self.rule_exps,
            keep_ws: &self.keep_ws,
        };

        match self.grammer.scan(expr, &mut shared) {
            Ok(mut b) => {
                if b.len() != 1 {
                    panic!("Error compiling rule expression.");
                }
                
                let r = b.pop().unwrap().rule.unwrap();
                self.keep_ws.clear().one(r);
            },
            Err(_) => panic!("Error compiling rule expression."),
        }
    }
    
    // TODO Replace with add_range_raw.
    fn add_range(rule: Rule<T, NoShared>, ctx: &ParseContext<T>) -> Rule<T, NoShared>
    {
        match ctx.range_type {
            RangeType::AtLeast => {
                let mut r = Rule::new(None);
                r.at_least(ctx.arg1, rule);
                r
            },
            RangeType::AtMost => {
                let mut r = Rule::new(None);
                r.at_most(ctx.arg1, rule);
                r
            },
            RangeType::Between => {
                let mut r = Rule::new(None);
                r.between(ctx.arg1, ctx.arg2, rule);
                r
            },
            RangeType::Exact => {
                let mut r = Rule::new(None);
                r.exact(ctx.arg1, rule);
                r
            },
            RangeType::NoRangeType => {
                rule
            },
            RangeType::Not => {
                panic!("Application error")     // TODO Fix this, this is `unreachable!()`
            },
        }
    }

    fn add_range_raw(rule: *const Rule<T, NoShared>, ctx: &ParseContext<T>) -> Rule<T, NoShared>
    {
        match ctx.range_type {
            RangeType::AtLeast => {
                let mut r = Rule::new(None);
                unsafe { r.at_least_raw(ctx.arg1, rule); }
                r
            },
            RangeType::AtMost => {
                let mut r = Rule::new(None);
                unsafe { r.at_most_raw(ctx.arg1, rule); }
                r
            },
            RangeType::Between => {
                let mut r = Rule::new(None);
                unsafe { r.between_raw(ctx.arg1, ctx.arg2, rule); }
                r
            },
            RangeType::Exact => {
                let mut r = Rule::new(None);
                unsafe { r.exact_raw(ctx.arg1, rule); }
                r
            },
            RangeType::NoRangeType => {
                let mut r = Rule::new(None);
                unsafe { r.one_raw(rule); }
                r
            },
            RangeType::Not => {
                panic!("Application error")     // TODO Fix this, this is `unreachable!()`
            },
        }
    }
}