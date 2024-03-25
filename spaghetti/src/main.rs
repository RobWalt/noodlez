use bevy::{
    input::{
        common_conditions::{input_just_pressed, input_pressed},
        mouse::MouseMotion,
    },
    prelude::*,
    sprite::MaterialMesh2dBundle,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_mod_picking::{
    picking_core::Pickable, pointer::PointerInteraction, selection::PickSelection,
    DefaultPickingPlugins, PickableBundle,
};
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
        .add_systems(
            Update,
            spawn_node.run_if(
                in_state(AppState::SpawnNodes).and_then(input_just_pressed(MouseButton::Left)),
            ),
        )
        .add_systems(Update, move_nodes.run_if(any_are_near))
        .add_systems(Update, position_to_transform)
        .add_systems(Update, add_node_to_graph)
        .add_systems(Update, select_node)
        .add_systems(Update, change_app_state)
        .add_systems(
            Update,
            apply_mouse_movements_to_nodes
                .run_if(input_pressed(KeyCode::Space).and_then(any_with_component::<NodeSelected>)),
        )
        .init_state::<AppState>()
        .add_systems(
            Update,
            (
                add_pickable.run_if(in_state(AppState::SpawnEdges)),
                add_pickable.run_if(in_state(AppState::Select)),
                remove_pickable.run_if(in_state(AppState::SpawnNodes)),
            )
                .run_if(state_changed::<AppState>),
        )
        .add_systems(
            Update,
            (
                spawn_edge.run_if(input_just_pressed(MouseButton::Left)),
                register_edge_in_nodes,
            )
                .run_if(in_state(AppState::SpawnEdges)),
        )
        .add_systems(
            Update,
            (
                init_edge_curve,
                render_edges,
                update_edges_from_positions,
                update_edges_to_positions,
            ),
        )
        .run();
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, States)]
pub enum AppState {
    #[default]
    Select,
    SpawnNodes,
    SpawnEdges,
}

impl AppState {
    pub fn from_keys(key: KeyCode) -> Option<Self> {
        match key {
            KeyCode::KeyN => Some(Self::SpawnNodes),
            KeyCode::KeyE => Some(Self::SpawnEdges),
            KeyCode::KeyS => Some(Self::Select),
            _ => None,
        }
    }
}

fn change_app_state(
    keys: Res<ButtonInput<KeyCode>>,
    state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if let Some(next) = keys
        .get_just_pressed()
        .filter_map(|key| AppState::from_keys(*key))
        .filter(|new_state| state.get() != new_state)
        .next()
    {
        next_state.set(next);
    }
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

#[derive(Debug, Clone, Component, Default, Reflect)]
pub struct ShouldBePickable;

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
        ConnectedEdges::default(),
        ShouldBePickable,
    ));
}

fn add_pickable(
    mut commands: Commands,
    pickables: Query<Entity, (With<ShouldBePickable>, Without<Pickable>)>,
) {
    pickables.iter().for_each(|e| {
        commands.entity(e).insert(PickableBundle::default());
    });
}

fn remove_pickable(
    mut commands: Commands,
    pickables: Query<Entity, (With<ShouldBePickable>, With<Pickable>)>,
) {
    pickables.iter().for_each(|e| {
        commands.entity(e).remove::<PickableBundle>();
    });
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
        commands
            .entity(n)
            .insert((Name::new(format!("{id:?}")), BevyNodeId(id)));
    })
}

#[derive(Debug, Component)]
pub struct NodeSelected;

