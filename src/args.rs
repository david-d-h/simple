use clap::Parser;

#[derive(Parser)]
pub(crate) struct Args {
    pub script: Option<String>,
}

#[inline(always)]
pub(crate) fn parse() -> Args {
    Args::parse()
}