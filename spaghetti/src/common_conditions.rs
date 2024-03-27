use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::EguiContext;

pub fn is_hovering_ui(ctx: Query<&EguiContext>) -> bool {
    ctx.iter()
        .cloned()
        .any(|mut ctx| ctx.get_mut().is_pointer_over_area())
}
