//! Just the basic setup

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (Self::setup_camera_2d, Self::setup_background));
    }
}

impl SetupPlugin {
    fn setup_camera_2d(mut commands: Commands) {
        commands.spawn(Camera2dBundle::default());
    }

    fn setup_background(
        mut commands: Commands,
        mut mesh: ResMut<Assets<Mesh>>,
        mut material: ResMut<Assets<ColorMaterial>>,
    ) {
        commands.spawn(MaterialMesh2dBundle {
            mesh: mesh
                .add(
                    Rectangle {
                        half_size: Vec2::ONE * 10000.0,
                    }
                    .mesh(),
                )
                .into(),
            material: material.add(Color::BISQUE),
            ..Default::default()
        });
    }
}
