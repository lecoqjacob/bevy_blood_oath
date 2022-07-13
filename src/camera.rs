use crate::prelude::*;

pub struct GameCamera {
    pub player_pos: Point,
    pub viewport: Rect,
}

impl GameCamera {
    pub fn new(player_pos: Point) -> Self {
        let viewport = Rect::with_size(player_pos.x - 20, player_pos.y - 15, 40, 31);

        Self {
            player_pos,
            viewport,
        }
    }

    pub fn on_player_move(&mut self, player_position: Point) {
        self.player_pos = player_position;
        self.viewport = Rect::with_size(self.player_pos.x - 20, self.player_pos.y - 15, 40, 31);
    }

    pub fn world_to_screen(&self, pt: Point) -> Point {
        let bot = pt - self.player_pos;
        bot + Point::new(20, 15)
    }

    pub fn world_to_screen_text(&self, pt: Point) -> Point {
        let ws = self.world_to_screen(pt);
        ws * Point::new(2, 1)
    }

    pub fn screen_to_world(&self, mouse_x: i32, mouse_y: i32) -> Point {
        Point::new(mouse_x + self.viewport.x1, mouse_y + self.viewport.y1)
    }

    pub fn render_map(&self, map: &Map, ctx: &BracketContext) {
        let mut batch = ctx.new_draw_batch();
        batch.target(LAYER_MAP);
        batch.cls();

        let layer = map.get_current();
        self.viewport.for_each(|pt| {
            if layer.in_bounds(pt) {
                let idx = layer.point2d_to_index(pt);

                if layer.visible[idx] {
                    let t = &layer.tiles[idx];
                    batch.set(self.world_to_screen(pt), t.color, t.glyph);
                } else if layer.revealed[idx] {
                    let t = &layer.tiles[idx];
                    batch.set(
                        self.world_to_screen(pt),
                        ColorPair::new(t.color.fg.to_greyscale(), BLACK),
                        t.glyph,
                    );
                }
            }
        });

        ctx.submit_batch(0, batch);
    }

    pub fn render_glyphs(
        &self,
        map: &Map,
        ctx: &BracketContext,
        glyphs: Query<(&Position, &Glyph)>,
    ) {
        let mut batch = ctx.new_draw_batch();
        batch.target(LAYER_CHR);
        batch.cls();

        for (pos, glyph) in glyphs.iter() {
            if pos.layer == map.current_layer {
                let idx = map.get_current().point2d_to_index(pos.pt);
                if map.get_current().visible[idx] {
                    let screen_pos = self.world_to_screen(pos.pt);
                    batch.set(screen_pos, glyph.color, glyph.glyph);
                }
            }
        }

        ctx.submit_batch(4000, batch);
    }
}
