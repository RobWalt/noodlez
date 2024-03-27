use bevy::prelude::*;
use noodlez::storage::{
    edge::EdgeId, graph::slotmap_graph::Graph, node::NodeId, traits::GraphStorage,
};

use crate::bevy_graph::{
    edges::{EdgeFrom, EdgeTo},
    nodes::BevyNode,
};

pub struct ExternalGraphPlugin;

impl Plugin for ExternalGraphPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BevyGraph>()
            .add_systems(Update, (Self::add_node_to_graph, Self::add_edge_to_graph));
    }
}

type GraphType = Graph<Vec2, ()>;

#[derive(Debug, Resource, Deref, DerefMut, Default)]
pub struct BevyGraph(GraphType);

#[derive(Debug, Clone, Copy, Component)]
pub struct BevyNodeId(NodeId<GraphType>);

#[derive(Component, Deref, DerefMut)]
pub struct BevyEdgeId(EdgeId<GraphType>);

impl ExternalGraphPlugin {
    fn add_node_to_graph(
        mut commands: Commands,
        new_nodes: Query<(Entity, &BevyNode), Added<BevyNode>>,
        mut graph: ResMut<BevyGraph>,
    ) {
        new_nodes.iter().for_each(|(n, p)| {
            let id = graph.add_node(**p);
            commands
                .entity(n)
                .insert((Name::new(format!("{id:?}")), BevyNodeId(id)));
        })
    }

    fn add_edge_to_graph(
        mut commands: Commands,
        new_edges: Query<(Entity, &EdgeFrom, &EdgeTo), (Added<EdgeFrom>, Added<EdgeTo>)>,
        nodes: Query<&BevyNodeId>,
        mut graph: ResMut<BevyGraph>,
    ) {
        new_edges
            .iter()
            .for_each(|(edge, EdgeFrom(from), EdgeTo(to))| {
                if let Ok([from, to]) = nodes.get_many([*from, *to]) {
                    let id = graph.add_edge(from.0, to.0, ()).unwrap();
                    commands
                        .entity(edge)
                        .insert((Name::new(format!("{id:?}")), BevyEdgeId(id)));
                }
            })
    }
}
