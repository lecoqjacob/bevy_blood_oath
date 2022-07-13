use super::*;

#[derive(Clone)]
pub struct Map {
    pub current_layer: usize,
    layers: Vec<Layer>,
}

impl Map {
    pub fn new(commands: &mut Commands) -> Self {
        let mut layers = Vec::with_capacity(NUM_LAYERS);
        for i in 0..NUM_LAYERS {
            layers.push(Layer::new(i, commands));
        }

        Self {
            current_layer: 0, // TODO: Set me back
            layers,
        }
    }

    pub fn get_current(&self) -> &Layer {
        &self.layers[self.current_layer]
    }

    pub fn get_current_mut(&mut self) -> &mut Layer {
        &mut self.layers[self.current_layer]
    }
}

pub fn setup_map(mut commands: Commands) {
    let map = Map::new(&mut commands);
    commands.insert_resource(map);
}

pub struct MapPlugin;
impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_map);
    }
}
