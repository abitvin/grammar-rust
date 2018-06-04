// Copyright (c) 2015-2018 Vincent van Ingen <code@abitvin.net>
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>
// This file may not be copied, modified, or distributed except according to those terms.

extern crate rule;

use ast::{Clause, ParseData};
use self::rule::Rule;

const ESC_CTRL_CHARS: [(&'static str, &'static str); 21] = [
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
];

pub fn root() -> Rule<ParseData> {
    let f = |mut b: Vec<ParseData>, _: &str| {
        match b.len() {
            1 => {
                ParseData::Clause(Clause::from((false, b.remove(0), ParseData::Range { min: 1, max: 1 })))
            },
            2 => {
                if b[0].is_not() {
                    b.remove(0);
                    ParseData::Clause(Clause::from((true, b.remove(0), ParseData::Range { min: 1, max: 1 })))
                }
                else {
                    ParseData::Clause(Clause::from((false, b.remove(0), b.remove(0))))
                }
            },
            3 => {
                b.remove(0);
                ParseData::Clause(Clause::from((true, b.remove(0), b.remove(0))))
            },
            _ => unreachable!()
        }
    };

    let clause = Rule::new(Some(Box::new(f)));
    let instr = instr(&clause);
    let ranges = ranges();
    let not = not();
    
    clause.maybe(&not).one(&instr).maybe(&ranges);

    let root = Rule::new(None);
    root.none_or_many(&clause);
    root

    /* TODO WIP
    let escaped_ctrl_chars = escaped_ctrl_chars();
    let clauses = Rule::new(Some(Box::new(f)));
    let ranges = ranges();
    let not = not();
    
    let alter_clause = alter(&escaped_ctrl_chars);

    let any_char_clause = Rule::new(None);
    any_char_clause.maybe(&not).one(&any_char()).maybe(&ranges);
    
    let any_char_except_clause = Rule::new(None);
    any_char_except_clause.maybe(&not).one(&any_char_except(&escaped_ctrl_chars)).maybe(&ranges);

    let any_of_clause = Rule::new(None);
    any_of_clause.maybe(&not).one(&any_of(&clauses)).maybe(&ranges);

    let at_least_one_ws_clause = at_least_one_ws();
    
    let char_ranges_clause = Rule::new(None);
    char_ranges_clause.maybe(&not).one(&char_ranges(&escaped_ctrl_chars)).maybe(&ranges);

    let eof_clause = eof();

    let id_clause = Rule::new(None);
    id_clause.maybe(&not).one(&id(&escaped_ctrl_chars)).maybe(&ranges);
    
    let literal_clause = Rule::new(None);
    literal_clause.maybe(&not).one(&literal(&escaped_ctrl_chars)).maybe(&ranges);

    let none_or_many_ws_clause = none_or_many_ws();
    
    clauses.any_of(vec![
        &any_char_clause, &at_least_one_ws_clause, &none_or_many_ws_clause, 
        &eof_clause, &alter_clause, &any_char_except_clause, 
        &char_ranges_clause, &id_clause, &any_of_clause, &literal_clause
    ]);

    let root = Rule::new(None);
    root.none_or_many(&clauses);
    root
    */
}

// Instructions

pub fn instr(sentence: &Rule<ParseData>) -> Rule<ParseData> {
    let escaped_ctrl_chars = escaped_ctrl_chars();
    let rule = Rule::new(None);
    
    rule.any_of(vec![
        &any_char(), &at_least_one_ws(), &none_or_many_ws(), &eof(), &alter(&escaped_ctrl_chars), 
        &any_char_except(&escaped_ctrl_chars), &char_ranges(&escaped_ctrl_chars), 
        &id(&escaped_ctrl_chars), &any_of(sentence), &literal(&escaped_ctrl_chars)
    ]);

    rule
}

pub fn any_char() -> Rule<ParseData> {
    let f = |_: Vec<ParseData>, _: &str| {
        ParseData::AnyChar
    };

    let rule = Rule::new(Some(Box::new(f)));
    rule.literal(".");
    rule
}

pub fn any_char_except(escaped_ctrl_chars: &Rule<ParseData>) -> Rule<ParseData> {
    let f = |_: Vec<ParseData>, l: &str| {
        let chars = String::from(l).chars().map(|c| c).collect();
        ParseData::AnyCharExcept(chars)
    };

    let any_other = Rule::new(None); 
    any_other.any_char_except(vec![']']);

    let chr = Rule::new(None); 
    chr.any_of(vec![escaped_ctrl_chars, &any_other]);

    let chars = Rule::new(Some(Box::new(f))); 
    chars.at_least(1, &chr);

    let rule = Rule::new(None); 
    rule.literal("[^").one(&chars).literal("]");
    rule
}

