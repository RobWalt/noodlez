use bevy::{
    input::{common_conditions::input_pressed, mouse::MouseMotion},
    prelude::*,
};
use bevy_mod_picking::{highlight::InitialHighlight, prelude::*};

use crate::appstate::AppState;

use super::nodes::BevyNode;

pub struct BevyGraphInteractionPlugin;

impl Plugin for BevyGraphInteractionPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((BevyGraphPickingPlugin, BevyGraphSelectionPlugin));
    }
}

pub struct BevyGraphPickingPlugin;

impl Plugin for BevyGraphPickingPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ShouldBePickableIn>()
            .add_systems(
                Update,
                (Self::add_pickable, Self::remove_pickable).run_if(state_changed::<AppState>),
            )
            .add_systems(Update, Self::states_for_nodes);
    }
}

#[derive(Debug, Clone, Component, Default, Reflect, Deref, DerefMut)]
struct ShouldBePickableIn(Vec<AppState>);

impl BevyGraphPickingPlugin {
    fn add_pickable(
        mut commands: Commands,
        pickables: Query<(Entity, &ShouldBePickableIn), Without<Pickable>>,
        state: Res<State<AppState>>,
    ) {
        pickables.iter().for_each(|(entity, states)| {
            if states.contains(state.get()) {
                commands.entity(entity).insert(PickableBundle::default());
            }
        });
    }

    fn remove_pickable(
        mut commands: Commands,
        pickables: Query<(Entity, &ShouldBePickableIn), With<Pickable>>,
        state: Res<State<AppState>>,
    ) {
        pickables.iter().for_each(|(entity, states)| {
            if !states.contains(state.get()) {
                commands
                    .entity(entity)
                    .remove::<(PickableBundle, InitialHighlight<ColorMaterial>)>();
            }
        });
    }

    fn states_for_nodes(mut commands: Commands, nodes: Query<Entity, Added<BevyNode>>) {
        nodes.iter().for_each(|node| {
            commands.entity(node).insert(ShouldBePickableIn(vec![
                AppState::SpawnEdges,
                AppState::Select,
                AppState::DFS,
            ]));
        });
    }
}

pub struct BevyGraphSelectionPlugin;

impl Plugin for BevyGraphSelectionPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<NodeSelected>().add_systems(
            Update,
            (
                Self::select_node,
                Self::apply_mouse_movements_to_nodes.run_if(
                    input_pressed(KeyCode::Space)
                        .and_then(any_with_component::<NodeSelected>)
                        .and_then(in_state(AppState::Select)),
                ),
            ),
        );
    }
}

#[derive(Debug, Component, Reflect)]
pub struct NodeSelected;

impl BevyGraphSelectionPlugin {
    fn select_node(
        mut commands: Commands,
        q_node: Query<(Entity, &PickSelection), (With<BevyNode>, Changed<PickSelection>)>,
        q_prev_selected: Query<Entity, With<NodeSelected>>,
    ) {
        q_node.iter().for_each(|(n, s)| {
            if s.is_selected {
                commands.entity(n).insert(NodeSelected);
            } else {
                commands.entity(n).remove::<NodeSelected>();
            }

            q_prev_selected.iter().for_each(|sel| {
                commands.entity(sel).remove::<NodeSelected>();
            })
        });
    }

    fn apply_mouse_movements_to_nodes(
        mut nodes: Query<&mut BevyNode, With<NodeSelected>>,
        mut mouse_motion: EventReader<MouseMotion>,
    ) {
        let delta: Vec2 = mouse_motion.read().map(|d| d.delta).sum();
        nodes.iter_mut().for_each(|mut node| {
            **node += delta * Vec2::new(1.0, -1.0);
        });
    }
}
