use bevy::prelude::*;

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};

#[derive(Eq, PartialEq, Clone, Copy, Hash)]
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

    fn dist(&self) -> i32 {
        (self.x as i32 - self.dst.0 as i32).abs() + (self.y as i32 - self.dst.1 as i32).abs()
    }
}

impl Ord for Coord {
    fn cmp(&self, other: &Self) -> Ordering {
        self.dist().cmp(&other.dist())
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

    pub fn step(&mut self) -> Option<Coord> {
        None
    }
}

