use std::marker::PhantomData;

slotmap::new_key_type! {
    /// Id of node objects
    pub struct InternalNodeID;
}

#[derive(Debug)]
pub struct NodeId<G> {
    id: InternalNodeID,
    _pd: PhantomData<G>,
}

impl<G> Clone for NodeId<G> {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            _pd: PhantomData,
        }
    }
}

impl<G> Copy for NodeId<G> {}

impl<G> NodeId<G> {
    #[inline]
    pub(crate) fn new(id: InternalNodeID) -> Self {
        Self {
            id,
            _pd: Default::default(),
        }
    }

    #[inline]
    pub(crate) fn get(self) -> InternalNodeID {
        self.id
    }
}
