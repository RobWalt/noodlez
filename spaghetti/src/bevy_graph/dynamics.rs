use bevy::prelude::*;

use super::{interaction::NodeSelected, nodes::BevyNode, RADIUS};

pub struct BevyGraphDynamicsPlugin;

impl Plugin for BevyGraphDynamicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                Self::move_nodes.run_if(Self::any_are_near),
                Self::position_to_transform,
            ),
        );
    }
}

impl BevyGraphDynamicsPlugin {
    fn any_are_near(nodes: Query<&BevyNode>) -> bool {
        nodes
            .iter_combinations()
            .any(|[a, b]| a.distance(**b) < RADIUS * 2.0 + 10.0)
    }

    fn move_nodes(mut nodes: Query<&mut BevyNode, Without<NodeSelected>>) {
        let mut combs = nodes.iter_combinations_mut();
        while let Some([mut a, mut b]) = combs.fetch_next() {
            if a.distance(**b) < RADIUS * 2.0 + 10.0 {
                let delta = **a - **b;
                **a += delta * 0.01;
                **b -= delta * 0.01;
            }
        }
    }

    fn position_to_transform(mut nodes: Query<(&BevyNode, &mut Transform), Changed<BevyNode>>) {
        nodes.iter_mut().for_each(|(n, mut t)| {
            t.translation = n.extend(1.0);
        });
    }
}
