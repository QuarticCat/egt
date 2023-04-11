use crate::eclass::EClass;
use crate::enode::ENode;
use crate::utils::{HashMap, UnionFind};
use crate::Id;

#[derive(Debug, Default)]
pub struct EGraph<'tk, N> {
    union_find: UnionFind<'tk>,
    memo: HashMap<N, Id<'tk>>, // id may not be canonical
    eclasses: HashMap<Id<'tk>, EClass<N>>,
    pending: Vec<Id<'tk>>,
}

impl<'tk, N: ENode<'tk>> EGraph<'tk, N> {
    fn canonicalize(&mut self, enode: &mut N) {
        for id in enode.children_mut() {
            *id = self.union_find.find(*id);
        }
    }

    pub fn add(&mut self, mut enode: N) -> Id<'tk> {
        self.canonicalize(&mut enode);

        if let Some(&existing_id) = self.memo.get(&enode) {
            existing_id
        } else {
            for cid in enode.children() {
                let eclass = unsafe { self.eclasses.get_mut(cid).unwrap_unchecked() };
                eclass.parents.push(enode.clone());
            }

            let cid = self.union_find.add();
            let eclass = EClass {
                enodes: vec![enode.clone()],
                parents: Default::default(),
            };
            self.eclasses.insert_unique_unchecked(cid, eclass);
            self.memo.insert_unique_unchecked(enode, cid);

            cid
        }
    }
}
