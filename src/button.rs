use bevy::prelude::*;

use crate::tile::State;

#[derive(Resource)]
pub struct Buttons {
    buttons: [Option<Entity>; 4],
}

impl Buttons {
    pub fn new() -> Self {
        Self { buttons: [None; 4] }
    }

    pub fn system(
        mut interaction_query: Query<
            (
                &Interaction,
                &mut BorderColor,
                &Children,
            ),
            (Changed<Interaction>, With<Button>),
        >,
        txt_query: Query<&Text>,
        mut state: ResMut<State>,
    ) {
        for (interaction, mut border_color, children) in &mut interaction_query {
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
                    border_color.0 = if state.is_same(txt) {
                        Color::WHITE
                    } else {
                        Color::GRAY
                    };
                }
            }
        }
    }

    pub fn spawn(&mut self, commands: &mut Commands) {
        let btns = [
            (super::BTN_SIZE * 1. - super::BTN_MARGIN,      "Start"), 
            (super::BTN_SIZE * 2. - super::BTN_MARGIN / 3., "End"), 
            (super::BTN_SIZE * 3. + super::BTN_MARGIN / 3., "Block"),
            (super::BTN_SIZE * 4. + super::BTN_MARGIN,      "Open"),
        ];

        for (i, (left, txt)) in btns.into_iter().enumerate() {
            self.buttons[i] = self.create_button(commands, Val::Px(left), txt);
        }
    }

    fn create_button(&self, commands: &mut Commands, left: Val, txt: &str) -> Option<Entity> {
        let mut entity = None;

        commands.spawn(NodeBundle {
            style: Style { left, top: Val::Px(super::TOP_OFFSET), ..default() },
            ..default()
        })
        .with_children(|parent| {
            let mut button = parent.spawn(ButtonBundle {
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
            });

            button.with_children(|builder| {
                builder.spawn(TextBundle::from_section(
                    txt,
                    TextStyle {
                        font_size: 24.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ));
            });

            entity = Some(button.id());
        });

        entity
    }
}
