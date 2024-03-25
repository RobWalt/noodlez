use bevy::{
    input::{
        common_conditions::{input_just_pressed, input_pressed},
        mouse::MouseMotion,
    },
    prelude::*,
    sprite::MaterialMesh2dBundle,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_mod_picking::{selection::PickSelection, DefaultPickingPlugins, PickableBundle};
use noodlez::{
    prelude::*,
    storage::{edge::EdgeId, node::NodeId},
};

const RADIUS: f32 = 20.0;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            WorldInspectorPlugin::default(),
            DefaultPickingPlugins,
        ))
        .init_resource::<BevyGraph>()
        .add_systems(Startup, setup_camera_2d)
        .add_systems(Startup, setup_plane)
        .add_systems(Update, spawn_node.run_if(input_just_pressed(KeyCode::KeyN)))
        .add_systems(Update, move_nodes.run_if(any_are_near))
        .add_systems(Update, position_to_transform)
        .add_systems(Update, add_node_to_graph)
        .add_systems(Update, select_node)
        .add_systems(
            Update,
            apply_mouse_movements_to_nodes
                .run_if(input_pressed(KeyCode::Space).and_then(any_with_component::<NodeSelected>)),
        )
        .add_systems(
            Update,
            spawn_edge.run_if(
                not(input_pressed(KeyCode::ControlLeft))
                    .and_then(|q: Query<(), With<NodeSelected>>| q.iter().count() == 2),
            ),
        )
        .add_systems(Update, render_edges)
        .run();
}

#[derive(Debug, Resource, Deref, DerefMut, Default)]
pub struct BevyGraph(Graph<Vec2, ()>);

fn setup_camera_2d(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn setup_plane(
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

#[derive(Debug, Clone, Copy, Component, Deref, DerefMut)]
pub struct Node(Vec2);

fn get_mouse_position(window: &Window) -> Vec2 {
    window
        .cursor_position()
        .map_or_else(Default::default, |pos| {
            let width = window.resolution.width();
            let height = window.resolution.height();
            (pos + Vec2::new(-width, -height) * 0.5) * Vec2::new(1.0, -1.0)
        })
}

fn spawn_node(
    mut commands: Commands,
    mut mesh: ResMut<Assets<Mesh>>,
    mut material: ResMut<Assets<ColorMaterial>>,
    window: Query<&Window>,
    node_handles: Query<(&Handle<Mesh>, &Handle<ColorMaterial>), With<Node>>,
) {
    let window = window.single();
    let mouse_location = get_mouse_position(window);
    let node_handle = node_handles.iter().next();
    let mesh = node_handle
        .map(|(m, _)| m)
        .cloned()
        .unwrap_or_else(|| mesh.add(Circle { radius: RADIUS }.mesh()));
    let material = node_handle
        .map(|(_, m)| m)
        .cloned()
        .unwrap_or_else(|| material.add(Color::CRIMSON));

    commands.spawn((
        Node(mouse_location),
        MaterialMesh2dBundle {
            mesh: mesh.into(),
            material,
            transform: Transform::from_translation(mouse_location.extend(1.0)),
            ..Default::default()
        },
        PickableBundle::default(),
    ));
}

fn any_are_near(nodes: Query<&Node>) -> bool {
    nodes
        .iter_combinations()
        .any(|[a, b]| a.distance(**b) < RADIUS * 2.0 + 10.0)
}

fn move_nodes(mut nodes: Query<(&mut Node, Has<NodeSelected>)>) {
    let mut combs = nodes.iter_combinations_mut();
    while let Some([(mut a, sel_a), (mut b, sel_b)]) = combs.fetch_next() {
        if !(sel_a && sel_b) && a.distance(**b) < RADIUS * 2.0 + 10.0 {
            let delta = **a - **b;
            if !sel_a {
                **a += delta * 0.01;
            }
            if !sel_b {
                **b -= delta * 0.01;
            }
        }
    }
}

fn position_to_transform(mut nodes: Query<(&Node, &mut Transform), Changed<Node>>) {
    nodes.iter_mut().for_each(|(n, mut t)| {
        t.translation = n.extend(1.0);
    });
}

#[derive(Debug, Clone, Copy, Component)]
pub struct BevyNodeId(NodeId<Graph<Vec2, ()>>);

fn add_node_to_graph(
    mut commands: Commands,
    new_nodes: Query<(Entity, &Node), Added<Node>>,
    mut graph: ResMut<BevyGraph>,
) {
    new_nodes.iter().for_each(|(n, p)| {
        let id = graph.add_node(**p);
        commands.entity(n).insert((
            Name::new(format!("{id:?}")),
            BevyNodeId(id),
            ConnectedEdges::default(),
        ));
    })
}

#[derive(Debug, Component)]
pub struct NodeSelected;

fn select_node(
    mut commands: Commands,
    q_node: Query<(Entity, &PickSelection), (With<Node>, Changed<PickSelection>)>,
) {
    q_node.iter().for_each(|(n, s)| {
        if s.is_selected {
            commands.entity(n).insert(NodeSelected);
        } else {
            commands.entity(n).remove::<NodeSelected>();
        }
    })
}

fn apply_mouse_movements_to_nodes(
    mut nodes: Query<&mut Node, With<NodeSelected>>,
    mut mouse_motion: EventReader<MouseMotion>,
) {
    mouse_motion.read().for_each(|delta| {
        nodes.iter_mut().for_each(|mut node| {
            **node += delta.delta * Vec2::new(1.0, -1.0);
        });
    });
}

#[derive(Component, Deref, DerefMut)]
pub struct BevyEdgeId(EdgeId<Graph<Vec2, ()>>);
#[derive(Component, Deref, DerefMut)]
pub struct EdgeFrom(Entity);
#[derive(Component, Deref, DerefMut)]
pub struct EdgeTo(Entity);
#[derive(Component, Deref, DerefMut)]
pub struct EdgeMid(Vec2);

#[derive(Debug, Component, Default, Deref, DerefMut)]
pub struct ConnectedEdges(Vec<Entity>);

fn spawn_edge(
    mut clicked_node: Query<(Entity, &Node, &mut ConnectedEdges, &BevyNodeId), With<NodeSelected>>,
    mut commands: Commands,
    mut graph: ResMut<BevyGraph>,
) {
    let [(a, a_pos, mut a_edges, a_id), (b, b_pos, mut b_edges, b_id)] = clicked_node
        .iter_mut()
        .take(2)
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();

    let id = graph.add_edge(a_id.0, b_id.0, ()).unwrap();

    let e = commands
        .spawn((
            Name::new(format!("{id:?}")),
            BevyEdgeId(id),
            EdgeFrom(a),
            EdgeTo(b),
            EdgeMid((**a_pos + **b_pos) * 0.5),
        ))
        .id();

    a_edges.push(e);
    b_edges.push(e);

    commands.entity(a).remove::<NodeSelected>();
    commands.entity(b).remove::<NodeSelected>();
}

// fn render_edges(
//     mut gizmos: Gizmos,
//     nodes: Query<&Node>,
//     edges: Query<(&EdgeFrom, &EdgeMid, &EdgeTo)>,
// ) {
//     edges
//         .iter()
//         .for_each(|(EdgeFrom(a), EdgeMid(mid), EdgeTo(b))| {
//             let a = nodes.get(*a).unwrap().0;
//             let b = nodes.get(*b).unwrap().0;
//             gizmos.line_2d(a, b, Color::CRIMSON);
//         })
// }
