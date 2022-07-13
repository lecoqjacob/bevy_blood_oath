use crate::prelude::*;

mod end_turn;
mod fov;
mod movement;
mod player_input;
mod random_move;

mod render;

use bevy::app::PluginGroupBuilder;
use GameStage::*;
use TurnState::*;

struct TickingPlugin;
impl Plugin for TickingPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            ConditionSet::new()
                .run_if_resource_exists::<GameCamera>()
                .with_system(render::render_map)
                .with_system(render::render_glyphs)
                .into(),
        );
    }
}

struct WaitingForInputPlugin;
impl Plugin for WaitingForInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            ConditionSet::new()
                .run_if_resource_equals(WaitingForInput)
                .with_system(player_input::player_input_system)
                .with_system(fov::fov)
                .into(),
        );
    }
}

struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set_to_stage(
            PlayerStage,
            ConditionSet::new()
                .run_if_resource_equals(PlayerTurn)
                .with_system(movement::movement)
                .with_system(fov::fov)
                .with_system(end_turn::end_turn)
                .into(),
        );
    }
}

struct AIPlugin;
impl Plugin for AIPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set_to_stage(
            GenerateAIMoves,
            ConditionSet::new()
                .run_if_resource_equals(AITurn)
                .with_system(random_move::random_move)
                // .with_system(chasing::chasing)
                .into(),
        );

        app.add_system_set_to_stage(
            AIStage,
            ConditionSet::new()
                .run_if_resource_equals(AITurn)
                .with_system(movement::movement)
                .with_system(fov::fov)
                .with_system(end_turn::end_turn)
                .into(),
        );
    }
}

pub struct SystemsPlugins;
impl PluginGroup for SystemsPlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(TickingPlugin);
        group.add(WaitingForInputPlugin);
        group.add(PlayerPlugin);
        group.add(AIPlugin);
    }
}
