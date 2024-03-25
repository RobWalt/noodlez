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
    pub fn new(id: InternalEdgeID) -> Self {
        Self {
            id,
            _pd: Default::default(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct EdgeEnds<G> {
    from: NodeId<G>,
    to: NodeId<G>,
}
