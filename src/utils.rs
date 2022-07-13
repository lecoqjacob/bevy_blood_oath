use crate::prelude::*;

///////////////////////////////////////////////////////////////////////////////
/// Utility Functions
///////////////////////////////////////////////////////////////////////////////
use bevy::ecs::system::Resource;

/// Despawn all entities with a given component type
pub fn despawn_with<T: Component>(mut commands: Commands, q: Query<Entity, With<T>>) {
    for e in q.iter() {
        commands.entity(e).despawn_recursive();
    }
}

pub fn init_resource<R: Resource + FromWorld>(mut commands: Commands) {
    commands.init_resource::<R>();
}

pub fn remove_resource<R: Resource>(mut commands: Commands) {
    commands.remove_resource::<R>();
}

pub fn format_health(current: i32, max: i32) -> String {
    format!("HP: {} / {}", current, max)
}

///////////////////////////////////////////////////////////////////////////////
/// Bundle
///////////////////////////////////////////////////////////////////////////////

#[derive(Bundle)]
pub struct EntityBundle<EntityType: Component> {
    pub entity: EntityType,
    pub glyph: Glyph,
    pub position: Position,
}

pub macro spawn_entity {
    ($commands:ident, $ent:expr, $glyph:expr, $position:expr, $($component:expr),*) => {{
        let entity = $commands
            .spawn_bundle(EntityBundle {
                entity: $ent,
                glyph: $glyph,
                position: $position,
            })
            .id();

            // Start a repetition:
            $(
                $commands.entity(entity).insert($component);
            )*

            entity
    }},
}
