use super::*;

mod colony_info;
use colony_info::*;

mod queries;
use queries::*;

mod skeleton;
use skeleton::*;

mod tooltips;
use tooltips::*;

mod status;
use status::*;

pub fn safe_print_color<T: ToString>(batch: &mut DrawBatch, pos: Point, text: T, color: ColorPair) {
    let len = text.to_string().len();
    if pos.x > 0 && pos.y > 0 && len > 0 {
        //println!("Batched text[{}] at {:?}", text.to_string(), pos);
        batch.print_color(pos, text, color);
    }
}

pub fn render_gui(
    map: Res<Map>,
    ctx: Res<BracketContext>,
    status_q: Query<(Entity, &Colonist, &Position, &ColonistStatus)>,
) {
    let mut batch = ctx.new_draw_batch();
    batch.target(LAYER_TEXT);
    batch.cls();

    let status = PlayerStatus::query(map.current_layer, &status_q);
    render_panels(&mut batch);
    render_status(&mut batch, &status);
    render_colony_info(&mut batch, &status.colony);

    ctx.submit_batch(50_000, batch);
}

pub struct GUIPlugin;
impl Plugin for GUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            ConditionSet::new()
                .run_if_resource_exists::<GameCamera>()
                .with_system(render_tooltips)
                .with_system(render_gui)
                .into(),
        );
    }
}
