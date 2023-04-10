mod eclass;
mod egraph;
mod enode;
mod utils;

pub use egraph::EGraph;

use std::marker::PhantomData;

type Token<'tk> = PhantomData<*mut &'tk ()>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Id<'tk> {
    val: u32,
    _tk: Token<'tk>,
}
