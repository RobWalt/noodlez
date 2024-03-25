use std::marker::PhantomData;

slotmap::new_key_type! {
    /// Id of node objects
    pub struct InternalNodeID;
}

#[derive(Debug, Clone, Copy)]
pub struct NodeId<G> {
    id: InternalNodeID,
    _pd: PhantomData<G>,
}

impl<G> NodeId<G> {
    pub fn new(id: InternalNodeID) -> Self {
        Self {
            id,
            _pd: Default::default(),
        }
    }
}
