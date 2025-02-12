use bevy::app::AppExit;
use bevy::prelude::*;
use rand::{rng, seq::IndexedRandom, Rng};

use super::{
    components::{EnterButton, ExitButton},
    PlayerName,
};

pub fn generate_random_name(mut commands: Commands) {
    let adjs = ["Fast", "Brave", "Mighty", "Silent", "Swift"];
    let nouns = ["Tiger", "Eagle", "Dragon", "Wolf", "Panther"];

    let mut rng = rng();

    let random_name = format!(
        "{}{}{}",
        adjs.choose(&mut rng).unwrap(),
        nouns.choose(&mut rng).unwrap(),
        rng.random_range(100..999)
    );

    commands.insert_resource(PlayerName(random_name));
}

/////////////////////////////////////////

const NORMAL_BUTTON: Color = Color::srgb(0., 1.0, 0.);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.75, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.85, 0.35);

// Exit Button Colors
const EXIT_NORMAL: Color = Color::srgb(1.0, 0.0, 0.0);
const EXIT_HOVERED: Color = Color::srgb(0.75, 0.25, 0.25);
const EXIT_PRESSED: Color = Color::srgb(0.85, 0.35, 0.35);

pub fn spawn_welcome_screen(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    player_name: Res<PlayerName>,
) {
    let font: Handle<Font> = asset_server.load("fonts/FiraSans-Bold.ttf");

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                padding: UiRect::all(Val::Px(12.0)),
                row_gap: Val::Px(12.0),
                ..default()
            },
            BackgroundColor(Color::BLACK),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Welcome back!"),
                TextFont {
                    font: font.clone(),
                    font_size: 24.0,
                    ..default()
                },
                TextColor::WHITE,
            ));
        })
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        width: Val::Px(220.),
                        height: Val::Px(50.),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::top(Val::Px(10.)),
                        border: UiRect::all(Val::Px(3.)),

                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
                    BorderColor(Color::WHITE),
                ))
                .with_children(|box_parent| {
                    box_parent.spawn((
                        Text::new(player_name.0.clone()),
                        TextColor::WHITE,
                        TextFont {
                            font: font.clone(),
                            font_size: 20.0,
                            ..default()
                        },
                    ));
                });
        })
        .with_children(|parent| {
            parent
                .spawn((
                    EnterButton {},
                    Button,
                    Node {
                        width: Val::Px(220.),
                        height: Val::Px(50.),

                        border: UiRect::all(Val::Px(5.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::top(Val::Px(10.0)),
                        ..default()
                    },
                    BorderColor(NORMAL_BUTTON),
                    BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
                ))
                .with_child((
                    Text::new("Enter game"),
                    TextFont {
                        font: font.clone_weak(),
                        font_size: 20.0,
                        ..Default::default()
                    },
                    TextColor(NORMAL_BUTTON),
                ));
        })
        .with_children(|parent| {
            parent
                .spawn((
                    ExitButton {},
                    Button,
                    Node {
                        width: Val::Px(220.),
                        height: Val::Px(50.),

                        border: UiRect::all(Val::Px(5.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::top(Val::Px(10.0)),
                        ..default()
                    },
                    BorderColor(EXIT_NORMAL),
                    BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
                ))
                .with_child((
                    Text::new("Exit game"),
                    TextFont {
                        font: font.clone_weak(),
                        font_size: 20.0,
                        ..Default::default()
                    },
                    TextColor(EXIT_NORMAL),
                ));
        });
}

pub fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BorderColor, &Children),
        (Changed<Interaction>, With<Button>, With<EnterButton>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut border_color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();

        match *interaction {
            Interaction::Hovered => {
                **text = "Join now...".to_string();
                border_color.0 = HOVERED_BUTTON;
            }
            Interaction::Pressed => {
                **text = "Loading...".to_string();
                border_color.0 = PRESSED_BUTTON;
            }
            Interaction::None => {
                **text = "Enter game".to_string();
                border_color.0 = NORMAL_BUTTON;
            }
        }
    }
}

pub fn exit_button_system(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<ExitButton>)>,
    mut exit_event_writer: EventWriter<AppExit>,
) {
    for interaction in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            println!("Exit button pressed! Quitting game...");
            exit_event_writer.send(AppExit::Success);
        }
    }
}
