use crate::Id;

#[derive(Debug, Default)]
pub struct UnionFind<'tk> {
    parents: Vec<Id<'tk>>,
}

impl<'tk> UnionFind<'tk> {
    fn parent(&self, id: Id<'tk>) -> Id<'tk> {
        unsafe { *self.parents.get_unchecked(id.val as usize) }
    }

    fn parent_mut(&mut self, id: Id<'tk>) -> &mut Id<'tk> {
        unsafe { self.parents.get_unchecked_mut(id.val as usize) }
    }

    pub fn add(&mut self) -> Id<'tk> {
        let id = Id {
            val: self.parents.len() as _,
            _tk: Default::default(),
        };
        self.parents.push(id);
        id
    }

    pub fn find(&mut self, id: Id<'tk>) -> Id<'tk> {
        if id != self.parent(id) {
            *self.parent_mut(id) = self.find(self.parent(id));
        }
        self.parent(id)
    }

    pub fn union(&mut self, cid1: Id<'tk>, cid2: Id<'tk>) -> Id<'tk> {
        *self.parent_mut(cid2) = cid1;
        cid1
    }
}
