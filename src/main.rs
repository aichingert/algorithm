use bevy::{
    prelude::*,
    window::PrimaryWindow,
};

mod tile;
use tile::{Tile, TileState, TileMap};

const TILES: usize= (HEIGHT / SIZE) as usize;

const OFFSET: f32 = (-(TILES as f32 / 2. * SIZE)) + SIZE / 2.;
const MARGIN: f32 = 15.0;
const SIZE: f32 = 50.0;

const WIDTH: f32 = 400.;
const HEIGHT: f32 = 400.;

fn main() {
    App::new()
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
        .run();
}

fn setup(
    mut commands: Commands,
    mut tiles: ResMut<TileMap>,
) {
    commands.spawn(Camera2dBundle::default());

    for row in 0..TILES {
        for col in 0..TILES {
            let position = Vec3::new(
                OFFSET + col as f32 * SIZE,
                OFFSET + row as f32 * SIZE,
                0.,
            );

            tiles[(row, col)] = Some(
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
    tiles: Res<TileMap>,
    buttons: Res<Input<MouseButton>>,
    mut t_query: Query<(&mut Tile, &mut Sprite)>,
) {
    if !buttons.pressed(MouseButton::Left) {
        return;
    }

    if let Some(Vec2 { x, y }) = windows.single().cursor_position() {
        let (x, y) = ((x/SIZE).floor() as usize,TILES - 1 - (y/SIZE).floor() as usize);

        if let Ok((_, mut sprite)) = t_query.get_mut(tiles[(y, x)].unwrap()) {
            sprite.color = Color::rgb(0., 0., 0.);
            //println!("{:?}", tile.position);
        }

    }
}
