use crate::Id;

#[derive(Debug)]
pub struct EClass<'tk, N> {
    pub enodes: Vec<N>,
    pub parents: Vec<(N, Id<'tk>)>,
}