fn select_node(
    mut commands: Commands,
    q_node: Query<(Entity, &PickSelection), (With<Node>, Changed<PickSelection>)>,
    q_prev_selected: Query<Entity, With<NodeSelected>>,
) {
    q_node.iter().for_each(|(n, s)| {
        if s.is_selected {
            commands.entity(n).insert(NodeSelected);
        } else {
            commands.entity(n).remove::<NodeSelected>();
        }

        q_prev_selected.iter().for_each(|sel| {
            commands.entity(sel).remove::<NodeSelected>();
        })
    });
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
    q_selected_node: Query<(Entity, &BevyNodeId), With<NodeSelected>>,
    q_unselected_node: Query<(Entity, &BevyNodeId), Without<NodeSelected>>,
    q_pointer: Query<&PointerInteraction>,
    mut commands: Commands,
    mut graph: ResMut<BevyGraph>,
) {
    if let Some((e, _hit)) = q_pointer.single().get_nearest_hit() {
        if let Ok((to, to_id)) = q_unselected_node.get(*e) {
            if let Ok((from, from_id)) = q_selected_node.get_single() {
                let id = graph.add_edge(from_id.0, to_id.0, ()).unwrap();

                commands.spawn((
                    Name::new(format!("{id:?}")),
                    BevyEdgeId(id),
                    EdgeFrom(from),
                    EdgeTo(to),
                ));

                commands.entity(from).remove::<NodeSelected>();
                commands.entity(to).remove::<NodeSelected>();
            }
        }
    }
}

fn register_edge_in_nodes(
    edges: Query<(Entity, &EdgeFrom, &EdgeTo), Or<(Added<EdgeFrom>, Added<EdgeTo>)>>,
    mut nodes: Query<&mut ConnectedEdges>,
) {
    edges.iter().for_each(|(edge, EdgeFrom(from), EdgeTo(to))| {
        if let Ok(mut node) = nodes.get_mut(*from) {
            node.push(edge);
        }
        if let Ok(mut node) = nodes.get_mut(*to) {
            node.push(edge);
        }
    })
}

#[derive(Debug, Clone, Component, Deref, DerefMut, Default, Reflect)]
pub struct EdgeCurveFrom(Vec2);
#[derive(Debug, Clone, Component, Deref, DerefMut, Default, Reflect)]
pub struct EdgeCurveMid(Vec2);
#[derive(Debug, Clone, Component, Deref, DerefMut, Default, Reflect)]
pub struct EdgeCurveTo(Vec2);

fn init_edge_curve(
    edges: Query<(Entity, &EdgeFrom, &EdgeTo), Or<(Added<EdgeFrom>, Added<EdgeTo>)>>,
    nodes: Query<&Node>,
    mut commands: Commands,
) {
    edges.iter().for_each(|(e, EdgeFrom(from), EdgeTo(to))| {
        let from = nodes.get(*from).unwrap().0;
        let to = nodes.get(*to).unwrap().0;
        let mid = (from + to) * 0.5;
        commands
            .entity(e)
            .insert((EdgeCurveFrom(from), EdgeCurveMid(mid), EdgeCurveTo(to)));
    });
}

fn update_edges_from_positions(
    changed_node: Query<(Entity, &Node, &ConnectedEdges), Changed<Node>>,
    mut edge_forms: Query<(&EdgeFrom, &mut EdgeCurveFrom)>,
) {
    changed_node.iter().for_each(|(node, new_pos, connected)| {
        connected.iter().copied().for_each(|e| {
            if let Some((_, mut edge_pos)) = edge_forms
                .get_mut(e)
                .ok()
                .filter(|(EdgeFrom(e), _)| e == &node)
            {
                **edge_pos = **new_pos;
            }
        });
    });
}

fn update_edges_to_positions(
    changed_node: Query<(Entity, &Node, &ConnectedEdges), Changed<Node>>,
    mut edge_tos: Query<(&EdgeTo, &mut EdgeCurveTo)>,
) {
    changed_node.iter().for_each(|(node, new_pos, connected)| {
        connected.iter().copied().for_each(|e| {
            if let Some((_, mut edge_pos)) =
                edge_tos.get_mut(e).ok().filter(|(EdgeTo(e), _)| e == &node)
            {
                **edge_pos = **new_pos;
            }
        });
    });
}

fn render_edges(mut gizmos: Gizmos, edges: Query<(&EdgeCurveFrom, &EdgeCurveMid, &EdgeCurveTo)>) {
    edges.iter().for_each(
        |(EdgeCurveFrom(from), EdgeCurveMid(mid), EdgeCurveTo(to))| {
            let curve = CubicBezier::new(vec![[*from, *mid, *mid, *to]]).to_curve();
            gizmos.linestrip_2d(curve.iter_positions(10), Color::CRIMSON);
        },
    )
}
