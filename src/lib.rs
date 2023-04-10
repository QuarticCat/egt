mod eclass;
mod egraph;
mod enode;
mod utils;

use std::marker::PhantomData;

type Token<'tk> = PhantomData<*mut &'tk ()>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Id<'tk> {
    val: u32,
    _token: Token<'tk>,
}
