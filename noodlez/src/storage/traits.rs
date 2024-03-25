use crate::error::NoodlezError;

use super::{
    edge::{EdgeEnds, EdgeId},
    node::NodeId,
};

pub trait EdgeObject {
    fn weight(&self) -> f32;
}

pub trait GraphStorage<N, E: EdgeObject>
where
    Self: Sized,
{
    // node stuff
    fn add_node(&mut self, node: N) -> NodeId<Self>;
    fn update_node(&mut self, id: NodeId<Self>, node: N);
    fn remove_node(&mut self, id: NodeId<Self>);
    fn contains_node(&self, id: NodeId<Self>) -> bool;
    fn node(&self, id: NodeId<Self>) -> &N;
    fn node_count(&self) -> usize;
    fn has_edge_between(&self, from: NodeId<Self>, to: NodeId<Self>) -> bool;

    // edge stuff
    fn add_edge(
        &mut self,
        from: NodeId<Self>,
        to: NodeId<Self>,
        edge: E,
    ) -> Result<EdgeId<Self>, NoodlezError>;
    fn update_edge(&mut self, id: EdgeId<Self>, edge: E);
    fn remove_edge(&mut self, id: EdgeId<Self>);
    fn contains_edge(&mut self, id: EdgeId<Self>) -> bool;
    fn edge(&self, id: EdgeId<Self>) -> &E;
    fn edge_ends(&self, id: EdgeId<Self>) -> EdgeEnds<Self>;
    fn edge_count(&self) -> usize;
}
