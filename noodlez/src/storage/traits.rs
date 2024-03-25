use super::{
    edge::{EdgeEnds, EdgeId},
    node::NodeId,
};

pub trait GraphStorage<N, E>
where
    Self: Sized,
{
    // === node stuff ===
    // operations
    fn add_node(&mut self, node: N) -> NodeId<Self>;
    fn update_node(&mut self, id: NodeId<Self>, node: N);
    fn remove_node(&mut self, id: NodeId<Self>) -> Option<N>;
    #[inline]
    fn disconnect_node(&mut self, id: NodeId<Self>) {
        self.neighbor_edges(id)
            .collect::<Vec<_>>()
            .into_iter()
            .for_each(|id| {
                self.remove_edge(id);
            });
    }
    // accessors
    fn node(&self, id: NodeId<Self>) -> Option<&N>;
    fn nodes(&self) -> impl Iterator<Item = NodeId<Self>>;
    fn incoming_nodes(&self, id: NodeId<Self>) -> impl Iterator<Item = NodeId<Self>>;
    fn outcoming_nodes(&self, id: NodeId<Self>) -> impl Iterator<Item = NodeId<Self>>;
    #[inline]
    fn neighbor_nodes(&self, id: NodeId<Self>) -> impl Iterator<Item = NodeId<Self>> {
        self.incoming_nodes(id).chain(self.outcoming_nodes(id))
    }
    // predicates
    fn contains_node(&self, id: NodeId<Self>) -> bool;
    fn has_edge_between(&self, edge_ends: EdgeEnds<Self>) -> bool;
    // properties
    fn node_count(&self) -> usize;

    // === edge stuff ===
    // operations
    fn add_edge(&mut self, from: NodeId<Self>, to: NodeId<Self>, edge: E) -> Option<EdgeId<Self>>;
    fn update_edge(&mut self, id: EdgeId<Self>, edge: E);
    fn remove_edge(&mut self, id: EdgeId<Self>) -> Option<E>;
    // accessors
    fn edge(&self, id: EdgeId<Self>) -> Option<&E>;
    fn edges(&self) -> impl Iterator<Item = EdgeId<Self>>;
    fn edge_between(&self, edge_ends: EdgeEnds<Self>) -> Option<EdgeId<Self>>;
    fn edge_ends(&self, id: EdgeId<Self>) -> Option<&EdgeEnds<Self>>;
    #[inline]
    fn incoming_edges(&self, id: NodeId<Self>) -> impl Iterator<Item = EdgeId<Self>> {
        self.incoming_nodes(id)
            .map(move |from| EdgeEnds { from, to: id })
            .filter_map(|ends| self.edge_between(ends))
    }
    #[inline]
    fn outcoming_edges(&self, id: NodeId<Self>) -> impl Iterator<Item = EdgeId<Self>> {
        self.outcoming_nodes(id)
            .map(move |to| EdgeEnds { from: id, to })
            .filter_map(|ends| self.edge_between(ends))
    }
    #[inline]
    fn neighbor_edges(&self, id: NodeId<Self>) -> impl Iterator<Item = EdgeId<Self>> {
        self.incoming_edges(id).chain(self.outcoming_edges(id))
    }
    // predicates
    fn contains_edge(&self, id: EdgeId<Self>) -> bool;
    // properties
    fn edge_count(&self) -> usize;
}
