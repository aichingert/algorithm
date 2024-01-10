use bevy::{
    prelude::*,
    window::PrimaryWindow,
};

const TILES: f32 = 10.;
const OFFSET: f32 = (-(TILES / 2. * SIZE)) + SIZE / 2.;
const MARGIN: f32 = 15.0;
const SIZE: f32 = 50.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, update)
        .run();
}

enum TileState {
    Start,
    Block,
    Open,
    End,
}

#[derive(Component)]
struct Tile {
    coord: Vec2,
    state: TileState,
}

impl Tile {
    fn create_open(coord: Vec2) -> Self {
        Self { 
            coord,
            state: TileState::Open,
        }
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    let mut sprite = SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(1., 1., 1.),
            custom_size: Some(Vec2::new(SIZE - MARGIN, SIZE - MARGIN)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
        ..default()
    };

    for row in 0..10 {
        for col in 0..10 {
            let position = Vec2::new(
                OFFSET + col as f32 * SIZE,
                OFFSET + row as f32 * SIZE,
            );

            sprite.transform = Transform::from_translation(position.extend(0.));
            commands.spawn((sprite.clone(), Tile::create_open(position)));
        }
    }
}

fn update(
    buttons: Res<Input<MouseButton>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    mut query: Query<(&mut Tile, &mut Sprite)>
) {
    if buttons.just_pressed(MouseButton::Left) {

        println!("{:?}", q_windows.single().cursor_position());
        /*
        for (mut tile, mut sprite) in query.iter_mut() {
        }
        */
    }
}
