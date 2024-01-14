use bevy::prelude::*;

use std::collections::VecDeque;

use crate::tile::{Tile, TileMap, TileState, State};
use crate::Bfs;

pub struct Buttons;

impl Buttons {
    pub fn system(
        mut interaction_query: Query<
            (&Interaction,&mut BorderColor, &Children),
            (Changed<Interaction>, With<Button>),
        >,
        txt_query: Query<&Text>,
        t_query: Query<&Tile>,
        tiles: Res<TileMap>,
        mut state: ResMut<State>,
        mut bfs: ResMut<Bfs>,
    ) {
        for (interaction, mut border_color, children) in &mut interaction_query {
            let txt = &txt_query.get(children[0]).unwrap().sections[0].value;

            match *interaction {
                Interaction::Pressed => {
                    border_color.0 = Color::RED;

                    if txt == "Solve" {
                        let mut start = VecDeque::new();
                        let mut end = (0, 0);

                        for i in 0..super::ROWS {
                            for j in 0..super::COLS {
                                let tile = t_query.get(tiles.get_entity(i, j)).unwrap();

                                match tile.state {
                                    TileState::Start => {
                                        start.push_back((i, j));
                                    }
                                    TileState::End => {
                                        end = (i, j);
                                    }
                                    _ => (),
                                }
                            }
                        }

                        bfs.queue = start.clone();
                        bfs.start = start;
                        bfs.goal = end;
                    }

                    state.set(txt);
                }
                Interaction::Hovered => {
                    border_color.0 = Color::WHITE;
                }
                Interaction::None => {
                    border_color.0 = if state.is_same(txt) {
                        Color::WHITE
                    } else {
                        Color::GRAY
                    };
                }
            }
        }
    }

    pub fn spawn(commands: &mut Commands) {
        let btns = [
            (super::BTN_SIZE * 1. - super::BTN_MARGIN,      "Start"), 
            (super::BTN_SIZE * 2. - super::BTN_MARGIN / 4., "End"), 
            (super::BTN_SIZE * 3. + super::BTN_MARGIN / 4., "Block"),
            (super::BTN_SIZE * 4. + super::BTN_MARGIN / 2., "Open"),
            (super::BTN_SIZE * 5. + super::BTN_MARGIN,      "Solve"),
        ];

        for (left, txt) in btns {
            Buttons::create_button(commands, Val::Px(left), txt);
        }
    }

    fn create_button(commands: &mut Commands, left: Val, txt: &str) {
        commands.spawn(NodeBundle {
            style: Style { left, top: Val::Px(super::TOP_OFFSET), ..default() },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(ButtonBundle {
                style: Style {
                    width: Val::Px(super::BTN_SIZE),
                    height: Val::Px(super::TOP - 2. * super::TOP_OFFSET),
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
                    txt,
                    TextStyle {
                        font_size: 24.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ));
            });
        });
    }
}
