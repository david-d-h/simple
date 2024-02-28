use crate::token::{T, TokenKind, Token};

use super::Parser;
use super::ast::{self, binop, int, float, string, ident};

trait Operator {
    fn binary_binding_power(&self) -> Option<(u8, u8)>;
}

impl Operator for TokenKind {
    fn binary_binding_power(&self) -> Option<(u8, u8)> {
        Some(match self {
            T![+] | T![-] => (1, 2),
            T![*] | T![/] => (3, 4),
            _ => None?,
        })
    }
}

impl<'a, I> Parser<'a, I>
    where I: Iterator<Item=Token>
{
    pub(super) fn parse_expression(&mut self, binding_power: u8) -> ast::Expr {
        let mut lhs = match self.peek() {
            // ident is here temporarily.
            | lit @ (T![int] | T![float] | T![string] | T![ident]) => {
                let literal_text = {
                    let literal_token = self.next().unwrap();
                    self.text(literal_token)
                };

                match lit {
                    | T![int] => int(literal_text
                        .parse::<usize>()
                        .expect(&format!(
                            "Invalid integer literal: {}",
                            literal_text,
                        ))),
                    | T![float] => float(literal_text
                        .parse::<f64>()
                        .expect(&format!(
                            "Invalid float literal: {}",
                            literal_text,
                        ))),
                    | T![string] => string(&literal_text[1..literal_text.len() - 1]),
                    | T![ident] => ident(literal_text),
                    | _ => unreachable!(),
                }
            }
            | T!['('] => {
                self.consume(T!['(']);
                let expr = self.parse_expression(0);
                self.consume(T![')']);
                expr
            }
            _ => todo!(),
        };


        loop {
            let op = match self.peek() {
                op @ (T![+] | T![-] | T![/] | T![*]) => op,
                T![EOF] | T![')'] => break,
                kind => panic!("Unknown operator: {}", kind),
            };

            if let Some((lbp, rbp)) = op.binary_binding_power() {
                if binding_power > lbp {
                    break;
                }

                self.consume(op);
                let rhs = self.parse_expression(rbp);

                lhs = binop(lhs, op, rhs);

                continue;
            }

            break;
        }

        lhs
    }
}

mod tests {
    #![allow(unused)]

    use crate::token::{T, TokenKind};

    use crate::parser::{self, ast::{self, binop, int, float, string}};
    use crate::parser::ast::ident;

    use crate::Unit;

    #[inline(always)]
    fn parse(input: &str) -> ast::Expr {
        parser::new(input).parse_expression(0)
    }

    #[test]
    fn basic_int() -> Unit {
        let expr = parse("1");

        assert_eq!(expr, int(1));
    }

    #[test]
    fn basic_float() -> Unit {
        let expr = parse("1.125");

        assert_eq!(expr, float(1.125));
    }

    #[test]
    fn basic_string() -> Unit {
        let expr = parse("\"hello\"");

        assert_eq!(expr, string("hello"))
    }

    #[test]
    fn basic_ident() -> Unit {
        let expr = parse("hallo");

        assert_eq!(expr, ident("hallo"));
    }

    #[test]
    fn binary_op() -> Unit {
        let expr = parse("3 * 2");

        assert_eq!(expr, binop(int(3), T![*], int(2)));
    }

    #[test]
    fn binary_op_binding_power() -> Unit {
        let expr = parse("1 + 2 * 3");

        assert_eq!(expr, binop(
            int(1),
            T![+],
            binop(int(2), T![*], int(3)),
        ));
    }

    #[test]
    fn grouped_expression_has_priority_over_binding_power() -> Unit {
        let expr = parse("(1 + 2) * 3");

        assert_eq!(expr, binop(
            binop(int(1), T![+], int(2)),
            T![*],
            int(3),
        ));
    }
}
