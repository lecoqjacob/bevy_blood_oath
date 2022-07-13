use super::*;

pub fn render_tooltips(
    map: Res<Map>,
    ctx: Res<BracketContext>,
    c: Res<GameCamera>,
    entities: Query<(&Position, &Description, &Naming)>,
) {
    ctx.set_active_console(LAYER_MAP);
    let Point {
        x: mouse_x,
        y: mouse_y,
    } = ctx.get_mouse_position_for_current_layer();
    let map_pos = c.screen_to_world(mouse_x, mouse_y);

    let mut lines = Vec::new();
    for (pos, desc, name) in entities.iter() {
        if pos.layer == map.current_layer && pos.pt == map_pos {
            let idx = map.get_current().point2d_to_index(pos.pt);
            if map.get_current().visible[idx] {
                lines.push((CYAN, name.0.clone()));
                lines.push((GRAY, desc.0.clone()));
            }
        }
    }

    let mut batch = ctx.new_draw_batch();
    batch.target(LAYER_TEXT);

    if !lines.is_empty() {
        let height = lines.len() + 1;
        let width = lines.iter().map(|s| s.1.len()).max().unwrap() + 2;

        let tip_x = if map_pos.x < WIDTH as i32 / 2 {
            i32::min((mouse_x * 2) + 2, 111)
        } else {
            i32::max(0, (mouse_x * 2) - (width as i32 + 1))
        };

        let tip_y = if map_pos.y > HEIGHT as i32 / 2 {
            mouse_y - height as i32
        } else {
            mouse_y
        };

        batch.draw_box(
            Rect::with_size(
                tip_x,
                tip_y - (lines.len() / 2) as i32,
                width as i32,
                height as i32,
            ),
            ColorPair::new(WHITE, BLACK),
        );

        let mut y = tip_y + 1 - (lines.len() / 2) as i32;
        lines.iter().for_each(|s| {
            safe_print_color(
                &mut batch,
                Point::new(tip_x + 1, y),
                &s.1,
                ColorPair::new(s.0, BLACK),
            );
            y += 1;
        });
    }

    ctx.submit_batch(100_000, batch);
}
