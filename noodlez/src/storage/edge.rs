use core::hash::Hash;
use std::fmt::Debug;
use std::marker::PhantomData;

use super::node::NodeId;

slotmap::new_key_type! {
    /// Id of edge objects
    pub struct InternalEdgeID;
}

pub struct EdgeId<G> {
    id: InternalEdgeID,
    _pd: PhantomData<G>,
}

impl<G> EdgeId<G> {
    #[inline]
    pub(crate) fn new(id: InternalEdgeID) -> Self {
        Self {
            id,
            _pd: PhantomData,
        }
    }

    #[inline]
    pub(crate) fn get(self) -> InternalEdgeID {
        self.id
    }
}

impl<G> Clone for EdgeId<G> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<G> Copy for EdgeId<G> {}

impl<G> Debug for EdgeId<G> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[E {id:?}]", id = self.id.0)
    }
}

impl<G> PartialEq for EdgeId<G> {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id)
    }
}

impl<G> Eq for EdgeId<G> {}

pub struct EdgeEnds<G> {
    pub(crate) from: NodeId<G>,
    pub(crate) to: NodeId<G>,
}

impl<G> Clone for EdgeEnds<G> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<G> Copy for EdgeEnds<G> {}

impl<G> Debug for EdgeEnds<G> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{from:?} -> {to:?}]", from = self.from, to = self.to)
    }
}

impl<G> Hash for EdgeEnds<G> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.from.get().hash(state);
        self.to.get().hash(state);
    }
}

impl<G> PartialEq for EdgeEnds<G> {
    fn eq(&self, other: &Self) -> bool {
        self.from == other.from && self.to == other.to
    }
}

impl<G> Eq for EdgeEnds<G> {}
