use crate::prelude::*;

mod end_turn;
mod fov;
mod movement;
mod player_input;
mod random_move;

mod render;
use bevy::ecs::schedule::ShouldRun;

use GameStage::*;
use TurnState::*;

struct AwaitingInputPlugin;
impl Plugin for AwaitingInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::new()
                .with_run_criteria(run_if_state_waiting)
                .with_system(player_input::player_input_system.label("input"))
                .with_system(fov::fov.label("fov").after("input")),
        );

        app.add_system_set_to_stage(
            GameStage::Render,
            SystemSet::new()
                .with_run_criteria(run_if_state_waiting)
                .with_system(move_camera.exclusive_system().label("move_camera"))
                .with_system(render::render_map.label("map").after("move_camera"))
                .with_system(render::render_glyphs.label("glyphs").after("move_camera")),
        );
    }
}

struct TickingPlugin;
impl Plugin for TickingPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::new()
                .with_run_criteria(run_if_state_ticking)
                .with_system(random_move::random_move.label("random_move"))
                .with_system(fov::fov.label("fov").after("random_move")),
        );

        app.add_system_set_to_stage(
            GameStage::Render,
            SystemSet::new()
                .with_run_criteria(run_if_state_ticking)
                .with_system(move_camera.exclusive_system().label("move_camera"))
                .with_system(render::render_map.label("map").after("move_camera"))
                .with_system(render::render_glyphs.label("glyphs").after("move_camera")),
        );
    }
}

pub struct SystemsPlugin;
impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        // app.add_system_set(
        //     ConditionSet::new()
        //         .run_if_resource_exists::<GameCamera>()
        //         .with_system(render::render_map)
        //         .with_system(render::render_glyphs)
        //         .into(),
        // );

        // app.add_plugin(AwaitingInputPlugin)
        //     .add_plugin(TickingPlugin);
        // // .add_plugin(RenderPlugin);

        app.add_system_set(
            ConditionSet::new()
                .run_if_resource_added::<GameCamera>()
                .with_system(end_turn::end_turn)
                .into(),
        );

        app.add_system_set(
            ConditionSet::new()
                .run_if_resource_exists::<GameCamera>()
                .with_system(render::render_map)
                .with_system(render::render_glyphs)
                .into(),
        );

        app.add_system_set(
            ConditionSet::new()
                .run_if_resource_equals(WaitingForInput)
                .with_system(player_input::player_input_system)
                .with_system(fov::fov)
                .into(),
        );

        app.add_system_set_to_stage(
            PlayerCombat,
            ConditionSet::new()
                .run_if_resource_equals(PlayerTurn)
                // .with_system(combat::combat)
                .into(),
        );

        app.add_system_set_to_stage(
            MovePlayer,
            ConditionSet::new()
                .run_if_resource_equals(PlayerTurn)
                .with_system(movement::movement)
                .with_system(end_turn::end_turn)
                .into(),
        );

        app.add_system_set_to_stage(
            PlayerFov,
            ConditionSet::new()
                .run_if_resource_equals(PlayerTurn)
                .with_system(fov::fov)
                .into(),
        );

        app.add_system_set_to_stage(
            GenerateMonsterMoves,
            ConditionSet::new()
                .run_if_resource_equals(MonsterTurn)
                .with_system(random_move::random_move)
                // .with_system(chasing::chasing)
                .into(),
        );

        app.add_system_set_to_stage(
            MonsterCombat,
            ConditionSet::new()
                .run_if_resource_equals(MonsterTurn)
                // .with_system(combat::combat)
                .into(),
        );

        app.add_system_set_to_stage(
            MoveMonsters,
            ConditionSet::new()
                .run_if_resource_equals(MonsterTurn)
                .with_system(movement::movement)
                .with_system(end_turn::end_turn)
                .into(),
        );

        app.add_system_set_to_stage(
            MonsterFov,
            ConditionSet::new()
                .run_if_resource_equals(MonsterTurn)
                .with_system(fov::fov)
                .into(),
        );
    }
}

fn run_if_state_waiting(state: Res<TurnState>) -> ShouldRun {
    if *state == TurnState::WaitingForInput {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}

fn run_if_state_ticking(state: Res<TurnState>) -> ShouldRun {
    if *state == TurnState::PlayerTurn {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}

pub fn move_camera(mut c: ResMut<GameCamera>, p_q: Query<&Position, With<Player>>) {
    c.on_player_move(p_q.single().pt)
}
