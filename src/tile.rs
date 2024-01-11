use bevy::prelude::*;
use std::ops::{Index, IndexMut};

#[derive(Resource)]
pub struct TileMap {
    pub entities: [[Option<Entity>; super::TILES]; super::TILES],
}

impl Index<(usize, usize)> for TileMap {
    type Output = Option<Entity>;

    fn index(&self, idx: (usize, usize)) -> &Self::Output {
        &self.entities[idx.0][idx.1]
    }
}

impl IndexMut<(usize, usize)> for TileMap {
    fn index_mut(&mut self, idx: (usize, usize)) -> &mut Self::Output {
        &mut self.entities[idx.0][idx.1]
    }
}

pub enum TileState {
    Start,
    Block,
    Open,
    End,
}

#[derive(Component)]
pub struct Tile {
    pub state: TileState,
    pub position: (usize, usize),
}

impl Tile {
    pub fn new(position: (usize, usize), state: TileState) -> Self {
        Self { 
            position,
            state,
        }
    }
}
