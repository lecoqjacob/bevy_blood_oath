use crate::prelude::*;

mod gui;

pub mod render_map;
pub use render_map::*;

pub use render_glyphs::*;
mod render_glyphs;

pub struct RenderPlugin;
impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(gui::GUIPlugin);
    }
}
