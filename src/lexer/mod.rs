use rules::{unambigious_char, RULES};

use crate::token::{T, Token, TokenKind, Span};

mod rules;
pub(crate) mod util;

pub(crate) struct Lexer<'a> {
    input: &'a str,
    position: usize,
    finished: bool,
}

pub(crate) fn new(input: &str) -> Lexer {
    Lexer {
        input,
        position: 0,
        finished: false,
    }
}

impl<'a> Lexer<'a> {
    #[inline(always)]
    fn span(&self, start: usize) -> Span {
        Span { start, end: self.position }
    }
}

fn next_token(input: &str) -> (TokenKind, usize) {
    let next = input.chars().next().unwrap();

    if next.is_whitespace() {
        (
            T![' '],
            input // Skip over the rest of the whitespace
                .char_indices()
                .take_while(|(_, c)| c.is_whitespace())
                .last()
                .unwrap()
                .0 + 1,
        )
    } else {
        // look for a valid token else accumulate a span for the `invalid` token
        valid_token(input).unwrap_or_else(|| (T![invalid], invalid_size(input)))
    }
}

fn valid_token(input: &str) -> Option<(TokenKind, usize)> {
    let next = input.chars().next()?;

    Some(if let Some(kind) = unambigious_char(next) {
        (kind, 1)
    } else {
        RULES // Iterate over all the available rules, check if they match
            .iter()
            .filter_map(|rule| {
                Some((rule.kind, (rule.matches)(input)?))
            })
            // The result of the rule that munches the most tokens will be used
            .max_by_key(|&(_, len)| len)?
    })
}

fn invalid_size(input: &str) -> usize {
    input
        .char_indices()
        .find(|(pos, _)| valid_token(&input[*pos..]).is_some())
        .map(|i| i.0)
        .unwrap_or_else(|| input.len())
}

impl Iterator for Lexer<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished { None? }

        let start_position = self.position;

        if self.position >= self.input.len() {
            self.finished = true;

            return Some(Token {
                kind: T![EOF],
                span: self.span(start_position),
            })
        }

        let (kind, len) = next_token(&self.input[self.position..]);
        self.position += len;

        Some(Token { kind,
            span: self.span(start_position)
        })
    }
}

mod tests {
    #![allow(unused)]

    use crate::lexer;

    use crate::token::{Token, T, Span};

    use crate::Unit;

    macro_rules! assert_tokens {
        ($tokens:expr, T![$(emit_whitespace $(@$($_:tt)* $ews:tt)?)?;
            $($kind:tt)*
        ]) => (
            let mut it = $tokens.iter()
                $($($ews)? .filter(|t| t.kind != T![' ']))?;

            $(
                let token = it.next().expect("not enough tokens");
                assert_eq!(token.kind, T![$kind]);
            )*
        );
    }

    #[test]
    fn emit_whitespace() -> Unit {
        let tokens: Vec<Token> = lexer::new("1 2").collect();

        assert_tokens!(tokens, T![emit_whitespace;
            int /* ' ' */ int EOF
        ]);
    }

    #[test]
    fn basic_int() -> Unit {
        let tokens: Vec<Token> = lexer::new("1").collect();

        assert_tokens!(tokens, T![;
            int EOF
        ]);
    }

    #[test]
    fn basic_float() -> Unit {
        let tokens: Vec<Token> = lexer::new("1.125").collect();

        assert_tokens!(tokens, T![;
            float EOF
        ]);
    }
    
    #[test]
    fn basic_string() -> Unit {
        let tokens: Vec<Token> = lexer::new(" \"Hello\" ").collect();
        
        assert_tokens!(tokens, T![;
            ' ' string ' ' EOF
        ]);
    }
    
    #[test]
    fn string_sum() -> Unit {
        let tokens: Vec<Token> = lexer::new("\"Hello\" + 1").collect();
        
        assert_tokens!(tokens, T![; 
            string ' ' + ' ' int
        ]);
    }

    #[test]
    fn accumulated_int() -> Unit {
        let tokens: Vec<Token> = lexer::new("123").collect();

        assert_tokens!(tokens, T![;
            int EOF
        ]);

        let span = tokens.first().unwrap().span;

        assert_eq!(span.start, 0);
        assert_eq!(span.end, 3); // 0..<3
    }

    #[test]
    fn basic_sum() -> Unit {
        let tokens: Vec<Token> = lexer::new("1 + 3").collect();

        assert_tokens!(tokens, T![emit_whitespace;
            int + int EOF
        ]);
    }

    #[test]
    fn maybe_whitespace() -> Unit {
        let tokens: Vec<Token> = lexer::new(" 1 2").collect();

        assert_tokens!(tokens, T![;
            ' ' int ' ' int EOF
        ]);
    }

    #[test]
    fn whitespace_accumulation() -> Unit {
        // 3 ' ', int, 2 ' '
        let tokens: Vec<Token> = lexer::new("   1  ").collect();

        assert_tokens!(tokens, T![;
            ' ' int ' ' EOF
        ]);
    }

    #[test]
    fn basic_ident() -> Unit {
        let tokens: Vec<Token> = lexer::new("_id9").collect();

        assert_tokens!(tokens, T![emit_whitespace;
            ident EOF
        ]);
    }

    #[test]
    fn alphabet_ident() -> Unit {
        let tokens: Vec<Token> = lexer::new("_the_quick_brown_fox_jumped_over_the_lazy_dog_0123456789").collect();

        assert_tokens!(tokens, T![emit_whitespace;
            ident EOF
        ]);
    }

    #[test]
    fn keyword_let() -> Unit {
        let tokens: Vec<Token> = lexer::new("let").collect();

        assert_tokens!(tokens, T![emit_whitespace;
            let EOF
        ]);
    }

    #[test]
    fn keyword_let_with_following_ident() -> Unit {
        let tokens: Vec<Token> = lexer::new("let banana").collect();

        assert_tokens!(tokens, T![emit_whitespace;
            let ident EOF
        ]);
    }

    #[test]
    fn variable_declaration() -> Unit {
        let tokens: Vec<Token> = lexer::new("let banana = 1;").collect();

        assert_tokens!(tokens, T![emit_whitespace;
            let ident = int ; EOF
        ]);
    }
}
