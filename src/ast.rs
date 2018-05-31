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
    Pattern(Pattern),
    Patterns(Vec<Pattern>),
    Range { min: u64, max: u64 },
    Whitespace { min: u64, max: u64 },
}

// TODO Implement other not's.
#[derive(Clone, Debug)]
pub enum Pattern {
    AlterChars { replacements: Vec<AlterChar>, min: u64, max: u64 },
    AnyChar { min: u64, max: u64 },
    AnyCharExcept { chars: Vec<char>, min: u64, max: u64 },
    AnyOf { patterns: Vec<Vec<Pattern>>, min: u64, max: u64 },
    CharRanges { ranges: Vec<CharRange>, min: u64, max: u64 },
    Eof,
    Id { name: String, min: u64, max: u64 },
    Literal { not: bool, text: String, min: u64, max: u64 },
    Whitespace { min: u64, max: u64 },
}

impl From<(bool, ParseData, ParseData)> for Pattern {
    fn from(val: (bool, ParseData, ParseData)) -> Self {
        match val {
            (_, ParseData::AlterChars(replacements), ParseData::Range { min, max }) => Pattern::AlterChars { replacements, min, max },
            (_, ParseData::AnyChar, ParseData::Range { min, max }) => Pattern::AnyChar { min, max },
            (_, ParseData::AnyCharExcept(chars), ParseData::Range { min, max }) => Pattern::AnyCharExcept { chars, min, max },
            (_, ParseData::AnyOf(patterns), ParseData::Range { min, max }) => Pattern::AnyOf { patterns, min, max },
            (_, ParseData::CharRanges(ranges), ParseData::Range { min, max }) => Pattern::CharRanges { ranges, min, max },
            (_, ParseData::Eof, _) => Pattern::Eof,
            (_, ParseData::Id(name), ParseData::Range { min, max }) => Pattern::Id { name, min, max },
            (not, ParseData::Literal(text), ParseData::Range { min, max }) => Pattern::Literal { not, text, min, max },
            (_, ParseData::Whitespace { min, max }, _) => Pattern::Whitespace { min, max },
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