use crate::prelude::*;

pub fn render_glyphs(
    ctx: Res<BracketContext>,
    map: Res<Map>,
    c: Res<GameCamera>,
    glyphs: Query<(&Position, &Glyph)>,
) {
    let mut batch = ctx.new_draw_batch();
    batch.target(LAYER_CHR);
    batch.cls();

    glyphs.iter().for_each(|(pos, glyph)| {
        if pos.layer == map.current_layer {
            let idx = map.get_current().point2d_to_index(pos.pt);
            if map.get_current().visible[idx] {
                let screen_pos = c.world_to_screen(pos.pt);
                batch.set(screen_pos, glyph.color, glyph.glyph);
            }
        }
    });

    ctx.submit_batch(4000, batch);
}
