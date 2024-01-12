use bevy::{
    prelude::*,
    window::PrimaryWindow,
};

mod button;
use button::Buttons;

mod tile;
use tile::{Tile, TileState, TileMap, State};

const ROWS: usize = ((HEIGHT - TOP) / SIZE) as usize;
const COLS: usize = (WIDTH / SIZE) as usize;

const TILES: usize = ROWS * COLS;

const ROW_OFFSET: f32 = (-(ROWS as f32 / 2. * SIZE)) + SIZE / 2.;
const COL_OFFSET: f32 = (-(COLS as f32 / 2. * SIZE)) + SIZE / 2.;

const MARGIN: f32 = 15.0;
const SIZE: f32 = 50.0;

const BTN_SIZE: f32 = WIDTH / 6.;
const BTN_MARGIN: f32 = BTN_SIZE / 1.5;

const TOP: f32 = 100.;
const TOP_OFFSET: f32 = 20.;

const WIDTH: f32 = 800.;
const HEIGHT: f32 = 800.;

fn main() {
    App::new()
        .insert_resource(State(TileState::Start))
        .insert_resource(Buttons::new())
        .insert_resource(TileMap { entities: [[None; TILES]; TILES] })
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (WIDTH, HEIGHT).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, mouse_pos)
        .add_systems(Update, Buttons::system)
        .run();
}

fn setup(
    mut commands: Commands,
    mut tiles: ResMut<TileMap>,
    mut buttons: ResMut<Buttons>,
) {
    commands.spawn(Camera2dBundle::default()); 
    buttons.spawn(&mut commands);

    let entities = tiles.get_mut_entities();

    for row in 0..ROWS {
        for col in 0..COLS {
            let position = Vec3::new(
                COL_OFFSET + col as f32 * SIZE,
                ROW_OFFSET + row as f32 * SIZE - TOP / 2 as f32,
                0.,
            );

            entities[row][col] = Some(
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

fn mouse_pos(
    windows: Query<&Window, With<PrimaryWindow>>,
    state: Res<State>,
    tiles: Res<TileMap>,
    buttons: Res<Input<MouseButton>>,
    mut t_query: Query<(&mut Tile, &mut Sprite)>,
) {
    if !buttons.pressed(MouseButton::Left) {
        return;
    }

    if let Some(Vec2 { x, y }) = windows.single().cursor_position() {
        if y < TOP { return; }

        let (x, y) = ((x/SIZE).floor() as usize,ROWS - 1 - ((y - TOP) /SIZE).floor() as usize);

        if let Ok((_, mut sprite)) = t_query.get_mut(tiles.get_entity(y, x)) {
            sprite.color = state.get_color();
        }
    }
}
