use super::*;

pub struct PlayerStatus {
    // pub current_hp: i32,
    // pub max_hp: i32,
    pub property_damage: i32,
    pub human_resources: i32,
    pub colony: ColonyInfo,
    // pub target: TargetInfo,
}

pub struct ColonyInfo {
    pub total_colonists: i32,
    pub colonists_on_layer: i32,
    pub located_alive: i32,
    pub located_dead: i32,
    pub died_in_rescue: i32,
    pub rescued: i32,
}

pub struct TargetInfo {
    pub target: Option<Entity>,
    pub color: Option<RGBA>,
    pub name: Option<String>,
    pub point: Option<Point>,
    pub probability: Option<u32>,
    pub range: Option<u32>,
}

impl PlayerStatus {
    pub fn query(
        map_layer: usize,
        status_q: &Query<(Entity, &Colonist, &Position, &ColonistStatus)>,
    ) -> Self {
        let colony = PlayerStatus::colony_calculator(status_q, map_layer);
        //     // let (current_hp, max_hp) = PlayerStatus::health(ecs);
        // let property_damage = PlayerStatus::property_damage(ecs);
        // let human_resources = PlayerStatus::human_resources(&colony, property_damage);
        //     // let target = PlayerStatus::targeting_info(ecs);

        Self {
            // current_hp,
            // max_hp,
            property_damage: 0,
            human_resources: 0,
            colony,
            // target,
        }
    }

    // fn property_damage(ecs: &World) -> i32 {
    //     <(&PropertyValue, &Position)>::query()
    //         .filter(!component::<Health>())
    //         .iter(ecs)
    //         .map(|(v, _)| v.0)
    //         .sum()
    // }

    fn colony_calculator(
        status_q: &Query<(Entity, &Colonist, &Position, &ColonistStatus)>,
        current_layer: usize,
    ) -> ColonyInfo {
        let mut total_colonists = 0;
        let mut colonists_on_layer = 0;
        let mut located_alive = 0;
        let mut located_dead = 0;
        let mut died_in_rescue = 0;
        let mut rescued = 0;

        for (entity, colonist, pos, status) in status_q.iter() {
            if *status != ColonistStatus::StartedDead {
                total_colonists += 1;
            }
            if pos.layer == current_layer
                && *status != ColonistStatus::Rescued
                && *status != ColonistStatus::DiedAfterStart
                && *status != ColonistStatus::StartedDead
            {
                colonists_on_layer += 1;
            }
            // if let Ok(entry) = ecs.entry_ref(*entity) {
            //     if let Ok(_) = entry.get_component::<Found>() {
            //         match *status {
            //             ColonistStatus::Alive => located_alive += 1,
            //             ColonistStatus::StartedDead => located_dead += 1,
            //             ColonistStatus::DiedAfterStart => died_in_rescue += 1,
            //             ColonistStatus::Rescued => rescued += 1,
            //         }
            //     }
            // }
        }

        ColonyInfo {
            total_colonists,
            colonists_on_layer,
            located_alive,
            located_dead,
            died_in_rescue,
            rescued,
        }
    }

    fn human_resources(colony: &ColonyInfo, property_damage: i32) -> i32 {
        let mut human_resources = 50;

        // Pay for what you break
        human_resources -= property_damage / 1000;

        // Colonist status
        human_resources += colony.rescued * 3;
        human_resources -= colony.located_dead;
        human_resources -= colony.died_in_rescue;
        human_resources += colony.located_alive;

        human_resources
    }
}
