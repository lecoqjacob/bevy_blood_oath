#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]
#![feature(decl_macro)]
#![feature(auto_traits)]

#[macro_use]
extern crate lazy_static;

mod camera;
mod components;
mod event;
mod gamelog;
mod map;
mod resources;
mod rng;
mod setup;
mod systems;
mod utils;

mod prelude {
    pub use bevy::prelude::*;

    pub use bracket_bevy::prelude::*;
    pub use bracket_bevy::FontCharType;
    pub use bracket_noise::prelude::*;
    pub use bracket_pathfinding::prelude::*;
    pub use bracket_random::prelude::*;

    pub use iyes_loopless::prelude::*;

    pub use crate::camera::*;
    pub use crate::components::*;
    pub use crate::event::*;
    pub use crate::gamelog::*;
    pub use crate::map::*;
    pub use crate::resources::*;
    pub use crate::rng::*;
    pub use crate::setup::*;
    pub use crate::systems::*;
    pub use crate::utils::*;

    pub const MAPWIDTH: usize = 80;
    pub const MAPHEIGHT: usize = 43;
    pub const MAPCOUNT: usize = (MAPHEIGHT * MAPWIDTH) as usize;

    pub const LAYER_MAP: usize = 0;
    pub const LAYER_DECOR: usize = 1;
    pub const LAYER_ITEMS: usize = 2;
    pub const LAYER_CHR: usize = 3;
    pub const LAYER_TEXT: usize = 4;
}

pub use prelude::*;
use GameStage::*;

fn main() {
    let mut app = App::new();

    app.insert_resource(TurnState::Setup);

    // In Bevy, it's necessary to register the event types.
    app.add_event::<WantsToMove>()
        .add_event::<WantsToAttack>()
        .add_event::<SufferDamage>();

    app.add_stage_after(CoreStage::Update, PlayerStage, SystemStage::parallel())
        .add_stage_after(PlayerStage, GenerateAIMoves, SystemStage::parallel())
        .add_stage_after(GenerateAIMoves, AIStage, SystemStage::parallel());

    // Add Plugins
    app.add_plugins(DefaultPlugins)
        .add_plugin(BTermPlugin)
        // .add_plugin(MapPlugin)
        .add_plugin(SpawnerPlugin)
        .add_plugins(SystemsPlugins);

    // Setup Game
    app.add_system(exit_system).run();
}

fn exit_system(mut exit: EventWriter<bevy::app::AppExit>, input: Res<Input<KeyCode>>) {
    if input.any_pressed([KeyCode::Escape]) {
        exit.send(bevy::app::AppExit);
    }
}

struct BTermPlugin;
impl Plugin for BTermPlugin {
    fn build(&self, app: &mut App) {
        let builder = BTermBuilder::empty()
            .with_random_number_generator(true)
            .with_font("font-transparent.png", 16, 16, (8.0, 8.0)) // Load big font
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

        app.add_plugin(builder);
    }
}
