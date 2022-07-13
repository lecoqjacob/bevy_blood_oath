use crate::prelude::*;
use bevy::ecs::schedule::ShouldRun;

mod gui;

pub mod render_map;
pub use render_map::*;

pub use render_glyphs::*;
mod render_glyphs;

pub struct RenderPlugin;
impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set_to_stage(
            GameStage::Render,
            ConditionSet::new()
                .run_if_resource_equals(TurnState::PlayerTurn)
                .with_system(render_map)
                .with_system(render_glyphs)
                .into(),
        );

        app.add_plugin(gui::GUIPlugin);
    }
}

fn run_if_state_ticking(state: Res<TurnState>) -> ShouldRun {
    if *state == TurnState::PlayerTurn {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}
