// Copyright (c) 2015-2020 Vincent van Ingen <code@abitvin.net>
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>
// This file may not be copied, modified, or distributed except according to those terms.

#[derive(Clone, Debug)]
pub struct AlterText { 
    pub find: String, 
    pub replace: String 
}

#[derive(Clone, Debug)]
pub struct CharRange {
    pub start: char,
    pub end: char,
}

#[derive(Debug)]
pub enum ParseData {
    AlterText { find: String, replace: String },
    AlterTexts(Vec<AlterText>),
    AnyChar,
    AnyCharExcept(Vec<char>),
    AnyOf(Vec<Vec<Clause>>),
    CharRange { start: char, end: char },
    CharRanges(Vec<CharRange>),
    Clause(Clause),
    Clauses(Vec<Clause>),
    Eof,
    Integer(u64),
    Id(String),
    Literal(String),
    NoBacktrack(String),
    Not,
    Range { min: u64, max: u64 },
    Text(String),
    Whitespace { min: u64, max: u64 },
}

#[derive(Clone, Debug)]
pub enum Clause {
    AlterTexts { replacements: Vec<AlterText>, min: u64, max: u64 },
    AnyChar { not: bool, min: u64, max: u64 },
    AnyCharExcept { not: bool, chars: Vec<char>, min: u64, max: u64 },
    AnyOf { not: bool, sentences: Vec<Vec<Clause>>, min: u64, max: u64 },
    CharRanges { not: bool, ranges: Vec<CharRange>, min: u64, max: u64 },
    Eof,
    Id { not: bool, name: String, min: u64, max: u64 },
    Literal { not: bool, text: String, min: u64, max: u64 },
    NoBacktrack(String),
    Whitespace { min: u64, max: u64 },
}

impl From<(bool, ParseData, ParseData)> for Clause {
    fn from(val: (bool, ParseData, ParseData)) -> Self {
        match val {
            (false, ParseData::AlterTexts(replacements), ParseData::Range { min, max }) => Clause::AlterTexts { replacements, min, max },
            (not, ParseData::AnyChar, ParseData::Range { min, max }) => Clause::AnyChar { not, min, max },
            (not, ParseData::AnyCharExcept(chars), ParseData::Range { min, max }) => Clause::AnyCharExcept { not, chars, min, max },
            (not, ParseData::AnyOf(sentences), ParseData::Range { min, max }) => Clause::AnyOf { not, sentences, min, max },
            (not, ParseData::CharRanges(ranges), ParseData::Range { min, max }) => Clause::CharRanges { not, ranges, min, max },
            (false, ParseData::Eof, ParseData::Range { min: 1, max: 1 }) => Clause::Eof,
            (not, ParseData::Id(name), ParseData::Range { min, max }) => Clause::Id { not, name, min, max },
            (not, ParseData::Literal(text), ParseData::Range { min, max }) => Clause::Literal { not, text, min, max },
            (false, ParseData::NoBacktrack(err_msg), ParseData::Range { min: 1, max: 1 }) => Clause::NoBacktrack(err_msg),
            (false, ParseData::Whitespace { min, max: ::std::u64::MAX }, _) => Clause::Whitespace { min, max: ::std::u64::MAX },
            (not, clause, range) => unreachable!("Unexpected match of\n- not: {},\n- clause: {:?}\n- range: {:?}", not, clause, range)
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

    pub fn unwrap_alter_text(self) -> AlterText {
        match self {
            ParseData::AlterText { find, replace } => AlterText { find, replace },
            _ => panic!("Not a ParseData::AlterText."),
        }
    }

    pub fn unwrap_char_range(self) -> CharRange {
        match self {
            ParseData::CharRange { start, end } => CharRange { start, end },
            _ => panic!("Not a ParseData::CharRange."),
        }
    }

    pub fn unwrap_clause(self) -> Clause {
        match self {
            ParseData::Clause(value) => value,
            _ => panic!("Not a ParseData::Clause."),
        }
    }

    pub fn unwrap_clauses(self) -> Vec<Clause> {
        match self {
            ParseData::Clauses(value) => value,
            _ => panic!("Not a ParseData::Clauses."),
        }
    }

    pub fn unwrap_int(self) -> u64 {
        match self {
            ParseData::Integer(value) => value,
            _ => panic!("Not a ParseData::Integer."),
        }
    }

    pub fn unwrap_text(self) -> String {
        match self {
            ParseData::Text(value) => value,
            _ => panic!("Not a ParseData::Text."),
        }
    }
}