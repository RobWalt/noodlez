use std::marker::PhantomData;

use super::node::NodeId;

slotmap::new_key_type! {
    /// Id of edge objects
    pub struct InternalEdgeID;
}

#[derive(Debug)]
pub struct EdgeId<G> {
    id: InternalEdgeID,
    _pd: PhantomData<G>,
}

impl<G> Clone for EdgeId<G> {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            _pd: PhantomData,
        }
    }
}

impl<G> Copy for EdgeId<G> {}

impl<G> EdgeId<G> {
    #[inline]
    pub(crate) fn new(id: InternalEdgeID) -> Self {
        Self {
            id,
            _pd: Default::default(),
        }
    }

    #[inline]
    pub(crate) fn get(self) -> InternalEdgeID {
        self.id
    }
}

#[derive(Debug, Clone, Copy)]
pub struct EdgeEnds<G> {
    pub(crate) from: NodeId<G>,
    pub(crate) to: NodeId<G>,
}
