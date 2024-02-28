use crate::token::TokenKind;

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Expr {
    Ident(String),
    Literal(Literal),
    Binary { op: TokenKind, lhs: Box<Expr>, rhs: Box<Expr> },
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Literal {
    Int(usize),
    Float(f64),
    String(String),
}

#[inline(always)]
pub(crate) fn binop(lhs: Expr, op: TokenKind, rhs: Expr) -> Expr {
    Expr::Binary { op,
        lhs: Box::new(lhs),
        rhs: Box::new(rhs),
    }
}

#[inline(always)]
pub(crate) const fn int(it: usize) -> Expr {
    Expr::Literal(Literal::Int(it))
}

#[inline(always)]
pub(crate) const fn float(it: f64) -> Expr {
    Expr::Literal(Literal::Float(it))
}

#[inline(always)]
pub(crate) fn string<S: ToString>(it: S) -> Expr {
    Expr::Literal(Literal::String(it.to_string()))
}