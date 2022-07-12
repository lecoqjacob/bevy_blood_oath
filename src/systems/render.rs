use crate::prelude::*;

pub fn render_map(
    ctx: Res<BracketContext>,
    map: Res<Map>,
    player_q: Query<&FieldOfView, With<Player>>,
) {
    let mut draw_batch = ctx.new_draw_batch();
    draw_batch.target(0);
    draw_batch.cls();

    let player_fov = player_q.single();

    // Draw Map
    let mut y = 0;
    let mut x = 0;
    for (_idx, tile) in map.tiles.iter().enumerate() {
        let pt = Point::new(x, y);

        // Render a tile depending upon the tile type
        if map.revealed_tiles.contains(&pt) {
            let glyph = match tile {
                TileType::Floor => to_cp437('.'),
                TileType::Wall => to_cp437('#'),
            };

            let tint = if player_fov.visible_tiles.contains(&pt) {
                GREEN
            } else {
                DARK_GRAY
            };

            draw_batch.set(pt, ColorPair::new(tint, BLACK), glyph);
        }

        // Move the coordinates
        x += 1;
        if x > 79 {
            x = 0;
            y += 1;
        }
    }

    ctx.submit_batch(0, draw_batch);
}

pub fn render_entities(
    ctx: Res<BracketContext>,
    player_q: Query<&FieldOfView, With<Player>>,
    renderables_q: Query<(&PointC, &Renderable)>,
) {
    let mut draw_batch = ctx.new_draw_batch();
    draw_batch.target(0);

    let player_fov = player_q.single();

    //Draw Entities
    for (pt_c, render) in renderables_q.iter() {
        if player_fov.visible_tiles.contains(&pt_c.0) {
            draw_batch.set(
                pt_c.0,
                ColorPair::new(render.color.fg, render.color.bg),
                render.glyph,
            );
        }
    }

    ctx.submit_batch(0, draw_batch);
}

pub fn render_ui(ctx: Res<BracketContext>, player_stats_q: Query<&CombatStats, With<Player>>) {
    let mut draw_batch = ctx.new_draw_batch();
    draw_batch.target(0);

    let stats = player_stats_q.single();

    draw_batch.draw_box(Rect::with_size(0, 43, 79, 6), ColorPair::new(WHITE, BLACK));
    let health = format!(" HP: {} / {} ", stats.hp, stats.max_hp);
    draw_batch.print_color(Point::new(12, 43), &health, ColorPair::new(YELLOW, BLACK));

    draw_batch.bar_horizontal(
        Point::new(28, 43),
        51,
        stats.hp,
        stats.max_hp,
        ColorPair::new(RED, BLACK),
    );

    crate::gamelog::print_log(&ctx, Point::new(2, 44));

    ctx.submit_batch(0, draw_batch);
}

pub fn render_tooltips(
    ctx: Res<BracketContext>,
    map: Res<Map>,
    query: Query<(&PointC, &Naming)>,
    player_fov_q: Query<&FieldOfView, With<Player>>,
) {
    let mut draw_batch = ctx.new_draw_batch();
    draw_batch.target(0);

    let mouse_pos = ctx.get_mouse_position_for_current_layer();
    if mouse_pos.x >= map.width || mouse_pos.y >= map.height {
        return;
    }

    let mut tooltip: Vec<String> = Vec::new();
    let fov = player_fov_q.single();

    for (pt, name) in query.iter() {
        if pt.0.x == mouse_pos.x && pt.0.y == mouse_pos.y && fov.visible_tiles.contains(&pt.0) {
            tooltip.push(name.0.to_string());
        }
    }

    if !tooltip.is_empty() {
        let mut width: i32 = 0;

        for s in tooltip.iter() {
            if width < s.len() as i32 {
                width = s.len() as i32;
            }
        }

        width += 3;

        if mouse_pos.x > 40 {
            let arrow_pos = Point::new(mouse_pos.x - 2, mouse_pos.y);
            let left_x = mouse_pos.x - width;
            let mut y = mouse_pos.y;

            for s in tooltip.iter() {
                draw_batch.print_color(Point::new(left_x, y), s, ColorPair::new(YELLOW, BLACK));

                let padding = (width - s.len() as i32) - 1;
                for i in 0..padding {
                    draw_batch.print_color(
                        Point::new(arrow_pos.x - i, y),
                        &" ".to_string(),
                        ColorPair::new(YELLOW, BLACK),
                    );
                }

                y += 1;
            }

            draw_batch.print_color(
                Point::new(arrow_pos.x, arrow_pos.y),
                &"->".to_string(),
                ColorPair::new(YELLOW, BLACK),
            );
        } else {
            let arrow_pos = Point::new(mouse_pos.x + 1, mouse_pos.y);
            let left_x = mouse_pos.x + 3;
            let mut y = mouse_pos.y;

            for s in tooltip.iter() {
                draw_batch.print_color(Point::new(left_x + 1, y), s, ColorPair::new(YELLOW, BLACK));

                let padding = (width - s.len() as i32) - 1;
                for i in 0..padding {
                    draw_batch.print_color(
                        Point::new(arrow_pos.x + 1 + i, y),
                        &" ".to_string(),
                        ColorPair::new(YELLOW, BLACK),
                    );
                }

                y += 1;
            }

            draw_batch.print_color(
                Point::new(arrow_pos.x, arrow_pos.y),
                &"<-".to_string(),
                ColorPair::new(YELLOW, BLACK),
            );
        }
    }

    ctx.submit_batch(5000, draw_batch);
}
