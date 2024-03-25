use std::collections::HashMap;

use slotmap::{SecondaryMap, SlotMap};

use crate::storage::{
    edge::{EdgeEnds, EdgeId, InternalEdgeID},
    node::{InternalNodeID, NodeId},
    traits::{EdgeObject, GraphStorage},
};

#[derive(Debug, Clone)]
pub struct Graph<N, E> {
    nodes: SlotMap<InternalNodeID, N>,
    edges: SlotMap<InternalEdgeID, E>,
    edge_ends: SecondaryMap<InternalEdgeID, EdgeEnds<Self>>,
    node_to_edges: HashMap<(InternalNodeID, InternalNodeID), InternalEdgeID>,
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

impl<N, E: EdgeObject> GraphStorage<N, E> for Graph<N, E> {
    fn add_node(&mut self, node: N) -> NodeId<Self> {
        let id = self.nodes.insert(node);
        NodeId::new(id)
    }

    fn update_node(&mut self, id: NodeId<Self>, node: N) {
        if let Some(old) = self.nodes.get_mut(id.get()) {
            *old = node;
        }
    }

    fn remove_node(&mut self, id: NodeId<Self>) -> Option<N> {
        self.nodes.remove(id.get())
    }

    fn node(&self, id: NodeId<Self>) -> Option<&N> {
        self.nodes.get(id.get())
    }

    fn nodes(&self) -> impl Iterator<Item = NodeId<Self>> {
        self.nodes.keys().map(NodeId::new)
    }

    fn incoming_nodes(&self, id: NodeId<Self>) -> impl Iterator<Item = NodeId<Self>> {
        self.incoming_nodes
            .get(id.get())
            .into_iter()
            .flatten()
            .copied()
            .map(NodeId::new)
    }

    fn outcoming_nodes(&self, id: NodeId<Self>) -> impl Iterator<Item = NodeId<Self>> {
        self.outgoing_nodes
            .get(id.get())
            .into_iter()
            .flatten()
            .copied()
            .map(NodeId::new)
    }

    fn contains_node(&self, id: NodeId<Self>) -> bool {
        self.nodes.contains_key(id.get())
    }

    fn has_edge_between(&self, from: NodeId<Self>, to: NodeId<Self>) -> bool {
        self.node_to_edges.contains_key(&(from.get(), to.get()))
    }

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
        self.node_to_edges.insert((from.get(), to.get()), id);

        Some(EdgeId::new(id))
    }

    fn update_edge(&mut self, id: EdgeId<Self>, edge: E) {
        if let Some(old) = self.edges.get_mut(id.get()) {
            *old = edge;
        }
    }

    fn remove_edge(&mut self, id: EdgeId<Self>) -> Option<E> {
        self.edges.remove(id.get())
    }

    fn edge(&self, id: EdgeId<Self>) -> Option<&E> {
        self.edges.get(id.get())
    }

    fn edges(&self) -> impl Iterator<Item = EdgeId<Self>> {
        self.edges.keys().map(EdgeId::new)
    }

    fn edge_between(&self, from: NodeId<Self>, to: NodeId<Self>) -> Option<EdgeId<Self>> {
        self.node_to_edges
            .get(&(from.get(), to.get()))
            .copied()
            .map(EdgeId::new)
    }

    fn edge_ends(&self, id: EdgeId<Self>) -> Option<&EdgeEnds<Self>> {
        self.edge_ends.get(id.get())
    }

    fn contains_edge(&self, id: EdgeId<Self>) -> bool {
        self.edges.contains_key(id.get())
    }

    fn edge_count(&self) -> usize {
        self.edges.len()
    }
}
