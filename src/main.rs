#![allow(incomplete_features)]
#![feature(
    exclusive_range_pattern,
    const_closures,
    fn_traits,
    unboxed_closures,
)]

mod lexer;
mod token;
mod args;
mod state;
mod parser;

pub(crate) type Unit = ();

fn main() -> Unit {
    //
}
