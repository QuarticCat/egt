use crate::eclass::EClass;
use crate::utils::{HashMap, UnionFind};
use crate::Id;

#[derive(Debug, Default)]
pub struct EGraph<'tk, N> {
    union_find: UnionFind<'tk>,
    memo: HashMap<N, Id<'tk>>,
    eclasses: HashMap<Id<'tk>, EClass<N>>, // use array?
    pending: Vec<Id<'tk>>,
}
