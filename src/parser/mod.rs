mod expressions;
pub(crate) mod ast;

use std::iter::Peekable;

use crate::token::{T, Token, TokenKind};

use crate::lexer::{self, Lexer};

pub(crate) struct TokenIter<'a> {
    lexer: Lexer<'a>,
}

impl<'a> TokenIter<'a> {
    pub(crate) fn new(input: &'a str) -> Self {
        Self {
            lexer: lexer::new(input),
        }
    }
}

impl<'a> Iterator for TokenIter<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let next = self.lexer.next()?;

            if next.kind == T![' '] {
                continue;
            }

            return Some(next);
        }
    }
}

pub(crate) struct Parser<'a, I>
    where I: Iterator,
{
    input: &'a str,
    tokens: Peekable<I>,
}

pub(crate) fn new(input: &str) -> Parser<TokenIter> {
    Parser {
        input,
        tokens: TokenIter::new(input).peekable(),
    }
}

pub(crate) fn parse(input: &str) -> ast::Expr {
    new(input).parse_expression(0)
}

impl<'input, I> Parser<'input, I>
    where I: Iterator<Item=Token>,
{
    fn text(&self, token: Token) -> &'input str {
        token.text(&self.input)
    }

    fn peek(&mut self) -> TokenKind {
        self.tokens.peek().map(|token| token.kind).unwrap_or(T![EOF])
    }

    fn at(&mut self, kind: TokenKind) -> bool {
        self.peek() == kind
    }

    fn next(&mut self) -> Option<Token> {
        self.tokens.next()
    }

    fn consume(&mut self, expected: TokenKind) -> TokenKind {
        let token = self.next().expect(&format!(
            "Expected to consume `{}`, but there was no next token",
            expected,
        ));

        debug_assert_eq!(
            token.kind, expected,
            "Expected to consume `{}`, but found `{}`",
            expected, token.kind,
        );
        
        expected
    }
}

