use std::collections::HashMap;

use slotmap::{SecondaryMap, SlotMap};

use crate::storage::{
    edge::{EdgeEnds, EdgeId, InternalEdgeID},
    node::{InternalNodeID, NodeId},
    traits::GraphStorage,
};

#[derive(Debug, Clone)]
pub struct Graph<N, E> {
    nodes: SlotMap<InternalNodeID, N>,
    edges: SlotMap<InternalEdgeID, E>,
    edge_ends: SecondaryMap<InternalEdgeID, EdgeEnds<Self>>,
    node_to_edges: HashMap<EdgeEnds<Self>, InternalEdgeID>,
    incoming_nodes: SecondaryMap<InternalNodeID, Vec<InternalNodeID>>,
    outgoing_nodes: SecondaryMap<InternalNodeID, Vec<InternalNodeID>>,
}

impl<N, E> Default for Graph<N, E> {
    fn default() -> Self {
        Self {
            nodes: SlotMap::with_key(),
            edges: SlotMap::with_key(),
            edge_ends: SecondaryMap::new(),
            node_to_edges: HashMap::new(),
            incoming_nodes: SecondaryMap::new(),
            outgoing_nodes: SecondaryMap::new(),
        }
    }
}

impl<N, E> GraphStorage<N, E> for Graph<N, E> {
    #[inline]
    fn add_node(&mut self, node: N) -> NodeId<Self> {
        let id = self.nodes.insert(node);
        NodeId::new(id)
    }

    #[inline]
    fn update_node(&mut self, id: NodeId<Self>, node: N) {
        if let Some(old) = self.nodes.get_mut(id.get()) {
            *old = node;
        }
    }

    #[inline]
    fn remove_node(&mut self, id: NodeId<Self>) -> Option<N> {
        self.disconnect_node(id);
        self.nodes.remove(id.get())
    }

    #[inline]
    fn node(&self, id: NodeId<Self>) -> Option<&N> {
        self.nodes.get(id.get())
    }

    #[inline]
    fn nodes(&self) -> impl Iterator<Item = NodeId<Self>> {
        self.nodes.keys().map(NodeId::new)
    }

    #[inline]
    fn incoming_nodes(&self, id: NodeId<Self>) -> impl Iterator<Item = NodeId<Self>> {
        self.incoming_nodes
            .get(id.get())
            .into_iter()
            .flatten()
            .copied()
            .map(NodeId::new)
    }

    #[inline]
    fn outcoming_nodes(&self, id: NodeId<Self>) -> impl Iterator<Item = NodeId<Self>> {
        self.outgoing_nodes
            .get(id.get())
            .into_iter()
            .flatten()
            .copied()
            .map(NodeId::new)
    }

    #[inline]
    fn contains_node(&self, id: NodeId<Self>) -> bool {
        self.nodes.contains_key(id.get())
    }

    #[inline]
    fn has_edge_between(&self, ends: EdgeEnds<Self>) -> bool {
        self.node_to_edges.contains_key(&ends)
    }

    #[inline]
    fn node_count(&self) -> usize {
        self.nodes.len()
    }

    fn add_edge(&mut self, from: NodeId<Self>, to: NodeId<Self>, edge: E) -> Option<EdgeId<Self>> {
        if !(self.nodes.contains_key(from.get()) && self.nodes.contains_key(to.get())) {
            return None;
        }

        let out_n = self.outgoing_nodes.entry(from.get())?.or_default();
        let in_n = self.incoming_nodes.entry(to.get())?.or_default();

        out_n.push(to.get());
        in_n.push(from.get());

        let id = self.edges.insert(edge);
        let edge_ends = EdgeEnds { from, to };
        self.edge_ends.insert(id, edge_ends);
        self.node_to_edges.insert(edge_ends, id);

        Some(EdgeId::new(id))
    }

    #[inline]
    fn update_edge(&mut self, id: EdgeId<Self>, edge: E) {
        if let Some(old) = self.edges.get_mut(id.get()) {
            *old = edge;
        }
    }

    fn remove_edge(&mut self, id: EdgeId<Self>) -> Option<E> {
        if let Some(edge_ends @ EdgeEnds { from, to }) = self.edge_ends.get(id.get()) {
            if let Some(nodes) = self.incoming_nodes.get_mut(to.get()) {
                let id = from.get();
                nodes.retain(|e| e != &id);
            }

            if let Some(nodes) = self.outgoing_nodes.get_mut(from.get()) {
                let id = to.get();
                nodes.retain(|e| e != &id);
            }

            if let Some(id) = self.node_to_edges.remove(edge_ends) {
                self.edge_ends.remove(id);
            }
        }
        self.edges.remove(id.get())
    }

    #[inline]
    fn edge(&self, id: EdgeId<Self>) -> Option<&E> {
        self.edges.get(id.get())
    }

    #[inline]
    fn edges(&self) -> impl Iterator<Item = EdgeId<Self>> {
        self.edges.keys().map(EdgeId::new)
    }

    #[inline]
    fn edge_between(&self, edge_ends: EdgeEnds<Self>) -> Option<EdgeId<Self>> {
        self.node_to_edges.get(&edge_ends).copied().map(EdgeId::new)
    }

    #[inline]
    fn edge_ends(&self, id: EdgeId<Self>) -> Option<&EdgeEnds<Self>> {
        self.edge_ends.get(id.get())
    }

    #[inline]
    fn contains_edge(&self, id: EdgeId<Self>) -> bool {
        self.edges.contains_key(id.get())
    }

    #[inline]
    fn edge_count(&self) -> usize {
        self.edges.len()
    }
}
