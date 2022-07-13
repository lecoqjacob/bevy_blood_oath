use crate::prelude::*;

mod gui;

mod render_glyphs;
mod render_map;
use render_glyphs::*;
use render_map::*;

pub struct RenderPlugin;
impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(gui::GUIPlugin);

        app.add_system_set(
            ConditionSet::new()
                .run_if_resource_exists::<GameCamera>()
                .with_system(render_map)
                .with_system(render_glyphs)
                .into(),
        );
    }
}
