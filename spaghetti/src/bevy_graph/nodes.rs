use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use crate::{appstate::AppState, common_conditions::is_hovering_ui};

pub struct BevyGraphNodePlugins;

impl Plugin for BevyGraphNodePlugins {
    fn build(&self, app: &mut App) {
        app.register_type::<BevyNode>()
            .register_type::<OutgoingEdges>()
            .register_type::<IncomingEdges>()
            .register_type::<NextNodes>()
            .register_type::<PreviousNodes>()
            .add_systems(
                Update,
                Self::spawn_node.run_if(
                    in_state(AppState::SpawnNodes)
                        .and_then(input_just_pressed(MouseButton::Left))
                        .and_then(not(is_hovering_ui)),
                ),
            );
    }
}

#[derive(Debug, Clone, Copy, Component, Deref, DerefMut, Reflect)]
pub struct BevyNode(pub Vec2);

#[derive(Debug, Component, Default, Deref, DerefMut, Reflect)]
pub struct OutgoingEdges(Vec<Entity>);

#[derive(Debug, Component, Default, Deref, DerefMut, Reflect)]
pub struct IncomingEdges(Vec<Entity>);

#[derive(Debug, Component, Deref, DerefMut, Default, Reflect)]
pub struct NextNodes(Vec<Entity>);

#[derive(Debug, Component, Deref, DerefMut, Default, Reflect)]
pub struct PreviousNodes(Vec<Entity>);

impl BevyGraphNodePlugins {
    fn spawn_node(mut commands: Commands, window: Query<&Window>) {
        let window = window.single();
        let mouse_location = get_mouse_position(window);

        commands.spawn((
            BevyNode(mouse_location),
            OutgoingEdges::default(),
            IncomingEdges::default(),
            NextNodes::default(),
            PreviousNodes::default(),
        ));
    }
}

fn get_mouse_position(window: &Window) -> Vec2 {
    window
        .cursor_position()
        .map_or_else(Default::default, |pos| {
            let width = window.resolution.width();
            let height = window.resolution.height();
            (pos + Vec2::new(-width, -height) * 0.5) * Vec2::new(1.0, -1.0)
        })
}
