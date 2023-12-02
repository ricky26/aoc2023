use std::sync::OnceLock;

use aho_corasick::{AhoCorasick, MatchKind};

static PATTERNS: &'static [&'static str] = &[
    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9",
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

static MATCHER: OnceLock<AhoCorasick> = OnceLock::new();

fn matcher() -> &'static AhoCorasick {
    MATCHER.get_or_init(|| AhoCorasick::builder()
        .match_kind(MatchKind::LeftmostFirst)
        .build(PATTERNS)
        .expect("invalid digit patterns"))
}

#[derive(Clone)]
pub struct DigitMatches<'a> {
    s: &'a str,
}

impl<'a> Iterator for DigitMatches<'a> {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(m) = matcher().find(self.s) {
            let v = (m.pattern().as_u32() % 10) as u8;
            self.s = &self.s[1..];
            Some(v)
        } else {
            self.s = "";
            None
        }
    }
}

pub trait FindNumbersExt {
    fn find_digits(&self) -> DigitMatches;
}

impl FindNumbersExt for str {
    fn find_digits(&self) -> DigitMatches {
        DigitMatches {
            s: self,
        }
    }
}

pub fn find_two_digits(line: &str) -> Option<u32> {
    let digits = line.chars().filter_map(|c| c.to_digit(10));
    let Some(first) = digits.clone().next() else {
        return None;
    };
    let Some(last) = digits.rev().next() else {
        return None;
    };
    Some(first * 10 + last)
}

pub fn find_two_digits_words(line: &str) -> Option<u32> {
    let digits = line.find_digits();
    let Some(first) = digits.clone().next() else {
        return None;
    };
    let Some(last) = digits.last() else {
        return None;
    };
    Some(first as u32 * 10 + (last as u32))
}
