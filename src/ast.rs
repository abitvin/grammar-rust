// Copyright (c) 2015-2018 Vincent van Ingen <code@abitvin.net>
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>
// This file may not be copied, modified, or distributed except according to those terms.

// TODO Do not make these public.

// TODO This is not a alter char.
#[derive(Clone, Debug)]
pub struct AlterChar { 
    pub (crate) find: String, 
    pub (crate) replace: String 
}

#[derive(Clone, Debug)]
pub struct CharRange {
    pub (crate) start: char,
    pub (crate) end: char,
}

#[derive(Clone, Debug)]
pub enum Range {
    None,
    //Not,
    Quantity { min: u64, max: u64 },
}

#[derive(Debug)]
pub enum ParseData {
    AlterChar { find: String, replace: String },
    AlterChars(Vec<AlterChar>),
    AnyChar,
    AnyCharExcept(Vec<char>),
    AnyOf(Vec<Vec<Pattern>>),
    CharRange { start: char, end: char },
    CharRanges(Vec<CharRange>),
    Eof,
    Integer(u64),
    Id(String),         // TODO Can we combine these?
    Literal(String),    // ..
    Not,
    NoRange,
    Pattern(Pattern),
    Patterns(Vec<Pattern>),
    Range { min: u64, max: u64 },
    Whitespace(Range),
}

// TODO Implement other not's.
#[derive(Clone, Debug)]
pub enum Pattern {
    AlterChars { replacements: Vec<AlterChar>, range: Range },
    AnyChar(Range),
    AnyCharExcept { chars: Vec<char>, range: Range },
    AnyOf { patterns: Vec<Vec<Pattern>>, range: Range },
    CharRanges { ranges: Vec<CharRange>, range: Range },
    Eof,
    Id { name: String, range: Range },
    Literal { not: bool, text: String, range: Range },
    Whitespace(Range),
}

impl From<(bool, ParseData, ParseData)> for Pattern {
    fn from(val: (bool, ParseData, ParseData)) -> Self {
        match val {
            (_, ParseData::AlterChars(replacements), ParseData::NoRange) => Pattern::AlterChars { replacements, range: Range::None },
            (_, ParseData::AlterChars(replacements), ParseData::Range { min, max }) => Pattern::AlterChars { replacements, range: Range::Quantity { min, max }},

            (_, ParseData::AnyChar, ParseData::NoRange) => Pattern::AnyChar(Range::None),
            (_, ParseData::AnyChar, ParseData::Range { min, max }) => Pattern::AnyChar(Range::Quantity { min, max }),

            (_, ParseData::AnyCharExcept(chars), ParseData::NoRange) => Pattern::AnyCharExcept { chars, range: Range::None },
            (_, ParseData::AnyCharExcept(chars), ParseData::Range { min, max }) => Pattern::AnyCharExcept { chars, range: Range::Quantity { min, max } },

            (_, ParseData::AnyOf(patterns), ParseData::NoRange) => Pattern::AnyOf { patterns, range: Range::None },
            (_, ParseData::AnyOf(patterns), ParseData::Range { min, max }) => Pattern::AnyOf { patterns, range: Range::Quantity { min, max } },

            (_, ParseData::CharRanges(ranges), ParseData::NoRange) => Pattern::CharRanges { ranges, range: Range::None },
            (_, ParseData::CharRanges(ranges), ParseData::Range { min, max }) => Pattern::CharRanges { ranges, range: Range::Quantity { min, max } },

            (_, ParseData::Eof, _) => Pattern::Eof,
            
            (_, ParseData::Id(name), ParseData::NoRange) => Pattern::Id { name, range: Range::None },
            (_, ParseData::Id(name), ParseData::Range { min, max }) => Pattern::Id { name, range: Range::Quantity { min, max } },

            (not, ParseData::Literal(text), ParseData::NoRange) => Pattern::Literal { not, text, range: Range::None },
            (not, ParseData::Literal(text), ParseData::Range { min, max }) => Pattern::Literal { not, text, range: Range::Quantity { min, max } },

            (_, ParseData::Whitespace(range), _) => Pattern::Whitespace(range),
            
            (not, pattern, range) => unreachable!("Not: {}, Pattern: {:?}\nRange: {:?}", not, pattern, range)
        }
    }
}

impl ParseData {
    pub fn is_not(&self) -> bool {
        match self {
            ParseData::Not => true,
            _ => false,
        }
    }

    pub fn unwrap_alter_text(self) -> AlterChar {
        match self {
            ParseData::AlterChar { find, replace } => AlterChar { find, replace },
            _ => panic!("Not a ParseData::AlterText."),
        }
    }

    pub fn unwrap_char_range(self) -> CharRange {
        match self {
            ParseData::CharRange { start, end } => CharRange { start, end },
            _ => panic!("Not a ParseData::CharRange."),
        }
    }

    pub fn unwrap_int(self) -> u64 {
        match self {
            ParseData::Integer(value) => value,
            _ => panic!("Not a ParseData::Integer."),
        }
    }

    pub fn unwrap_literal(self) -> String {
        match self {
            ParseData::Literal(value) => value,
            _ => panic!("Not a ParseData::Literal."),
        }
    }

    pub fn unwrap_pattern(self) -> Pattern {
        match self {
            ParseData::Pattern(value) => value,
            _ => panic!("Not a ParseData::Pattern."),
        }
    }

    pub fn unwrap_patterns(self) -> Vec<Pattern> {
        match self {
            ParseData::Patterns(value) => value,
            _ => panic!("Not a ParseData::Patterns."),
        }
    }
}