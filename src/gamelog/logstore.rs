use super::LogFragment;
use crate::prelude::*;
use std::sync::Mutex;

lazy_static! {
    static ref LOG: Mutex<Vec<Vec<LogFragment>>> = Mutex::new(Vec::new());
}

pub fn append_entry(fragments: Vec<LogFragment>) {
    LOG.lock().unwrap().push(fragments);
}

pub fn clear_log() {
    LOG.lock().unwrap().clear();
}

pub fn print_log(ctx: &BracketContext, pos: Point) {
    let mut y = pos.y;
    let mut x = pos.x;

    LOG.lock().unwrap().iter().rev().take(5).for_each(|log| {
        log.iter().for_each(|frag| {
            ctx.print_color(
                x,
                y,
                &frag.text,
                frag.color.to_rgba(1.0),
                RGBA::named(BLACK),
            );
            x += frag.text.len() as i32;
            x += 1;
        });

        y += 1;
        x = pos.x;
    });
}

pub fn clone_log() -> Vec<Vec<crate::gamelog::LogFragment>> {
    LOG.lock().unwrap().clone()
}

pub fn restore_log(log: &mut Vec<Vec<crate::gamelog::LogFragment>>) {
    LOG.lock().unwrap().clear();
    LOG.lock().unwrap().append(log);
}
