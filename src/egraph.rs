use crate::utils::{HashMap, UnionFind};
use crate::Id;
use std::hash::Hash;

pub trait ENode<'tk>: Hash + Eq + Clone {
    fn children(&self) -> &[Id<'tk>];

    fn children_mut(&mut self) -> &mut [Id<'tk>];
}

#[derive(Debug)]
pub struct EClass<'tk, N> {
    pub enodes: Vec<N>,
    // (uncanon e-node, uncanon id)
    pub parents: Vec<(N, Id<'tk>)>,
}

#[derive(Debug, Default)]
pub struct EGraph<'tk, N> {
    union_find: UnionFind<'tk>,
    // canon e-node -> uncanon id
    memo: HashMap<N, Id<'tk>>,
    // canon id -> e-class
    eclasses: HashMap<Id<'tk>, EClass<'tk, N>>,
    // (uncanon e-node, uncanon id)
    pending: Vec<(N, Id<'tk>)>,
}

impl<'tk, N: ENode<'tk>> EGraph<'tk, N> {
    fn canonicalize(&mut self, enode: &mut N) {
        for id in enode.children_mut() {
            *id = self.union_find.find(*id);
        }
    }

    pub fn add(&mut self, mut enode: N) -> Id<'tk> {
        self.canonicalize(&mut enode);

        if let Some(existing_id) = self.memo.get(&enode) {
            *existing_id
        } else {
            let cid = self.union_find.add();

            for child_cid in enode.children() {
                let eclass = unsafe { self.eclasses.get_mut(child_cid).unwrap_unchecked() };
                eclass.parents.push((enode.clone(), cid));
            }

            let eclass = EClass {
                enodes: vec![enode.clone()],
                parents: Default::default(),
            };
            self.eclasses.insert_unique_unchecked(cid, eclass);
            self.memo.insert_unique_unchecked(enode, cid);

            cid
        }
    }

    pub fn union(&mut self, id1: Id<'tk>, id2: Id<'tk>) {
        let mut cid1 = self.union_find.find(id1);
        let mut cid2 = self.union_find.find(id2);
        if cid1 == cid2 {
            return;
        }

        let eclass1 = unsafe { self.eclasses.get(&cid1).unwrap_unchecked() };
        let eclass2 = unsafe { self.eclasses.get(&cid2).unwrap_unchecked() };
        if eclass1.parents.len() < eclass2.parents.len() {
            std::mem::swap(&mut cid1, &mut cid2);
        }

        let mut eclass2 = unsafe { self.eclasses.remove(&cid2).unwrap_unchecked() };
        let eclass1 = unsafe { self.eclasses.get_mut(&cid1).unwrap_unchecked() };

        self.union_find.union(cid1, cid2);
        self.pending.extend(eclass2.parents.iter().cloned());

        if eclass1.enodes.len() < eclass2.enodes.len() {
            std::mem::swap(&mut eclass1.enodes, &mut eclass2.enodes);
        }
        eclass1.enodes.extend(eclass2.enodes);
        eclass1.parents.extend(eclass2.parents);
    }
}
