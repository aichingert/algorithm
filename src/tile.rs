use bevy::prelude::*;

type Entities = [[Option<Entity>; super::COLS]; super::ROWS];

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

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub enum TileState {
    Visited,
    Solve,
    Start,
    Block,
    Open,
    End,
}

#[derive(Resource)]
pub struct State(pub TileState);

impl State {
    pub fn is_drawable(&self) -> bool {
        self.0 != TileState::Solve
    }

    pub fn is_same(&self, txt: &str) -> bool {
        txt == match self.0 {
            TileState::Visited => "Visited",
            TileState::Start => "Start",
            TileState::Block => "Block",
            TileState::Solve => "Solve",
            TileState::Open  => "Open",
            TileState::End   => "End",
        }
    }

    pub fn set(&mut self, txt: &str) {
        self.0 = match txt {
            "Visited" => TileState::Visited,
            "Solve" => TileState::Solve,
            "Start" => TileState::Start,
            "Block" => TileState::Block,
            "Open" => TileState::Open,
            "End" => TileState::End,
            _ => unreachable!(),
        };
    }

    pub fn get_color(&self) -> Color {
        match self.0 {
            TileState::Visited => Color::YELLOW,
            TileState::Start => Color::GREEN,
            TileState::Block => Color::BLACK,
            TileState::Open  => Color::WHITE,
            TileState::End   => Color::RED,
            _ => unreachable!(),
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
