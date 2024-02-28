use anyhow::Result;

use crate::Unit;

#[derive(Default)]
pub(crate) struct State;

pub(crate) fn run(_state: &mut State) -> Result<Unit> {
    Ok(())
}