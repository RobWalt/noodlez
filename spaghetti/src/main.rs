pub(crate) mod appstate;
pub(crate) mod bevy_graph;
pub(crate) mod common_conditions;
pub(crate) mod external_graph;
pub(crate) mod setup;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            bevy_inspector_egui::quick::WorldInspectorPlugin::default(),
            bevy_mod_picking::DefaultPickingPlugins,
            bevy_prototype_lyon::prelude::ShapePlugin,
        ))
        .add_plugins((
            setup::SetupPlugin,
            appstate::AppStatePlugin,
            external_graph::ExternalGraphPlugin,
            bevy_graph::BevyGraphPlugin,
        ))
        .run();
}
