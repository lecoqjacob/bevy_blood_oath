use crate::prelude::*;

pub fn damage_system(
    mut damage_events: EventReader<SufferDamage>,
    mut stats_q: Query<&mut CombatStats>,
) {
    for SufferDamage { amount, target } in damage_events.iter() {
        if let Ok(mut stats) = stats_q.get_mut(*target) {
            stats.hp -= amount;
        }
    }
}

pub fn delete_the_dead(
    mut commands: Commands,
    dead_query: Query<(Entity, &CombatStats, &Naming, Option<&Player>), Changed<CombatStats>>,
) {
    for (entity, stats, name, player) in dead_query.iter() {
        if stats.hp < 1 && player.is_none() {
            crate::gamelog::Logger::new()
                .append_with_color(&name.0, RED)
                .append("is dead!")
                .log();

            commands.entity(entity).despawn_recursive();
        }
    }
}
