use crate::prelude::*;

pub fn map_indexing(mut map: ResMut<Map>, blockers: Query<(Entity, &PointC, Option<&BlocksTile>)>) {
    map.populate_blocked();
    map.clear_content_index();

    for (entity, pos, blocker) in blockers.iter() {
        if blocker.is_some() {
            map.blocked.insert(pos.0);
        }

        // Push the entity to the appropriate index slot. It's a Copy
        // type, so we don't need to clone it (we want to avoid moving it out of the ECS!)
        if let Some(content) = map.tile_content.get_mut(&pos.0) {
            content.push(entity);
        } else {
            map.tile_content.insert(pos.0, vec![entity]);
        }
    }
}
