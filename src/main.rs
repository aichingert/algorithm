use bevy::{
    prelude::*,
    window::PrimaryWindow,
};

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
        .add_systems(Update, button_system)
        .run();
}

fn setup(
    mut commands: Commands,
    mut tiles: ResMut<TileMap>,
) {
    commands.spawn(Camera2dBundle::default()); 

    spawn_node(&mut commands, "Start", Val::Px(BTN_SIZE - BTN_MARGIN), Val::Px(TOP_OFFSET));
    spawn_node(&mut commands, "End",   Val::Px(BTN_SIZE *2. - BTN_MARGIN /3.), Val::Px(TOP_OFFSET));
    spawn_node(&mut commands, "Block", Val::Px(BTN_SIZE *3. + BTN_MARGIN /3.), Val::Px(TOP_OFFSET));
    spawn_node(&mut commands, "Open", Val::Px(BTN_SIZE *4. + BTN_MARGIN), Val::Px(TOP_OFFSET));

    for row in 0..ROWS {
        for col in 0..COLS {
            let position = Vec3::new(
                COL_OFFSET + col as f32 * SIZE,
                ROW_OFFSET + row as f32 * SIZE - TOP / 2 as f32,
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

fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    txt_query: Query<&Text>,
    mut state: ResMut<State>,

) {
    for (interaction, mut color, mut border_color, children) in &mut interaction_query {
        let txt = &txt_query.get(children[0]).unwrap().sections[0].value;

        match *interaction {
            Interaction::Pressed => {
                border_color.0 = Color::RED;
                state.set(txt);
            }
            Interaction::Hovered => {
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                println!("{txt}");
                border_color.0 = if state.is_same(txt) {
                    Color::WHITE
                } else {
                    Color::GRAY
                };
            }
        }
    }
}

fn spawn_node(commands: &mut Commands, text: &str, left: Val, top: Val) {
    commands.spawn(NodeBundle {
       style: Style {
           left,
           top,
            ..default()
        },
        ..default()
    })
    .with_children(|parent| {
        parent.spawn(ButtonBundle {
            style: Style {
                width: Val::Px(BTN_SIZE),
                height: Val::Px(60.),
                border: UiRect::all(Val::Px(5.)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            border_color: Color::GRAY.into(),
            background_color: Color::rgb(0.15, 0.15, 0.15).into(),
            ..default()
        })
        .with_children(|builder| {
            builder.spawn(TextBundle::from_section(
                text,
                TextStyle {
                    font_size: 24.0,
                    color: Color::WHITE,
                    ..default()
                },
            ));
        });
    });
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

        if let Ok((_, mut sprite)) = t_query.get_mut(tiles[(y, x)].unwrap()) {
            sprite.color = state.get_color();
        }
    }
}
