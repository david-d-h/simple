#[derive(Debug, Copy, Clone)]
pub(crate) struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl Token {
    pub(crate) fn text<'a>(&self, input: &'a str) -> &'a str {
        let Span { start, end } = self.span;
        &input[start..end]
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(crate) enum TokenKind {
    Plus,
    Minus,
    Star,
    Slash,
    Equals,
    Semicolon,
    LParen,
    RParen,
    Int,
    Float,
    String,
    Identifier,
    KeywordLet,
    Whitespace,
    Eof,
    Invalid,
}

#[derive(Debug, Copy, Clone)]
pub(crate) struct Span {
    pub start: usize,
    pub end: usize,
}

macro_rules! __gen_token_helper {
    ($enum:ty, [
        $($tt:tt => $ident:ident,)*
    ]) => {
        mod __t_gen {
            #[macro_export]
            macro_rules! T {
                $([$tt] => (<$enum>::$ident)
                ;)*
            }

            pub use T as _T;
        }

        pub(crate) use __t_gen::_T as T;

        impl ::std::fmt::Display for $enum {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(f, "{}", match self {
                    $(Self::$ident => stringify!($tt),)*
                })
            }
        }
    };
}

__gen_token_helper!(crate::token::TokenKind, [
    + => Plus,
    - => Minus,
    * => Star,
    / => Slash,
    = => Equals,
    ; => Semicolon,
    '(' => LParen,
    ')' => RParen,
    int => Int,
    float => Float,
    string => String,
    ident => Identifier,
    let => KeywordLet,
    ' ' => Whitespace,
    EOF => Eof,
    invalid => Invalid,
]);
