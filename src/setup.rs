use crate::prelude::*;

fn setup(mut commands: Commands, mut state: ResMut<TurnState>) {
    let map = Map::new(&mut commands);

    let player_start = map.get_current().starting_point;

    commands.insert_resource(map);
    commands.insert_resource(GameCamera::new(player_start));

    crate::utils::spawn_entity!(
        commands,
        Player {},
        Glyph {
            glyph: to_cp437('@'),
            color: ColorPair::new(YELLOW, BLACK),
        },
        Position::with_pt(player_start, 0),
        Description("Everybody's favorite Bracket Corp SecBot".to_string(),),
        FieldOfView::new(8),
        Naming("SecBot".to_string())
    );

    // Transition to game
    *state = TurnState::WaitingForInput;
}

pub struct SpawnerPlugin;
impl Plugin for SpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}
