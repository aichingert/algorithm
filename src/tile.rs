use bevy::prelude::*;

#[derive(Resource)]
pub struct TileMap {
    pub entities: [[Option<Entity>; super::TILES]; super::TILES],
}

pub enum TileState {
    Start,
    Block,
    Open,
    End,
}

#[derive(Component)]
pub struct Tile {
    state: TileState,
    position: (usize, usize),
}

impl Tile {
    pub fn new(position: (usize, usize), state: TileState) -> Self {
        Self { 
            position,
            state,
        }
    }
}
