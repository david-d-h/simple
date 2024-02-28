use std::iter::Iterator;

use super::util::Consume;

use crate::token::{T, TokenKind};

pub(super) const RULES: &'static [Rule] = &[
    IDENTIFIER,
    KEYWORD_LET,
    STRING,
    INTEGER,
    FLOAT,
];

pub(super) struct Rule {
    pub kind: TokenKind,
    pub matches: fn(&str) -> Option<usize>,
}

impl FnOnce<(&str,)> for Rule {
    type Output = Option<usize>;

    extern "rust-call" fn call_once(self, args: (&str,)) -> Self::Output {
        (self.matches)(args.0)
    }
}

const fn rule(kind: TokenKind, matches: fn(&str) -> Option<usize>) -> Rule {
    Rule { kind, matches }
}

macro_rules! keyword_rule {
    ($keyword:ident) => (rule(T![$keyword], const |input| {
        const STR: &'static str = stringify!($keyword);
        match const_str::starts_with!(input, STR) {
            true => Some(STR.len()),
            false => None,
        }
    }));
}

const FLOAT: Rule = rule(T![float], |input| Some({
    let whole_length = INTEGER(input)?;

    if input[whole_length..].chars().next()? != '.' { None? }

    let fraction_length = INTEGER(&input[whole_length + 1..])?;

    whole_length + 1 + fraction_length
}));

const INTEGER: Rule = rule(T![int], |input| Some(input
    .char_indices()
    .take_while(|(_, c)| c.is_ascii_digit())
    .last()?
    .0 + 1
));

const STRING: Rule = rule(T![string], |input| Some(input
    .chars()
    .peekable()
    .consume('"')?
    .position(|c| c == '"')? + 2
));

const KEYWORD_LET: Rule = keyword_rule!(let);

const IDENTIFIER: Rule = rule(T![ident], |input| Some(input
    .char_indices()
    .take_while(|(i, c)| match c {
        '0'..='9' if *i == 0 => false,
        '0'..='9' | 'a'..='z' | 'A'..='Z' | '_' => true,
        _ => false,
    })
    .last()?
    .0 + 1
));

pub(super) const fn unambigious_char(c: char) -> Option<TokenKind> {
    Some(match c {
        '+' => T![+],
        '-' => T![-],
        '*' => T![*],
        '/' => T![/],
        '=' => T![=],
        ';' => T![;],
        '(' => T!['('],
        ')' => T![')'],
        _ => return None,
    })
}
