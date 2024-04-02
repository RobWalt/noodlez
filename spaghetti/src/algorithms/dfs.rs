use bevy::{ecs::entity::EntityHashSet, input::common_conditions::input_just_pressed, prelude::*};
use bevy_eventlistener::prelude::*;

use crate::{
    appstate::AppState,
    bevy_graph::{
        interaction::NodeSelected,
        nodes::{NextNodes, PreviousNodes},
    },
};

use super::{
    AlgorithmEvents, DirectedAlgorithm, Discover, Finish, Process, Start, UndirectedAlgorithm,
};

pub struct DFSRunnerPlugin;

impl Plugin for DFSRunnerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<DFSStarts>()
            .add_systems(Update, (Self::run_directed_dfs, Self::run_undirected_dfs))
            .add_systems(
                Update,
                (
                    Self::init_directed_dfs.run_if(input_just_pressed(KeyCode::Numpad1)),
                    Self::init_undirected_dfs.run_if(input_just_pressed(KeyCode::Numpad2)),
                )
                    .run_if(in_state(AppState::DFS)),
            );
    }
}

/// sets the starting points of the DFS algorithms. Note that the algorithm will only traverse
/// all the nodes which are reachable by these starting points
#[derive(Debug, Component, Default, Reflect)]
pub struct DFSStarts(Vec<Entity>);

#[derive(Bundle)]
pub struct DFSBundle {
    name: Name,
    start_node: DFSStarts,
    on_start: On<Start>,
    on_discover: On<Discover>,
    on_processed: On<Process>,
    on_finish: On<Finish>,
}

impl Default for DFSBundle {
    fn default() -> Self {
        Self {
            name: Name::new("DFS"),
            start_node: DFSStarts::default(),
            on_start: On::run(|| {}),
            on_discover: On::run(|| {}),
            on_processed: On::run(|| {}),
            on_finish: On::run(|| {}),
        }
    }
}

#[derive(Bundle, Default)]
pub struct DirectedDFSBundle {
    dfs: DFSBundle,
    tag: DirectedAlgorithm,
}

#[derive(Bundle, Default)]
pub struct UndirectedDFSBundle {
    dfs: DFSBundle,
    tag: UndirectedAlgorithm,
}

impl DFSBundle {
    pub fn logging_with_starts(starts: Vec<Entity>) -> Self {
        DFSBundle {
            name: Name::new("DFS logging"),
            start_node: DFSStarts(starts),
            on_start: On::run(|listener: Listener<Start>| {
                info!("start: {node:?}", node = listener.listener());
            }),
            on_discover: On::run(|listener: Listener<Discover>| {
                info!("discover: {node:?}", node = listener.node);
            }),
            on_processed: On::run(|listener: Listener<Process>| {
                info!("process: {node:?}", node = listener.node);
            }),
            on_finish: On::run(|listener: Listener<Finish>| {
                info!("finish: {node:?}", node = listener.listener());
            }),
        }
    }
}

impl DFSRunnerPlugin {
    fn init_directed_dfs(mut commands: Commands, selected: Query<Entity, With<NodeSelected>>) {
        let starts = selected.iter().collect();
        commands.spawn(DirectedDFSBundle {
            dfs: DFSBundle::logging_with_starts(starts),
            ..Default::default()
        });
    }

    fn init_undirected_dfs(mut commands: Commands, selected: Query<Entity, With<NodeSelected>>) {
        let starts = selected.iter().collect();
        commands.spawn(UndirectedDFSBundle {
            dfs: DFSBundle::logging_with_starts(starts),
            ..Default::default()
        });
    }

    fn run_dfs(
        (dfs, DFSStarts(starts)): (Entity, &DFSStarts),
        commands: &mut Commands,
        node_stack: &mut Vec<Entity>,
        visited: &mut EntityHashSet,
        events: &mut AlgorithmEvents,
        get_next_fn: impl Fn(Entity, &EntityHashSet, &mut EventWriter<Discover>) -> Vec<Entity>,
    ) {
        // send signal indicating the algorithm has started
        events.start.send(Start { dfs });

        // set all starting nodes, read more in DFSStarts docs
        *node_stack = starts.clone();

        visited.clear();
        while let Some(current_node) = node_stack.pop() {
            // mark current node as visited
            visited.insert(current_node);

            let new_next_nodes = get_next_fn(current_node, &visited, &mut events.discover);

            let should_process = new_next_nodes.is_empty();

            if should_process {
                events.process.send(Process {
                    dfs,
                    node: current_node,
                });
            } else {
                node_stack.push(current_node);
                node_stack.extend(new_next_nodes);
            }
        }
        commands.entity(dfs).remove::<DFSStarts>();
        events.finish.send(Finish { dfs });
    }

    fn run_directed_dfs(
        mut commands: Commands,
        requests: Query<(Entity, &DFSStarts), With<DirectedAlgorithm>>,
        mut node_stack: Local<Vec<Entity>>,
        mut visited: Local<EntityHashSet>,
        next_nodes: Query<&NextNodes>,
        mut events: AlgorithmEvents,
    ) {
        requests.iter().for_each(|(dfs, starts)| {
            Self::run_dfs(
                (dfs, starts),
                &mut commands,
                &mut node_stack,
                &mut visited,
                &mut events,
                |current_node, visited, discover| {
                    next_nodes
                        .get(current_node)
                        .iter()
                        .flat_map(|nodes| nodes.iter())
                        .copied()
                        .filter(|node| !visited.contains(node))
                        .inspect(|node| {
                            discover.send(Discover { dfs, node: *node });
                        })
                        .collect::<Vec<_>>()
                },
            );
            commands.entity(dfs).remove::<DirectedAlgorithm>();
        });
    }

    fn run_undirected_dfs(
        mut commands: Commands,
        requests: Query<(Entity, &DFSStarts), With<UndirectedAlgorithm>>,
        mut node_stack: Local<Vec<Entity>>,
        mut visited: Local<EntityHashSet>,
        next_nodes: Query<(&NextNodes, &PreviousNodes)>,
        mut events: AlgorithmEvents,
    ) {
        requests.iter().for_each(|(dfs, starts)| {
            Self::run_dfs(
                (dfs, starts),
                &mut commands,
                &mut node_stack,
                &mut visited,
                &mut events,
                |current_node, visited, discover| {
                    next_nodes
                        .get(current_node)
                        .iter()
                        .flat_map(|(nexts, prevs)| nexts.iter().chain(prevs.iter()))
                        .copied()
                        .filter(|node| !visited.contains(node))
                        .inspect(|node| {
                            discover.send(Discover { dfs, node: *node });
                        })
                        .collect::<Vec<_>>()
                },
            );
            commands.entity(dfs).remove::<UndirectedAlgorithm>();
        });
    }
}
