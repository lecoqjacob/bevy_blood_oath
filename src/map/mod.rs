use crate::prelude::*;

pub const WIDTH: usize = 80;
pub const HEIGHT: usize = 60;
const TILES: usize = WIDTH * HEIGHT;
pub const NUM_LAYERS: usize = 5;

mod tile;
use tile::{Tile, TileType};

mod layer;
use layer::Layer;

mod map;
pub use map::*;

mod layerbuilder;
