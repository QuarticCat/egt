use crate::Id;

pub trait ENode {
    fn children(&self) -> &[Id];

    fn children_mut(&mut self) -> &mut [Id];
}
