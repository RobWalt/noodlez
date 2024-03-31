use bevy::prelude::*;

pub(crate) mod dfs;

pub struct BevyGraphAlgorithmPlugin;

impl Plugin for BevyGraphAlgorithmPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((dfs::DFSRunnerPlugin,));
    }
}
