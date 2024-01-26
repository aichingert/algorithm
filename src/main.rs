use bevy::{
    prelude::*,
    window::PrimaryWindow,
};

use std::collections::HashSet;

mod button;
use button::Buttons;

mod tile;
use tile::{Tile, TileState, TileMap, State};

const ROWS: usize = ((HEIGHT - TOP) / SIZE) as usize;
const COLS: usize = (WIDTH / SIZE) as usize;

const ROW_OFFSET: f32 = (-(ROWS as f32 / 2. * SIZE)) + SIZE / 2.;
const COL_OFFSET: f32 = (-(COLS as f32 / 2. * SIZE)) + SIZE / 2.;

const MARGIN: f32 = 7.5;
const SIZE: f32 = 25.0;

const BTN_SIZE: f32 = WIDTH / 7.;
const BTN_MARGIN: f32 = BTN_SIZE / 1.5;

const TOP: f32 = 100.;
const TOP_OFFSET: f32 = 20.;

const WIDTH: f32 = 800.;
const HEIGHT: f32 = 800.;

fn main() {
    App::new()
        .insert_resource(State(TileState::Start))
        .insert_resource(Bfs::new())
        .insert_resource(TileMap { entities: [[None; COLS]; ROWS] })
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (WIDTH, HEIGHT).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, (draw, Buttons::system))
        .add_systems(Update, solve/*.run_if(on_timer(Duration::from_millis(100))*/)
        .run();
}


#[derive(Resource, Debug)]
struct Bfs {
    pub queue: Vec<(usize, usize)>,
    pub goal: (usize, usize),
    pub start: Vec<(usize, usize)>,
    pub blocked: HashSet<(i32, i32)>,
    is_finished: bool,
    seen: HashSet<(usize, usize)>,
}

impl Bfs {
    fn new() -> Self {
        Self { 
            queue: Vec::new(), 
            seen: HashSet::new(), 
            start: Vec::new(),
            goal: (0, 0), 
            blocked: HashSet::new(),
            is_finished: false 
        }
    }

    fn dis(&self, coord: &(usize, usize)) -> i32 {
        (self.goal.0 as i32 - coord.0 as i32).abs() + (self.goal.1 as i32 - coord.1 as i32).abs()
    }

    fn step(&mut self) -> Option<(usize, usize)> {
        let mut current = None;

        {
            let mut queue = self.queue.clone();
            queue.sort_by(|a, b| self.dis(b).cmp(&self.dis(a)));
            self.queue = queue;
        }

        while current.is_none() && !self.queue.is_empty() {
            let next = self.queue.pop().unwrap();

            if self.seen.insert(next) {
                current = Some(next);
            }
        }

        if let Some((y, x)) = current {
            if self.goal == (y, x) {
                self.is_finished = true;
                return None;
            }

            for (dy, dx) in [(0,1),(1,0),(0,-1),(-1,0)] {
                let (ny, nx) = (y as i32 + dy, x as i32 + dx);

                if ny < 0 || nx < 0 || ny >= ROWS as i32 || nx >= COLS as i32 || self.blocked.contains(&(ny, nx)) {
                    continue;
                }

                self.queue.push((ny as usize, nx as usize));
            }
        } else {
            self.is_finished = true;
        }

        current
    }
}

fn solve(
    state: Res<State>,
    mut bfs: ResMut<Bfs>,
    tiles: ResMut<TileMap>,
    mut t_query: Query<(&mut Tile, &mut Sprite)>,
) {
    if state.is_drawable() || bfs.is_finished {
        return;
    }

    if let Some((y, x)) = bfs.step() {
        if bfs.start.contains(&(y, x)) {
            return;
        }

        let (_, mut sprite) = t_query.get_mut(tiles.get_entity(y, x)).unwrap();
        sprite.color = Color::YELLOW;
    }
}

fn setup(
    mut commands: Commands,
    mut tiles: ResMut<TileMap>,
) {
    commands.spawn(Camera2dBundle::default()); 
    Buttons::spawn(&mut commands);

    let entities = tiles.get_mut_entities();

    for (row, rows) in entities.iter_mut().enumerate().take(ROWS) {
        for (col, entity) in rows.iter_mut().enumerate().take(COLS) {
            let position = Vec3::new(
                COL_OFFSET + col as f32 * SIZE,
                ROW_OFFSET + row as f32 * SIZE - TOP / 2.,
                0.,
            );

            *entity = Some(
                commands.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            color: Color::rgb(1., 1., 1.),
                            custom_size: Some(Vec2::new(SIZE - MARGIN, SIZE - MARGIN)),
                            ..default()
                        },
                        transform: Transform::from_translation(position),
                        ..default()
                    },
                    Tile::new((row, col), TileState::Open),
                )).id()
            );
        }
    }
}

fn draw(
    windows: Query<&Window, With<PrimaryWindow>>,
    state: Res<State>,
    tiles: Res<TileMap>,
    buttons: Res<Input<MouseButton>>,
    mut t_query: Query<(&mut Tile, &mut Sprite)>,
) {
    if !buttons.pressed(MouseButton::Left) || !state.is_drawable() {
        return;
    }

    if let Some(Vec2 { x, y }) = windows.single().cursor_position() {
        if y < TOP { return; }

        let (x, y) = ((x/SIZE).floor() as usize,ROWS - 1 - ((y - TOP) /SIZE).floor() as usize);

        if let Ok((mut t, mut sprite)) = t_query.get_mut(tiles.get_entity(y, x)) {
            t.state = state.0;
            sprite.color = state.get_color();
        }
    }
}
