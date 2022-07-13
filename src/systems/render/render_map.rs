use crate::prelude::*;

pub fn render_map(ctx: Res<BracketContext>, map: Res<Map>, c: Res<GameCamera>) {
    let mut draw_batch = ctx.new_draw_batch();
    draw_batch.target(LAYER_MAP);
    draw_batch.cls();

    let layer = map.get_current();
    c.viewport.for_each(|pt| {
        let idx = layer.point2d_to_index(pt);
        if layer.in_bounds(pt) && layer.revealed[idx] {
            let t = &layer.tiles[idx];
            let mut color = t.color;

            if !layer.visible[idx] {
                color.fg = color.fg.to_greyscale();
            }

            draw_batch.set(c.world_to_screen(pt), t.color, t.glyph);
        }
    });

    ctx.submit_batch(0, draw_batch);
}
