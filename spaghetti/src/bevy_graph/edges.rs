use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use bevy_mod_picking::pointer::PointerInteraction;

use crate::appstate::AppState;

use super::{
    interaction::NodeSelected,
    nodes::{BevyNode, IncomingEdges, OutgoingEdges},
};

pub struct BevyGraphEdgePlugin;

impl Plugin for BevyGraphEdgePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<EdgeFrom>()
            .register_type::<EdgeTo>()
            .register_type::<EdgeFromPosition>()
            .register_type::<EdgeMidPosition>()
            .register_type::<EdgeToPosition>()
            .add_systems(
                Update,
                (
                    Self::spawn_edge.run_if(
                        input_just_pressed(MouseButton::Left)
                            .and_then(in_state(AppState::SpawnEdges)),
                    ),
                    Self::register_outgoing_edge_connections,
                    Self::register_incoming_edge_connections,
                    Self::init_edge_from_position,
                    Self::init_edge_to_position,
                    Self::init_edge_mid_position,
                    Self::update_edges_from_positions,
                    Self::update_edges_to_positions,
                ),
            );
    }
}

#[derive(Component, Deref, DerefMut, Reflect)]
pub struct EdgeFrom(pub Entity);
#[derive(Component, Deref, DerefMut, Reflect)]
pub struct EdgeTo(pub Entity);

#[derive(Debug, Clone, Component, Deref, DerefMut, Default, Reflect)]
pub struct EdgeFromPosition(pub Vec2);
#[derive(Debug, Clone, Component, Deref, DerefMut, Default, Reflect)]
pub struct EdgeMidPosition(pub Vec2);
#[derive(Debug, Clone, Component, Deref, DerefMut, Default, Reflect)]
pub struct EdgeToPosition(pub Vec2);

impl BevyGraphEdgePlugin {
    fn spawn_edge(
        q_selected_node: Query<Entity, (With<NodeSelected>, With<BevyNode>)>,
        q_unselected_node: Query<Entity, (Without<NodeSelected>, With<BevyNode>)>,
        q_pointer: Query<&PointerInteraction>,
        mut commands: Commands,
    ) {
        if let Some((e, _hit)) = q_pointer.single().get_nearest_hit() {
            if let Ok(to) = q_unselected_node.get(*e) {
                if let Ok(from) = q_selected_node.get_single() {
                    commands.spawn((EdgeFrom(from), EdgeTo(to)));
                    commands.entity(from).remove::<NodeSelected>();
                    commands.entity(to).remove::<NodeSelected>();
                }
            }
        }
    }

    fn register_outgoing_edge_connections(
        edges: Query<(Entity, &EdgeFrom), Added<EdgeFrom>>,
        mut nodes: Query<&mut OutgoingEdges>,
    ) {
        edges.iter().for_each(|(edge, EdgeFrom(from))| {
            if let Ok(mut node) = nodes.get_mut(*from) {
                node.push(edge);
            }
        })
    }

    fn register_incoming_edge_connections(
        edges: Query<(Entity, &EdgeTo), Added<EdgeTo>>,
        mut nodes: Query<&mut IncomingEdges>,
    ) {
        edges.iter().for_each(|(edge, EdgeTo(to))| {
            if let Ok(mut node) = nodes.get_mut(*to) {
                node.push(edge);
            }
        })
    }

    fn init_edge_from_position(
        edges: Query<(Entity, &EdgeFrom), Added<EdgeFrom>>,
        nodes: Query<&BevyNode>,
        mut commands: Commands,
    ) {
        edges.iter().for_each(|(e, EdgeFrom(from))| {
            let from = nodes.get(*from).unwrap().0;
            commands.entity(e).insert(EdgeFromPosition(from));
        });
    }

    fn init_edge_to_position(
        edges: Query<(Entity, &EdgeTo), Added<EdgeTo>>,
        nodes: Query<&BevyNode>,
        mut commands: Commands,
    ) {
        edges.iter().for_each(|(e, EdgeTo(to))| {
            let to = nodes.get(*to).unwrap().0;
            commands.entity(e).insert(EdgeToPosition(to));
        });
    }

    fn init_edge_mid_position(
        edges: Query<(Entity, &EdgeFromPosition, &EdgeToPosition), Without<EdgeMidPosition>>,
        mut commands: Commands,
    ) {
        edges
            .iter()
            .for_each(|(e, EdgeFromPosition(from), EdgeToPosition(to))| {
                let mid = (*from + *to) * 0.5;
                commands.entity(e).insert(EdgeMidPosition(mid));
            });
    }

    fn update_edges_from_positions(
        changed_node: Query<(Entity, &BevyNode, &OutgoingEdges), Changed<BevyNode>>,
        mut edge_forms: Query<(&EdgeFrom, &mut EdgeFromPosition, &mut EdgeMidPosition)>,
    ) {
        changed_node.iter().for_each(|(node, new_pos, connected)| {
            connected.iter().copied().for_each(|e| {
                if let Some((_, mut edge_pos, mut edge_mid)) = edge_forms
                    .get_mut(e)
                    .ok()
                    .filter(|(EdgeFrom(e), _, _)| e == &node)
                {
                    let delta = **new_pos - **edge_pos;
                    **edge_pos = **new_pos;
                    **edge_mid += delta;
                }
            });
        });
    }

    fn update_edges_to_positions(
        changed_node: Query<(Entity, &BevyNode, &IncomingEdges), Changed<BevyNode>>,
        mut edge_tos: Query<(&EdgeTo, &mut EdgeToPosition, &mut EdgeMidPosition)>,
    ) {
        changed_node.iter().for_each(|(node, new_pos, connected)| {
            connected.iter().copied().for_each(|e| {
                if let Some((_, mut edge_pos, mut edge_mid)) = edge_tos
                    .get_mut(e)
                    .ok()
                    .filter(|(EdgeTo(e), _, _)| e == &node)
                {
                    let delta = **new_pos - **edge_pos;
                    **edge_pos = **new_pos;
                    **edge_mid += delta;
                }
            });
        });
    }
}
