use crate::prelude::*;

mod fov;
mod player_input;
mod random_move;

mod render;
use render::*;

struct AwaitingInputPlugin;
impl Plugin for AwaitingInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            ConditionSet::new()
                .run_if_resource_equals(TurnState::WaitingForInput)
                .with_system(player_input::player_input_system)
                .with_system(fov::fov)
                .into(),
        );
    }
}

struct TickingPlugin;
impl Plugin for TickingPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            ConditionSet::new()
                .run_if_resource_equals(TurnState::Ticking)
                .with_system(random_move::random_move)
                .with_system(fov::fov)
                .into(),
        );
    }
}

pub struct SystemsPlugin;
impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RenderPlugin);

        // Every Turn Systems
        app.add_system_set(
            ConditionSet::new()
                // .with_system(melee_combat::combat)
                // .with_system(damage::damage_system)
                // .with_system(damage::delete_the_dead)
                .into(),
        );

        app.add_plugin(AwaitingInputPlugin)
            .add_plugin(TickingPlugin);
    }
}
