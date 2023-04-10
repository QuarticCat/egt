use crate::{Id, Token};

#[derive(Debug, Default)]
pub struct UnionFind<'tk> {
    parents: Vec<Id<'tk>>,
}

impl<'tk> UnionFind<'tk> {
    pub fn make_set(&mut self) -> Id<'tk> {
        let id = Id {
            val: self.parents.len() as _,
            _token: Token::default(),
        };
        self.parents.push(id);
        id
    }

    pub fn size(&self) -> usize {
        self.parents.len()
    }

    fn parent(&self, id: Id<'tk>) -> Id<'tk> {
        unsafe { *self.parents.get_unchecked(id.val as usize) }
    }

    fn parent_mut(&mut self, id: Id<'tk>) -> &mut Id<'tk> {
        unsafe { self.parents.get_unchecked_mut(id.val as usize) }
    }

    pub fn find(&self, mut id: Id<'tk>) -> Id<'tk> {
        while id != self.parent(id) {
            id = self.parent(id)
        }
        id
    }

    pub fn find_mut(&mut self, mut id: Id<'tk>) -> Id<'tk> {
        if id != self.parent(id) {
            *self.parent_mut(id) = self.find_mut(self.parent(id));
        }
        self.parent(id)
    }

    pub fn union(&mut self, root1: Id<'tk>, root2: Id<'tk>) -> Id<'tk> {
        *self.parent_mut(root2) = root1;
        root1
    }
}
