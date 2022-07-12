mod resources;
mod map;
mod gamelog;

mod prelude {
    pub use bevy::prelude::*;
    pub use bracket_bevy::prelude::*;
    pub use bracket_pathfinding::prelude::*;
    pub use iyes_loopless::prelude::*;

    pub use bracket_bevy::FontCharType;

    pub use crate::resources::*;
    pub use crate::map::*;
    pub use crate::gamelog::*;

    pub const MAPWIDTH: usize = 80;
    pub const MAPHEIGHT: usize = 43;
    pub const MAPCOUNT: usize = (MAPHEIGHT * MAPWIDTH) as usize;
}

use prelude::*;

fn main() {
    let mut app = App::new();

    // Set the additional stages
    app.add_stage_after(CoreStage::Update, Render, SystemStage::parallel())
        .add_stage_after(Render, CameraMove, SystemStage::parallel());

        let bterm = BTermBuilder::empty()
        .with_random_number_generator(true)
        .with_font("font.png", 16, 16, (8.0, 8.0)) // Load big font
        .with_font("vga.png", 16, 16, (8.0, 16.0)) // Load easy-to-read font
        // Console 0: Base map
        .with_simple_console(0, 56, 31)
        .with_background(true)
        // Console 1: Decorations
        .with_sparse_console(0, 56, 31)
        .with_background(false)
        // Console 2: Items
        .with_sparse_console(0, 56, 31)
        .with_background(false)
        // Console 3: Characters
        .with_sparse_console(0, 56, 31)
        .with_background(false)
        // Console 4: User Interface
        .with_sparse_console(1, 112, 31)
        .with_background(true);

    // Add Plugins
    app.add_loopless_state(TurnState::Setup)
        .add_plugins(DefaultPlugins)
        .add_plugin(bterm)
        .add_plugin(SpawnerPlugin)
        .add_plugin(SystemsPlugin);

     // Setup Game
    app.add_startup_system(setup)
        .add_system(tick)
        .add_system(exit_system)
        .run();
}

fn setup(mut commands: Commands, rng: Res<RandomNumbers>) {
    let map = Map::new_map_rooms_and_corridors(&rng);
    commands.insert_resource(map);
}

fn tick(mut commands: Commands, turn_state: Res<CurrentState<TurnState>>) {
    match turn_state.0 {
        TurnState::Setup => commands.insert_resource(NextState(TurnState::AwaitingInput)),
        TurnState::PlayerTurn => commands.insert_resource(NextState(TurnState::MonsterTurn)),
        TurnState::MonsterTurn => commands.insert_resource(NextState(TurnState::AwaitingInput)),
        TurnState::AwaitingInput => {}
        TurnState::GameOver => {}
    }
}

fn exit_system(mut exit: EventWriter<bevy::app::AppExit>, input: Res<Input<KeyCode>>) {
    if input.just_pressed(KeyCode::Escape) {
        exit.send(bevy::app::AppExit);
    }
}