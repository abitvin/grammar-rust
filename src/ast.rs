// Copyright (c) 2015-2018 Vincent van Ingen <code@abitvin.net>
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
    Not,
    Range { min: u64, max: u64 },
    Text(String),
    Whitespace { min: u64, max: u64 },
}

// TODO Implement other not's.
#[derive(Clone, Debug)]
pub enum Clause {
    AlterTexts { replacements: Vec<AlterText>, min: u64, max: u64 },
    AnyChar { min: u64, max: u64 },
    AnyCharExcept { chars: Vec<char>, min: u64, max: u64 },
    AnyOf { sentences: Vec<Vec<Clause>>, min: u64, max: u64 },
    CharRanges { ranges: Vec<CharRange>, min: u64, max: u64 },
    Eof,
    Id { name: String, min: u64, max: u64 },
    Literal { not: bool, text: String, min: u64, max: u64 },
    Whitespace { min: u64, max: u64 },
}

impl From<(bool, ParseData, ParseData)> for Clause {
    fn from(val: (bool, ParseData, ParseData)) -> Self {
        match val {
            (_, ParseData::AlterTexts(replacements), ParseData::Range { min, max }) => Clause::AlterTexts { replacements, min, max },
            (_, ParseData::AnyChar, ParseData::Range { min, max }) => Clause::AnyChar { min, max },
            (_, ParseData::AnyCharExcept(chars), ParseData::Range { min, max }) => Clause::AnyCharExcept { chars, min, max },
            (_, ParseData::AnyOf(sentences), ParseData::Range { min, max }) => Clause::AnyOf { sentences, min, max },
            (_, ParseData::CharRanges(ranges), ParseData::Range { min, max }) => Clause::CharRanges { ranges, min, max },
            (_, ParseData::Eof, _) => Clause::Eof,
            (_, ParseData::Id(name), ParseData::Range { min, max }) => Clause::Id { name, min, max },
            (not, ParseData::Literal(text), ParseData::Range { min, max }) => Clause::Literal { not, text, min, max },
            (_, ParseData::Whitespace { min, max }, _) => Clause::Whitespace { min, max },
            (not, clause, range) => unreachable!("Not: {}, Clause: {:?}\nRange: {:?}", not, clause, range)
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