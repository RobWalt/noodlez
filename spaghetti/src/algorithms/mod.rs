use bevy::{ecs::system::SystemParam, prelude::*};
use bevy_eventlistener::prelude::*;

pub(crate) mod dfs;

pub struct BevyGraphAlgorithmPlugin;

impl Plugin for BevyGraphAlgorithmPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Start>()
            .register_type::<Discover>()
            .register_type::<Process>()
            .register_type::<Finish>()
            .register_type::<DirectedAlgorithm>()
            .register_type::<UndirectedAlgorithm>()
            .register_type::<TraverseAll>()
            .add_plugins((
                EventListenerPlugin::<Start>::default(),
                EventListenerPlugin::<Discover>::default(),
                EventListenerPlugin::<Process>::default(),
                EventListenerPlugin::<Finish>::default(),
            ))
            .add_plugins((dfs::DFSRunnerPlugin,));
    }
}

/// Event emitted once an algorithm starts
#[derive(Debug, Clone, Event, EntityEvent, Reflect)]
pub struct Start {
    #[target]
    dfs: Entity,
}

/// Event emitted once a node is first discovered by an algorithm, note that this is only the
/// time when it is put onto the recursion stack. You'll most likely want to do the "real work" on
/// [`Processed`]
#[derive(Debug, Clone, Event, EntityEvent, Reflect)]
pub struct Discover {
    #[target]
    dfs: Entity,
    node: Entity,
}

/// Event emitted once the node is processed. This is the time the node is most likely going to be
/// handled by your algorithm
#[derive(Debug, Clone, Event, EntityEvent, Reflect)]
pub struct Process {
    #[target]
    dfs: Entity,
    node: Entity,
}

/// Event emitted once an algorithm has finished up. Useful for cleanups since otherwise the
/// algorithm entity will stick around
#[derive(Debug, Clone, Event, EntityEvent, Reflect)]
pub struct Finish {
    #[target]
    dfs: Entity,
}

#[derive(SystemParam)]
pub struct AlgorithmEvents<'w> {
    pub start: EventWriter<'w, Start>,
    pub discover: EventWriter<'w, Discover>,
    pub process: EventWriter<'w, Process>,
    pub finish: EventWriter<'w, Finish>,
}

/// Component indicating that the algorithm is running on a directed graph
#[derive(Debug, Clone, Component, Default, Reflect)]
pub struct DirectedAlgorithm;

/// Component indicating that the algorithm is running on an indirect graph
#[derive(Debug, Clone, Component, Default, Reflect)]
pub struct UndirectedAlgorithm;

/// Component indicating that the algorithm should traverse the whole graph. This is especially
/// useful for graphs with disconnected components. If you, for example, make a poor choice for the
/// start nodes of a DFS, then it might happen that you don't traverse all the nodes you intended
/// to traverse
#[derive(Debug, Clone, Component, Default, Reflect)]
pub struct TraverseAll;
