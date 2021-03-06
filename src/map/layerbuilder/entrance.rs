use super::*;

pub fn build_entrance(commands: &mut Commands) -> Layer {
    let mut layer = Layer::new(std::usize::MAX, commands); // Gets a default layer

    all_space(&mut layer);
    add_landscape(&mut layer);
    add_docking_capsule(&mut layer, commands);

    layer
}

fn add_docking_capsule(map: &mut Layer, commands: &mut Commands) {
    const MIDDLE: usize = HEIGHT / 2;
    const TOP: usize = MIDDLE - 3;
    const BOTTOM: usize = MIDDLE + 3;
    const LEFT: usize = 1;
    const RIGHT: usize = 8;

    // Floor
    for y in TOP..=BOTTOM {
        for x in LEFT..=RIGHT {
            let idx = map.point2d_to_index(Point::new(x, y));
            map.tiles[idx] = Tile::capsule_floor();
        }
    }

    // Encasing Walls
    for x in LEFT - 1..=RIGHT + 1 {
        let idx = map.point2d_to_index(Point::new(x, TOP - 1));
        map.tiles[idx] = Tile::capsule_wall();
        let idx = map.point2d_to_index(Point::new(x, BOTTOM + 1));
        map.tiles[idx] = Tile::capsule_wall();
    }
    for y in TOP - 1..=BOTTOM + 1 {
        let idx = map.point2d_to_index(Point::new(LEFT - 1, y));
        map.tiles[idx] = Tile::capsule_wall();
        let idx = map.point2d_to_index(Point::new(RIGHT + 1, y));
        map.tiles[idx] = Tile::capsule_wall();
    }

    // Add some windows
    let x_middle = (LEFT + RIGHT) / 2;
    let idx = map.point2d_to_index(Point::new(x_middle - 2, TOP - 1));
    map.tiles[idx] = Tile::capsule_window();
    let idx = map.point2d_to_index(Point::new(x_middle - 2, BOTTOM + 1));
    map.tiles[idx] = Tile::capsule_window();
    let idx = map.point2d_to_index(Point::new(x_middle + 2, TOP - 1));
    map.tiles[idx] = Tile::capsule_window();
    let idx = map.point2d_to_index(Point::new(x_middle + 2, BOTTOM + 1));
    map.tiles[idx] = Tile::capsule_window();

    // Window 1
    commands
        .spawn()
        .insert(Position::with_pt(Point::new(x_middle - 2, TOP - 1), 0))
        .insert(Description(
            "A window. It doesn't look fun outside.".to_string(),
        ));
    // Window 2
    commands
        .spawn()
        .insert(Position::with_pt(Point::new(x_middle - 2, BOTTOM + 1), 0))
        .insert(Description(
            "A window. It doesn't look fun outside.".to_string(),
        ));
    // Window 3
    commands
        .spawn()
        .insert(Position::with_pt(Point::new(x_middle + 2, TOP - 1), 0))
        .insert(Description(
            "A window. It doesn't look fun outside.".to_string(),
        ));
    // Window 4
    commands
        .spawn()
        .insert(Position::with_pt(Point::new(x_middle + 2, BOTTOM + 1), 0))
        .insert(Description(
            "A window. It doesn't look fun outside.".to_string(),
        ));

    // Start adding in building complex features
    add_door(map, commands, Point::new(RIGHT + 1, MIDDLE));

    // Spawn the game exit
    add_game_exit(map, commands, Point::new(LEFT - 1, MIDDLE));

    let start_room = add_entryway(map, commands, Point::new(RIGHT + 1, MIDDLE));
    let mut rooms = vec![start_room];
    while rooms.len() < 24 {
        try_random_room(map, commands, &mut rooms);
    }

    edge_filler(map);
    add_windows(map, commands);
    add_exit(&mut rooms, map, commands);

    // Populate rooms
    populate_rooms(&rooms, commands);

    map.starting_point = Point::new(LEFT + 1, MIDDLE);
}

