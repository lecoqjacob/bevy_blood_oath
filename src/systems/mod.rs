use crate::prelude::*;

mod camera;
mod damage;
mod fov;
mod map_indexing;
mod melee_combat;
mod monster_ai;
mod player_input;
mod render;

use GameStage::*;
use TurnState::*;

struct AwaitingInputPlugin;
impl Plugin for AwaitingInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(AwaitingInput)
                .with_system(player_input::player_input_system)
                .with_system(fov::fov)
                .into(),
        );
    }
}

struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(PlayerTurn)
                .with_system(fov::fov)
                .into(),
        );
    }
}

struct MonsterPlugin;
impl Plugin for MonsterPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(MonsterTurn)
                .with_system(fov::fov)
                .with_system(monster_ai::monster_ai)
                .into(),
        );
    }
}

pub struct SystemsPlugin;
impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set_to_stage(
            GameStage::Render,
            ConditionSet::new()
                .with_system(render::render_map)
                .with_system(render::render_entities)
                .with_system(render::render_ui)
                .with_system(render::render_tooltips)
                .into(),
        );

        app.add_system_set_to_stage(
            GameStage::CameraMove,
            ConditionSet::new()
                .with_system(camera::camera_follow)
                .into(),
        );

        // Every Turn Systems
        app.add_system_set(
            ConditionSet::new()
                .with_system(map_indexing::map_indexing)
                .with_system(melee_combat::combat)
                .with_system(damage::damage_system)
                .with_system(damage::delete_the_dead)
                .into(),
        );

        app.add_plugin(AwaitingInputPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(MonsterPlugin);
    }
}
