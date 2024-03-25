use slotmap::{SecondaryMap, SlotMap};

use super::{
    edge::{EdgeEnds, InternalEdgeID},
    node::InternalNodeID,
};

#[derive(Debug, Clone)]
pub struct Graph<N, E> {
    nodes: SlotMap<InternalNodeID, N>,
    edges: SlotMap<InternalEdgeID, E>,
    edge_ends: SecondaryMap<InternalEdgeID, EdgeEnds<Self>>,
    incoming_edges: SecondaryMap<InternalNodeID, Vec<InternalNodeID>>,
    outgoing_edges: SecondaryMap<InternalNodeID, Vec<InternalNodeID>>,
}

impl<N, E> Default for Graph<N, E> {
    fn default() -> Self {
        Self {
            nodes: SlotMap::with_key(),
            edges: SlotMap::with_key(),
            edge_ends: SecondaryMap::new(),
            incoming_edges: SecondaryMap::new(),
            outgoing_edges: SecondaryMap::new(),
        }
    }
}