fn add_game_exit(map: &mut Layer, commands: &mut Commands, pt: Point) {
    let exit_idx = map.point2d_to_index(pt);
    map.tiles[exit_idx] = Tile::game_over();

    commands
        .spawn()
        .insert(Position::with_pt(pt, 0))
        .insert(Description(
            "Exit to SecBot's Ship. Leave through here when you are ready to call it game over."
                .to_string(),
        ))
        .insert(TileTrigger(crate::components::TriggerType::EndGame));
}

fn add_landscape(map: &mut Layer) {
    let mut rng = crate::RNG.lock();
    let mut noise = FastNoise::seeded(rng.next_u64());

    noise.set_noise_type(NoiseType::SimplexFractal);
    noise.set_fractal_type(FractalType::FBM);
    noise.set_fractal_octaves(10);
    noise.set_fractal_gain(0.5);
    noise.set_fractal_lacunarity(3.5);
    noise.set_frequency(0.02);

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let h = noise.get_noise(x as f32, y as f32);
            let idx = map.point2d_to_index(Point::new(x, y));
            map.tiles[idx] = Tile::alien_landscape(h);
        }
    }
}

fn add_door(map: &mut Layer, commands: &mut Commands, pt: Point) {
    let idx = map.point2d_to_index(pt);

    commands
        .spawn()
        .insert(Position::with_pt(pt, 0))
        .insert(Description("A heavy, steel door.".to_string()))
        .insert(Glyph {
            glyph: to_cp437('+'),
            color: ColorPair::new(CYAN, BLACK),
        })
        .insert(Door);

    map.create_door(idx);
}

fn add_entryway(map: &mut Layer, _commands: &mut Commands, entrance: Point) -> Rect {
    let room = Rect::with_size(entrance.x + 1, entrance.y - 5, 20, 10);
    fill_room(map, &room);

    room
}

fn fill_room(map: &mut Layer, room: &Rect) {
    room.for_each(|pt| {
        if map.in_bounds(pt) {
            let idx = map.point2d_to_index(pt);
            map.tiles[idx] = Tile::floor();
        }
    });
    for x in i32::max(0, room.x1 - 1)..=i32::min(WIDTH as i32 - 1, room.x2 + 1) {
        try_wall(map, Point::new(x, room.y1 - 1));
        try_wall(map, Point::new(x, room.y2 + 1));
    }
    for y in i32::max(room.y1, 0)..=i32::min(room.y2, HEIGHT as i32 - 1) {
        try_wall(map, Point::new(room.x1 - 1, y));
        try_wall(map, Point::new(room.x2 + 1, y));
    }
}

fn try_wall(map: &mut Layer, pt: Point) {
    if map.in_bounds(pt) {
        let idx = map.point2d_to_index(pt);
        if !map.is_door[idx] {
            map.tiles[idx] = Tile::wall();
        }
    }
}

fn edge_filler(map: &mut Layer) {
    for y in 0..HEIGHT {
        let idx = map.point2d_to_index(Point::new(0, y));
        if map.tiles[idx].tile_type == TileType::Floor {
            map.tiles[idx] = Tile::wall();
        }
        let idx = map.point2d_to_index(Point::new(WIDTH - 1, y));
        if map.tiles[idx].tile_type == TileType::Floor {
            map.tiles[idx] = Tile::wall();
        }
    }
    for x in 0..WIDTH {
        let idx = map.point2d_to_index(Point::new(x, 0));
        if map.tiles[idx].tile_type == TileType::Floor {
            map.tiles[idx] = Tile::wall();
        }
        let idx = map.point2d_to_index(Point::new(x, HEIGHT - 1));
        if map.tiles[idx].tile_type == TileType::Floor {
            map.tiles[idx] = Tile::wall();
        }
    }
}

