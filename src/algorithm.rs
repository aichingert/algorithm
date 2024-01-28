use bevy::prelude::*;

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};

#[derive(Eq, PartialEq, Clone, Copy, Hash, Debug)]
pub struct Coord {
    pub x: usize,
    pub y: usize,
    // TODO: find a way to not store the destination in the coord
    pub dst: (usize, usize), 
}

impl Coord {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y, dst: (0, 0), }
    }

    pub fn new_dst(x: usize, y: usize, dst: (usize, usize)) -> Self {
        Self { x, y, dst }
    }

    fn dist(&self) -> i32 {
        (self.x as i32 - self.dst.0 as i32).abs() + (self.y as i32 - self.dst.1 as i32).abs()
    }
}

impl Ord for Coord {
    fn cmp(&self, other: &Self) -> Ordering {
        other.dist().cmp(&self.dist())
    }
}

impl PartialOrd for Coord {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Resource)]
pub struct Bfs {
    src: Coord,
    dst: Coord,
    queue: BinaryHeap<Coord>,
    visited: HashSet<Coord>,
}

impl Bfs {
    pub fn new() -> Self {
        Self {
            src: Coord::new(0, 0),
            dst: Coord::new(0, 0),
            queue: BinaryHeap::new(),
            visited: HashSet::new(),
        }
    }

    pub fn set_src(&mut self, src: Coord) {
        self.queue = BinaryHeap::from([src]);
        self.src = src;
    }

    pub fn set_dst(&mut self, dst: Coord) { 
        self.dst = dst;
    }

    pub fn set_visited(&mut self, visited: HashSet<Coord>) {
        self.visited = visited;
    } 

    pub fn is_valid(&self, coord: Coord) -> bool {
        self.src != coord && self.dst != coord
    }

    fn get_next_valid_coord(&mut self) -> Option<Coord> {
        while let Some(coord) = self.queue.pop() {
            if self.visited.insert(coord) {
                return Some(coord);
            }
        }

        None
    }

    pub fn step(&mut self) -> Option<Coord> {
        let next = self.get_next_valid_coord();

        if let Some(coord) = next {
            if self.dst == coord {
                self.queue.clear();
                return None;
            }

            for (dy, dx) in [(0,1),(1,0),(0,-1),(-1,0)] {
                let (y, x) = (coord.y as i32 + dy, coord.x as i32 + dx);

                if y < 0 || x < 0 || y >= super::ROWS as i32 || x >= super::COLS as i32 {
                    continue;
                }

                let coord = Coord::new_dst(x as usize, y as usize, (self.dst.x, self.dst.y));
                self.queue.push(coord);
            }
        }

        next
    }
}