pub fn any_of(clause: &Rule<ParseData>) -> Rule<ParseData> {
    let any_of_fn = |b: Vec<ParseData>, _: &str| {
        let unwrapped = b
            .into_iter()
            .map(|x| x.unwrap_clauses())
            .collect();

        ParseData::AnyOf(unwrapped)
    };

    let sentence_fn = |b: Vec<ParseData>, _: &str| {
        let unwrapped = b
            .into_iter()
            .map(|x| x.unwrap_clause())
            .collect();

        ParseData::Clauses(unwrapped)
    };

    let sentence = Rule::new(Some(Box::new(sentence_fn)));
    sentence.at_least(1, clause);

    let more = Rule::new(None);
    more.literal("|").one(&sentence);

    let rule = Rule::new(Some(Box::new(any_of_fn)));
    rule.literal("(").one(&sentence).none_or_many(&more).literal(")");
    rule
}

pub fn alter(escaped_ctrl_chars: &Rule<ParseData>) -> Rule<ParseData> {
    let f = |b: Vec<ParseData>, _: &str| {
        let to_alter = b
            .into_iter()
            .map(|x| x.unwrap_alter_text())
            .collect();

        ParseData::AlterTexts(to_alter)
    };

    let alter_tuple = alter_tuple(escaped_ctrl_chars);

    let more = Rule::new(None);
    more.literal("|").one(&alter_tuple);

    let alter = Rule::new(Some(Box::new(f)));
    alter.literal("(~").one(&alter_tuple).none_or_many(&more).literal(")");
    alter
}

pub fn alter_tuple(escaped_ctrl_chars: &Rule<ParseData>) -> Rule<ParseData> {
    let tuple_fn = |mut b: Vec<ParseData>, _: &str| {
        let replace = b.pop().unwrap().unwrap_text();
        let find = b.pop().unwrap().unwrap_text();
        ParseData::AlterText{ find, replace }
    };

    let left_text_fn = |_: Vec<ParseData>, l: &str| {
        ParseData::Text(String::from(l))
    };

    let right_text_fn = |_: Vec<ParseData>, l: &str| {
        ParseData::Text(String::from(l))
    };

    let all_except_left_char = Rule::new(None);
    all_except_left_char.any_char_except(vec![',']);
    
    let left_char = Rule::new(None);
    left_char.any_of(vec![escaped_ctrl_chars, &all_except_left_char]);

    let left_text = Rule::new(Some(Box::new(left_text_fn)));
    left_text.at_least(1, &left_char);

    let all_except_right_char = Rule::new(None);
    all_except_right_char.any_char_except(vec!['|', ')']);
    
    let right_char = Rule::new(None);
    right_char.any_of(vec![escaped_ctrl_chars, &all_except_right_char]);

    let right_text = Rule::new(Some(Box::new(right_text_fn)));
    right_text.at_least(1, &right_char);

    let tuple = Rule::new(Some(Box::new(tuple_fn)));
    tuple.one(&left_text).literal(",").one(&right_text);
    tuple
}

pub fn char_ranges(escaped_ctrl_chars: &Rule<ParseData>) -> Rule<ParseData> {
    let char_range_fn = |_: Vec<ParseData>, l: &str| {
        let mut chars = l.chars();
        let start = chars.next().unwrap();
        let end = chars.skip(1).next().unwrap();
        ParseData::CharRange { start, end }
    };

    let char_ranges_fn = |b: Vec<ParseData>, _: &str| {
        let char_ranges = b
            .into_iter()
            .map(|x| x.unwrap_char_range())
            .collect();

        ParseData::CharRanges(char_ranges)
    };

    let char_range_char = char_range_char(escaped_ctrl_chars);

    let char_range = Rule::new(Some(Box::new(char_range_fn)));
    char_range.one(&char_range_char).literal("-").one(&char_range_char);

    let char_ranges = Rule::new(Some(Box::new(char_ranges_fn)));
    char_ranges.literal("[").at_least(1, &char_range).literal("]");

    char_ranges
}

pub fn char_range_char(escaped_ctrl_chars: &Rule<ParseData>) -> Rule<ParseData> {
    let all_except = Rule::new(None);
    all_except.any_char_except(vec!['-', ']']);

    let rule = Rule::new(None);
    rule.any_of(vec![escaped_ctrl_chars, &all_except]);
    rule
}

pub fn eof() -> Rule<ParseData> {
    let f = |_: Vec<ParseData>, _: &str| {
        ParseData::Eof
    };

    let rule = Rule::new(Some(Box::new(f)));
    rule.literal("$");
    rule
}

pub fn escaped_ctrl_chars() -> Rule<ParseData> {
    let rule = Rule::new(None);
    rule.alter(ESC_CTRL_CHARS.to_vec());
    rule
}

pub fn id(escaped_ctrl_chars: &Rule<ParseData>) -> Rule<ParseData> {
    let f = |_: Vec<ParseData>, l: &str| {
        ParseData::Id(String::from(l))
    };

    let any_char_except = Rule::new(None);
    any_char_except.any_char_except(vec!['>']);

    let chr = Rule::new(None);
    chr.any_of(vec![escaped_ctrl_chars, &any_char_except]);

    let id = Rule::new(Some(Box::new(f)));
    id.at_least(1, &chr);

    let rule = Rule::new(None);
    rule.literal("<").one(&id).literal(">");
    rule
}

