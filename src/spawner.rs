use crate::prelude::*;

pub fn spawn_player(mut commands: Commands, map: Res<Map>, mut state: ResMut<TurnState>) {
    let start = map.get_current().starting_point;

    crate::utils::spawn_entity!(
        commands,
        Player {},
        Glyph {
            glyph: to_cp437('@'),
            color: ColorPair::new(YELLOW, BLACK),
        },
        Position::with_pt(start, 0),
        Description("Everybody's favorite Bracket Corp SecBot".to_string(),),
        FieldOfView::new(8),
        Naming("SecBot".to_string())
    );

    commands.insert_resource(GameCamera::new(start));
}

pub struct SpawnerPlugin;
impl Plugin for SpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            ConditionSet::new()
                .run_if_resource_added::<TurnState>()
                .with_system(spawn_player)
                .into(),
        );
    }
}
