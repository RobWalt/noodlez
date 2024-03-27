use bevy::{
    ecs::{query::QueryFilter, system::SystemParam},
    math::cubic_splines::CubicCurve,
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_prototype_lyon::prelude::*;

use super::{
    edges::{EdgeFromPosition, EdgeMidPosition, EdgeToPosition},
    nodes::BevyNode,
    RADIUS,
};

pub struct BevyGraphRenderPlugin;

impl Plugin for BevyGraphRenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                Self::attach_rendering_to_node,
                Self::attach_rendering_to_edge,
                Self::update_edge_rendering,
            ),
        );
    }
}

#[derive(SystemParam, Deref, DerefMut)]
pub struct EdgePositions<'w, 's, F: QueryFilter + 'static>(
    Query<
        'w,
        's,
        (
            Entity,
            (
                &'static EdgeFromPosition,
                &'static EdgeMidPosition,
                &'static EdgeToPosition,
            ),
        ),
        F,
    >,
);

impl<'w, 's, F: QueryFilter> EdgePositions<'w, 's, F> {
    pub fn iter_positions(&self) -> impl Iterator<Item = (Entity, [Vec2; 3])> + '_ {
        self.iter().map(
            |(entity, (EdgeFromPosition(from), EdgeMidPosition(mid), EdgeToPosition(to)))| {
                (entity, [*from, *mid, *to])
            },
        )
    }
    pub fn iter_curves(&self) -> impl Iterator<Item = (Entity, CubicCurve<Vec2>)> + '_ {
        self.iter_positions().map(|(entity, [from, mid, to])| {
            (
                entity,
                CubicBezier::new(vec![[from, mid, mid, to]]).to_curve(),
            )
        })
    }
}

impl BevyGraphRenderPlugin {
    fn attach_rendering_to_node(
        mut commands: Commands,
        mut mesh: ResMut<Assets<Mesh>>,
        mut material: ResMut<Assets<ColorMaterial>>,
        node_handles: Query<(&Handle<Mesh>, &Handle<ColorMaterial>), With<BevyNode>>,
        added_nodes: Query<(Entity, &BevyNode), (Added<BevyNode>, Without<Mesh2dHandle>)>,
    ) {
        let node_handle = node_handles.iter().next();
        let mesh: Mesh2dHandle = node_handle
            .map(|(m, _)| m)
            .cloned()
            .unwrap_or_else(|| mesh.add(Circle { radius: RADIUS }.mesh()))
            .into();
        let material = node_handle
            .map(|(_, m)| m)
            .cloned()
            .unwrap_or_else(|| material.add(Color::CRIMSON));
        added_nodes.iter().for_each(|(node, pos)| {
            commands.entity(node).insert(MaterialMesh2dBundle {
                mesh: mesh.clone(),
                material: material.clone(),
                transform: Transform::from_translation(pos.extend(2.0)),
                ..Default::default()
            });
        });
    }

    fn attach_rendering_to_edge(
        mut commands: Commands,
        positions: EdgePositions<Without<Mesh2dHandle>>,
    ) {
        positions.iter_curves().for_each(|(entity, curve)| {
            let shape_bundle = ShapeBundle {
                path: curve_to_path(curve),
                spatial: SpatialBundle::from_transform(Transform::from_translation(Vec3::Z * 0.5)),
                ..Default::default()
            };
            commands
                .entity(entity)
                .insert((shape_bundle, Stroke::new(Color::CRIMSON, 10.0)));
        })
    }

    fn update_edge_rendering(
        mut commands: Commands,
        positions: EdgePositions<
            Or<(
                Changed<EdgeFromPosition>,
                Changed<EdgeMidPosition>,
                Changed<EdgeToPosition>,
            )>,
        >,
    ) {
        positions.iter_curves().for_each(|(entity, curve)| {
            commands.entity(entity).insert(curve_to_path(curve));
        });
    }
}

fn curve_to_path(curve: CubicCurve<Vec2>) -> Path {
    let mut path = PathBuilder::new();
    let mut iter = curve.iter_positions(20);
    path.move_to(iter.next().unwrap_or_default());
    iter.for_each(|to| {
        path.line_to(to);
    });
    path.build()
}
