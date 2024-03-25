use std::fmt::Debug;
use std::marker::PhantomData;

slotmap::new_key_type! {
    /// Id of node objects
    pub struct InternalNodeID;
}

pub struct NodeId<G> {
    id: InternalNodeID,
    _pd: PhantomData<G>,
}

impl<G> NodeId<G> {
    #[inline]
    pub(crate) fn new(id: InternalNodeID) -> Self {
        Self {
            id,
            _pd: PhantomData,
        }
    }

    #[inline]
    pub(crate) fn get(self) -> InternalNodeID {
        self.id
    }
}

impl<G> Clone for NodeId<G> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<G> Copy for NodeId<G> {}

impl<G> Debug for NodeId<G> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[N {id:?}]", id = self.id.0)
    }
}

impl<G> PartialEq for NodeId<G> {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id)
    }
}

impl<G> Eq for NodeId<G> {}