fn try_random_room(map: &mut Layer, commands: &mut Commands, rooms: &mut Vec<Rect>) {
    let mut rng = crate::RNG.lock();
    if let Some(parent_room) = rng.random_slice_entry(rooms) {
        let x;
        let y;
        let next_x;
        let next_y;

        // Decide where to consider an exit
        if rng.range(0, 2) == 0 {
            // Take from the horizontal walls
            x = parent_room.x1 + rng.range(0, parent_room.width() + 1);
            next_x = x;
            if rng.range(0, 2) == 0 {
                // Take from the north side
                y = parent_room.y1 - 1;
                next_y = y - 1;
            } else {
                // Take from the south side
                y = parent_room.y2 + 1;
                next_y = y + 1;
            }
        } else {
            // Take from the vertical walls
            y = parent_room.y1 + rng.range(0, parent_room.height() + 1);
            next_y = y;
            if rng.range(0, 2) == 0 {
                x = parent_room.x1 - 1;
                next_x = x - 1;
            } else {
                x = parent_room.x2 + 1;
                next_x = x + 1;
            }
        }
        let dx = next_x - x;
        let dy = next_y - y;

        // Try to place it
        let next_pt = Point::new(next_x, next_y);
        if !map.in_bounds(next_pt) {
            return;
        }
        let next_idx = map.point2d_to_index(next_pt);
        if map.tiles[next_idx].tile_type == TileType::Outside {
            let new_room = if dx == 1 {
                Rect::with_size(x + 1, y, rng.range(4, 10), rng.range(3, 6))
            } else if dy == 1 {
                Rect::with_size(x, next_y, rng.range(3, 6), rng.range(4, 10))
            } else if dx == -1 {
                let w = 5;
                Rect::with_size(x - w, y, rng.range(4, 10), rng.range(3, 6))
            } else {
                let h = 5;
                Rect::with_size(x, y - h, rng.range(3, 6), rng.range(4, 10))
            };

            let mut can_add = true;
            new_room.for_each(|p| {
                if map.in_bounds(p) {
                    let idx = map.point2d_to_index(p);
                    if map.tiles[idx].tile_type != TileType::Outside {
                        can_add = false;
                    }
                } else {
                    can_add = false;
                }
            });

            if can_add {
                add_door(map, commands, Point::new(x, y));
                fill_room(map, &new_room);
                rooms.push(new_room);
            }
        }
    }
}

fn add_windows(map: &mut Layer, commands: &mut Commands) {
    let mut rng = crate::RNG.lock();

    for y in 1..HEIGHT - 1 {
        for x in 1..WIDTH - 1 {
            let pt = Point::new(x, y);
            let idx = map.point2d_to_index(pt);
            if map.tiles[idx].tile_type == TileType::Wall
                && (map.tiles[idx - 1].tile_type == TileType::Outside
                    || map.tiles[idx + 1].tile_type == TileType::Outside
                    || map.tiles[idx - WIDTH].tile_type == TileType::Outside)
                && rng.range(0, 10) == 0
            {
                map.tiles[idx] = Tile::window();

                commands
                    .spawn()
                    .insert(Position::with_pt(Point::new(x, y), 0))
                    .insert(Description("Stairs further into the complex".to_string()));
            }
        }
    }
}
fn add_exit(rooms: &mut Vec<Rect>, map: &mut Layer, commands: &mut Commands) {
    let mut rng = crate::RNG.lock();
    let room = rng.random_slice_entry(rooms).unwrap();
    let exit_location = room.center();
    let idx = map.point2d_to_index(exit_location);
    map.tiles[idx] = Tile::stairs_down();

    commands
        .spawn()
        .insert(Position::with_pt(exit_location, 0))
        .insert(Description("Stairs further into the complex".to_string()));
}

fn populate_rooms(rooms: &[Rect], commands: &mut Commands) {
    let mut rng = crate::RNG.lock();

    // The first room always contains a single colonist
    spawn_random_colonist(commands, rooms[0].center(), 0);

    // Each room after that can be random. This is an initial, very boring spawn to get
    // the colonist functionality going.
    rooms.iter().skip(1).for_each(|r| {
        if rng.range(0, 5) == 0 {
            spawn_random_colonist(commands, r.center(), 0);
        }
    });
}
