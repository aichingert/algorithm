use bevy::prelude::*;

type Entities = [[Option<Entity>; super::TILES]; super::TILES];

#[derive(Resource)]
pub struct TileMap {
    pub entities: Entities,
}

impl TileMap {
    pub fn get_mut_entities(&mut self) -> &mut Entities {
        &mut self.entities
    }

    pub fn get_entity(&self, row: usize, col: usize) -> Entity {
        self.entities[row][col].unwrap()
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
