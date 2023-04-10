use crate::eclass::EClass;
use crate::enode::ENode;
use crate::utils::{HashMap, UnionFind};
use crate::Id;

#[derive(Debug, Default)]
pub struct EGraph<'tk, N> {
    union_find: UnionFind<'tk>,
    /// e-node -> canonical id (after rebuild)
    memo: HashMap<N, Id<'tk>>,
    /// canonical id -> e-class
    eclasses: HashMap<Id<'tk>, EClass<N>>,
    pending: Vec<Id<'tk>>,
}

impl<'tk, N: ENode<'tk>> EGraph<'tk, N> {
    pub fn add(&mut self, mut enode: N) -> Id<'tk> {
        for id in enode.children_mut() {
            *id = self.union_find.find(*id);
        }

        if let Some(&existing_id) = self.memo.get(&enode) {
            existing_id
        } else {
            for id in enode.children() {
                let eclass = unsafe { self.eclasses.get_mut(id).unwrap_unchecked() };
                eclass.parents.push(enode.clone());
            }

            let id = self.union_find.add();
            let eclass = EClass {
                enodes: vec![enode.clone()],
                parents: Default::default(),
            };
            self.eclasses.insert(id, eclass);
            self.memo.insert(enode, id);

            id
        }
    }
}
