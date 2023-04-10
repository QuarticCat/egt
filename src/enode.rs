use crate::Id;
use std::hash::Hash;

pub trait ENode<'tk>: Hash + Eq + Clone {
    fn children(&self) -> &[Id<'tk>];

    fn children_mut(&mut self) -> &mut [Id<'tk>];
}
