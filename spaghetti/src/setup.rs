//! Just the basic setup

use crate::bevy_graph::{edges::*, nodes::*};
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (
                Self::setup_camera_2d,
                Self::setup_background,
                Self::setup_base_nodes,
            ),
        );
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

    fn setup_base_nodes(mut commands: Commands) {
        let [a, b, c, d] = {
            let commands = &mut commands;
            [Vec2::X, Vec2::Y, Vec2::NEG_X, Vec2::NEG_Y]
                .map(|p| p * 50.0)
                .map(move |pos| {
                    commands
                        .spawn((
                            BevyNode(pos),
                            OutgoingEdges::default(),
                            IncomingEdges::default(),
                            NextNodes::default(),
                            PreviousNodes::default(),
                        ))
                        .id()
                })
        };
        let [ab, bc, cd] = {
            let commands = &mut commands;
            [(a, b), (b, c), (c, d)]
                .map(|(from, to)| commands.spawn((EdgeFrom(from), EdgeTo(to))).id())
        };
    }
}
