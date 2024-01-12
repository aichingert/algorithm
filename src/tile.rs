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

#[derive(Resource)]
pub struct State(pub TileState);

impl State {
    pub fn is_same(&self, txt: &str) -> bool {
        match self.0 {
            TileState::Start => txt == "Start",
            TileState::Block => txt == "Block",
            TileState::Open  => txt == "Open",
            TileState::End   => txt == "End",
        }
    }

    pub fn set(&mut self, txt: &str) {
        self.0 = match txt {
            "Start" => TileState::Start,
            "Block" => TileState::Block,
            "Open" => TileState::Open,
            "End" => TileState::End,
            _ => unreachable!(),
        };
    }

    pub fn get_color(&self) -> Color {
        match self.0 {
            TileState::Start => Color::GREEN,
            TileState::Block => Color::BLACK,
            TileState::Open  => Color::WHITE,
            TileState::End   => Color::RED,
        }
    }
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
