#[derive(Debug)]
pub struct EClass<N> {
    pub enodes: Vec<N>,
    pub parents: Vec<N>,
}
