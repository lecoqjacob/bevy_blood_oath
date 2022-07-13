use crate::prelude::*;

pub fn spawn_random_colonist(commands: &mut Commands, location: Point, layer: usize) {
    crate::utils::spawn_entity!(
        commands,
        Colonist {},
        Glyph {
            glyph: to_cp437('☺'),
            color: ColorPair::new(LIME_GREEN, BLACK),
        },
        Position::with_pt(location, layer),
        Description("A squishy friend. You are here to rescue your squishies.".to_string(),),
        ColonistStatus::Alive
    );
}
