use bracket_geometry::prelude::{Point, Rect};
use bracket_pathfinding::prelude::{Algorithm2D, BaseMap};

use crate::prelude::*;
use std::{
    cmp::{max, min},
    collections::{HashMap, HashSet},
};

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum TileType {
    Wall,
    Floor,
}

pub struct Map {
    pub rooms: Vec<Rect>,
    pub width: i32,
    pub height: i32,
    pub tiles: Vec<TileType>,
    pub blocked: HashSet<Point>,
    pub revealed_tiles: HashSet<Point>,
    pub tile_content: HashMap<Point, Vec<Entity>>,
}

impl Map {
    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        (y as usize * self.width as usize) + x as usize
    }

    pub fn pt_idx(&self, pt: Point) -> usize {
        self.xy_idx(pt.x, pt.y)
    }

    fn apply_room_to_map(&mut self, room: &Rect) {
        for y in room.y1 + 1..=room.y2 {
            for x in room.x1 + 1..=room.x2 {
                let idx = self.xy_idx(x, y);
                self.tiles[idx] = TileType::Floor;
            }
        }
    }

    fn apply_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        for x in min(x1, x2)..=max(x1, x2) {
            let idx = self.xy_idx(x, y);
            if idx > 0 && idx < self.width as usize * self.height as usize {
                self.tiles[idx as usize] = TileType::Floor;
            }
        }
    }

    fn apply_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        for y in min(y1, y2)..=max(y1, y2) {
            let idx = self.xy_idx(x, y);
            if idx > 0 && idx < self.width as usize * self.height as usize {
                self.tiles[idx as usize] = TileType::Floor;
            }
        }
    }

    pub fn populate_blocked(&mut self) {
        for (i, tile) in self.tiles.iter().enumerate() {
            let pt = self.index_to_point2d(i);
            match *tile {
                TileType::Floor => self.blocked.remove(&pt),
                TileType::Wall => self.blocked.insert(pt),
            };
        }
    }

    /// Makes a new map using the algorithm from http://rogueliketutorials.com/tutorials/tcod/part-3/
    /// This gives a handful of random rooms and corridors joining them together.
    pub fn new_map_rooms_and_corridors(rng: &RandomNumbers) -> Map {
        let mut map = Map {
            width: MAPWIDTH as i32,
            height: MAPHEIGHT as i32,
            rooms: Vec::new(),
            blocked: HashSet::new(),
            tile_content: HashMap::new(),
            revealed_tiles: HashSet::new(),
            tiles: vec![TileType::Wall; MAPCOUNT],
        };

        const MAX_ROOMS: i32 = 30;
        const MIN_SIZE: i32 = 6;
        const MAX_SIZE: i32 = 10;

        for _i in 0..MAX_ROOMS {
            let w = rng.range(MIN_SIZE, MAX_SIZE);
            let h = rng.range(MIN_SIZE, MAX_SIZE);
            let x = rng.roll_dice(1, map.width - w - 1) - 1;
            let y = rng.roll_dice(1, map.height - h - 1) - 1;
            let new_room = Rect::with_size(x, y, w, h);

            let mut ok = true;
            for other_room in map.rooms.iter() {
                if new_room.intersect(other_room) {
                    ok = false
                }
            }

            if ok {
                map.apply_room_to_map(&new_room);

                if !map.rooms.is_empty() {
                    let Point { x: new_x, y: new_y } = new_room.center();
                    let Point {
                        x: prev_x,
                        y: prev_y,
                    } = map.rooms[map.rooms.len() - 1].center();

                    if rng.range(0, 2) == 1 {
                        map.apply_horizontal_tunnel(prev_x, new_x, prev_y);
                        map.apply_vertical_tunnel(prev_y, new_y, new_x);
                    } else {
                        map.apply_vertical_tunnel(prev_y, new_y, prev_x);
                        map.apply_horizontal_tunnel(prev_x, new_x, new_y);
                    }
                }

                map.rooms.push(new_room);
            }
        }

        map
    }

    // checks if it is physically possible (ie no wall or physical object)
    pub fn can_enter_tile(&self, point: Point) -> bool {
        let idx = self.pt_idx(point);
        self.in_bounds(point)
            && self.tiles[idx] == TileType::Floor
            && !self.blocked.contains(&point)
    }

    pub fn clear_content_index(&mut self) {
        self.tile_content.clear();
    }

    fn valid_exit(&self, loc: Point, delta: Point) -> Option<usize> {
        let destination = loc + delta;

        if self.in_bounds(destination) {
            if self.can_enter_tile(destination) {
                let idx = self.point2d_to_index(destination);
                Some(idx)
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(self.width, self.height)
    }
}

#[rustfmt::skip]
impl BaseMap for Map {
    fn get_available_exits(&self, idx: usize) -> SmallVec<[(usize, f32); 10]> {
        let mut exits = SmallVec::new();
        let location = self.index_to_point2d(idx);

        // Cardinal Directions
        if let Some(idx) = self.valid_exit(location, Point::new(-1, 0)) { exits.push((idx, 1.0)) }
        if let Some(idx) = self.valid_exit(location, Point::new(1, 0)) { exits.push((idx, 1.0)) }
        if let Some(idx) = self.valid_exit(location, Point::new(0, -1)) { exits.push((idx, 1.0)) }
        if let Some(idx) = self.valid_exit(location, Point::new(0, 1)) { exits.push((idx, 1.0)) }

        // Diagonals
        if let Some(idx) = self.valid_exit(location, Point::new(-1, -1)) { exits.push((idx, 1.45)) }
        if let Some(idx) = self.valid_exit(location, Point::new(1, -1)) { exits.push((idx, 1.45)) }
        if let Some(idx) = self.valid_exit(location, Point::new(-1, 1)) { exits.push((idx, 1.45)) }
        if let Some(idx) = self.valid_exit(location, Point::new(1, 1)) { exits.push((idx, 1.45)) }

        exits
    }

    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        DistanceAlg::Pythagoras.distance2d(self.index_to_point2d(idx1), self.index_to_point2d(idx2))
    }

    fn is_opaque(&self, idx: usize) -> bool {
        self.tiles[idx] != TileType::Floor
    }
}