pub fn integer() -> Rule<ParseData> {
    let f = |_: Vec<ParseData>, l: &str| {
        ParseData::Integer(l.parse::<u64>().unwrap())
    };

    let digit = Rule::new(None);
    digit.char_in('0', '9');

    let integer = Rule::new(Some(Box::new(f)));
    integer.at_least(1, &digit);
    integer
}

pub fn literal(escaped_ctrl_chars: &Rule<ParseData>) -> Rule<ParseData> {
    let f = |_: Vec<ParseData>, l: &str| {
        ParseData::Literal(String::from(l))
    };

    let all_except = Rule::new(None);
    all_except.any_char_except(vec!['<', '{', '(', ')', '|', '[', '+', '?', '*', '.', '$', ' ', '_', '!']);

    let chr = Rule::new(None);
    chr.any_of(vec![escaped_ctrl_chars, &all_except]);
    
    let rule = Rule::new(Some(Box::new(f)));
    rule.at_least(1, &chr);
    rule
}

pub fn at_least_one_ws() -> Rule<ParseData> {
    let f = |_: Vec<ParseData>, _: &str| {
        ParseData::Whitespace { min: 1, max: ::std::u64::MAX }
    };
    
    let rule = Rule::new(Some(Box::new(f)));
    rule.literal("_");
    rule
}

pub fn none_or_many_ws() -> Rule<ParseData> {
    let f = |_: Vec<ParseData>, _: &str| {
        ParseData::Whitespace { min: 0, max: ::std::u64::MAX }
    };

    let rule = Rule::new(Some(Box::new(f)));
    rule.literal(" ");
    rule
}

// Ranges

pub fn ranges() -> Rule<ParseData> {
    let integer = integer();

    let rule = Rule::new(None);
    
    rule.any_of(vec![
        &at_least(&integer), &at_least_one(), &at_most(&integer), 
        &between(&integer), &exact(&integer), &maybe(), &none_or_many()
    ]);

    rule
}

pub fn at_least(integer: &Rule<ParseData>) -> Rule<ParseData> {
    let f = |mut b: Vec<ParseData>, _: &str| {
        let count = b.pop().unwrap().unwrap_int();
        ParseData::Range { min: count, max: ::std::u64::MAX }
    };

    let rule = Rule::new(Some(Box::new(f)));
    rule.literal("{").one(integer).literal(",}");
    rule
}

pub fn at_least_one() -> Rule<ParseData> {
    let f = |_: Vec<ParseData>, _: &str| {
        ParseData::Range { min: 1, max: ::std::u64::MAX }
    };

    let rule = Rule::new(Some(Box::new(f)));
    rule.literal("+");
    rule
}

pub fn at_most(integer: &Rule<ParseData>) -> Rule<ParseData> {
    let f = |mut b: Vec<ParseData>, _: &str| {
        let count = b.pop().unwrap().unwrap_int();
        ParseData::Range { min: 0, max: count }
    };

    let rule = Rule::new(Some(Box::new(f)));
    rule.literal("{,").one(integer).literal("}");
    rule
}

pub fn between(integer: &Rule<ParseData>) -> Rule<ParseData> {
    let f = |mut b: Vec<ParseData>, _: &str| {
        let max = b.pop().unwrap().unwrap_int();
        let min = b.pop().unwrap().unwrap_int();
        ParseData::Range { min, max }
    };

    let rule = Rule::new(Some(Box::new(f)));
    rule.literal("{").one(integer).literal(",").one(integer).literal("}");
    rule
}

pub fn exact(integer: &Rule<ParseData>) -> Rule<ParseData> {
    let f = |mut b: Vec<ParseData>, _: &str| {
        let count = b.pop().unwrap().unwrap_int();
        ParseData::Range { min: count, max: count }
    };

    let rule = Rule::new(Some(Box::new(f)));
    rule.literal("{").one(integer).literal("}");
    rule
}

pub fn maybe() -> Rule<ParseData> {
    let f = |_: Vec<ParseData>, _: &str| {
        ParseData::Range { min: 0, max: 1 }
    };

    let rule = Rule::new(Some(Box::new(f)));
    rule.literal("?");
    rule
}

pub fn none_or_many() -> Rule<ParseData> {
    let f = |_: Vec<ParseData>, _: &str| {
        ParseData::Range { min: 0, max: ::std::u64::MAX }
    };

    let rule = Rule::new(Some(Box::new(f)));
    rule.literal("*");
    rule
}

pub fn not() -> Rule<ParseData> {
    let f = |_: Vec<ParseData>, _: &str| {
        ParseData::Not
    };

    let rule = Rule::new(Some(Box::new(f)));
    rule.literal("!");
    rule
}