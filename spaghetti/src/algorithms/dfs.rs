use bevy::{ecs::entity::EntityHashSet, input::common_conditions::input_just_pressed, prelude::*};
use bevy_eventlistener::prelude::*;

use crate::{
    appstate::AppState,
    bevy_graph::{interaction::NodeSelected, nodes::NextNodes},
};

pub struct DFSRunnerPlugin;

impl Plugin for DFSRunnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            EventListenerPlugin::<Discovered>::default(),
            EventListenerPlugin::<Finished>::default(),
        ))
        .add_systems(Update, Self::run_dfs)
        .add_systems(
            Update,
            Self::log_dfs_run
                .run_if(in_state(AppState::DFS).and_then(input_just_pressed(KeyCode::Enter))),
        );
    }
}

#[derive(Debug, Clone, Event, EntityEvent)]
pub struct Discovered {
    #[target]
    dfs: Entity,
    node: Entity,
}

#[derive(Debug, Clone, Event, EntityEvent)]
pub struct Finished {
    #[target]
    dfs: Entity,
    node: Entity,
}

#[derive(Debug, Component)]
pub struct DFSStart(Entity);

#[derive(Bundle)]
pub struct DFSBundle {
    start_node: DFSStart,
    on_discover: On<Discovered>,
    on_finished: On<Finished>,
}

impl DFSRunnerPlugin {
    fn log_dfs_run(mut commands: Commands, selected: Query<Entity, With<NodeSelected>>) {
        if let Ok(selected) = selected.get_single() {
            info!("spawn dfs");
            commands.spawn(DFSBundle {
                start_node: DFSStart(selected),
                on_discover: On::<Discovered>::run(|listener: Listener<Discovered>| {
                    info!("discover: {node:?}", node = listener.node);
                }),
                on_finished: On::<Finished>::run(|listener: Listener<Finished>| {
                    info!("finish: {node:?}", node = listener.node);
                }),
            });
        }
    }

    fn run_dfs(
        mut commands: Commands,
        requests: Query<(Entity, &DFSStart)>,
        mut node_stack: Local<Vec<Entity>>,
        mut visited: Local<EntityHashSet>,
        next_nodes: Query<&NextNodes>,
        mut ev_discovered: EventWriter<Discovered>,
        mut ev_finished: EventWriter<Finished>,
    ) {
        requests.iter().for_each(|(dfs, DFSStart(start_node))| {
            info!("start dfs");
            *node_stack = vec![*start_node];
            visited.clear();
            while let Some(current_node) = node_stack.pop() {
                // mark current node as visited
                visited.insert(current_node);

                let new_next_nodes = next_nodes
                    .get(current_node)
                    .iter()
                    .flat_map(|nodes| nodes.iter())
                    .copied()
                    .filter(|node| !visited.contains(node))
                    .inspect(|node| {
                        ev_discovered.send(Discovered { dfs, node: *node });
                    })
                    .collect::<Vec<_>>();

                let finished = new_next_nodes.is_empty();

                if finished {
                    ev_finished.send(Finished {
                        dfs,
                        node: current_node,
                    });
                } else {
                    node_stack.extend(std::iter::once(current_node).chain(new_next_nodes));
                }
            }
            commands.entity(dfs).remove::<DFSStart>();
        });
    }
}
