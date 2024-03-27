pub mod dynamics;
pub mod edges;
pub mod interaction;
pub mod nodes;
pub mod rendering;
use bevy::prelude::*;

pub struct BevyGraphPlugin;

pub const RADIUS: f32 = 20.0;

impl Plugin for BevyGraphPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            rendering::BevyGraphRenderPlugin,
            interaction::BevyGraphInteractionPlugin,
            dynamics::BevyGraphDynamicsPlugin,
            nodes::BevyGraphNodePlugins,
            edges::BevyGraphEdgePlugin,
        ));
    }
}
