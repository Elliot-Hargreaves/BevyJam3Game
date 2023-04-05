use crate::creature::Creature;
use crate::loading::{FontAssets, TextureAssets};
use crate::GameState;
use bevy::prelude::*;

pub struct MenuPlugin;

/// This plugin is responsible for the game menu (containing only one button...)
/// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ButtonColors>()
            .add_system(setup_menu.in_schedule(OnEnter(GameState::Menu)))
            .add_system(click_play_button.in_set(OnUpdate(GameState::Menu)))
            .add_system(cleanup_menu.in_schedule(OnExit(GameState::Menu)))
            .add_system(update_health.in_set(OnUpdate(GameState::Menu)))
            .add_system(update_health_bar.in_set(OnUpdate(GameState::Menu)));
    }
}

#[derive(Resource)]
struct ButtonColors {
    normal: Color,
    hovered: Color,
}

impl Default for ButtonColors {
    fn default() -> Self {
        ButtonColors {
            normal: Color::rgb(0.15, 0.15, 0.15),
            hovered: Color::rgb(0.25, 0.25, 0.25),
        }
    }
}

#[derive(Component)]
struct HealthBar {}

fn setup_menu(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    texture_assets: Res<TextureAssets>,
    button_colors: Res<ButtonColors>,
) {
    commands.spawn(Camera2dBundle::default());
    let mut button_position: UiRect = UiRect::horizontal(Val::Auto);
    button_position.bottom = Val::Percent(20.0);
    button_position.top = Val::Auto;

    let creature = Creature::new("Jeff".to_string());
    let creature_tag = creature.get_creature_tag();

    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::all(Val::Percent(100.0)),
                flex_wrap: FlexWrap::Wrap,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            // Play Area
            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(80.0)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    // Player creature plaque
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Px(200.0), Val::Auto),
                                margin: UiRect {
                                    top: Val::Auto,
                                    right: Val::Auto,
                                    left: Val::Px(20.0),
                                    bottom: Val::Px(20.0),
                                },
                                border: UiRect::all(Val::Px(10.0)),
                                flex_wrap: FlexWrap::Wrap,
                                ..default()
                            },
                            background_color: BackgroundColor::from(Color::Rgba {
                                red: 0.5,
                                green: 0.5,
                                blue: 0.5,
                                alpha: 0.9,
                            }),

                            ..default()
                        })
                        .with_children(|parent| {
                            let creature_status_text_style = TextStyle {
                                font: font_assets.fira_sans.clone(),
                                font_size: 20.0,
                                color: Color::rgb(0.1, 0.1, 0.1),
                            };
                            parent
                                .spawn(
                                    TextBundle::from_sections(vec![
                                        // Name
                                        TextSection {
                                            value: String::new(),
                                            style: creature_status_text_style.clone(),
                                        },
                                        // health
                                        TextSection {
                                            value: String::new(),
                                            style: creature_status_text_style,
                                        },
                                    ])
                                    .with_style(Style {
                                        flex_wrap: FlexWrap::Wrap,
                                        ..default()
                                    }),
                                )
                                .insert(creature);

                            parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        size: Size::new(Val::Percent(100.0), Val::Px(20.0)),
                                        margin: UiRect::top(Val::Px(5.0)),
                                        flex_wrap: FlexWrap::Wrap,
                                        ..default()
                                    },
                                    background_color: BackgroundColor::from(Color::rgb_u8(
                                        99, 10, 10,
                                    )),
                                    ..default()
                                })
                                .with_children(|parent| {
                                    parent
                                        .spawn(NodeBundle {
                                            style: Style {
                                                size: Size::new(
                                                    Val::Percent(100.0),
                                                    Val::Percent(100.0),
                                                ),
                                                ..default()
                                            },
                                            background_color: BackgroundColor::from(Color::rgb_u8(
                                                198, 33, 33,
                                            )),
                                            ..default()
                                        })
                                        .insert(HealthBar {});
                                });
                        });
                });

            // Button area
            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(20.0)),
                        flex_wrap: FlexWrap::Wrap,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(ButtonBundle {
                            style: Style {
                                size: Size::new(Val::Px(120.0), Val::Px(50.0)),
                                margin: UiRect::all(Val::Auto),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            background_color: button_colors.normal.into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Attack",
                                TextStyle {
                                    font: font_assets.fira_sans.clone(),
                                    font_size: 40.0,
                                    color: Color::rgb(0.9, 0.9, 0.9),
                                },
                            ));
                        });
                });
        });

    commands
        .spawn(SpriteBundle {
            texture: texture_assets.gargoylemon.clone(),
            transform: Transform::from_translation(Vec3::new(-80.0, 0.0, 0.1)),
            visibility: Visibility::Visible,
            ..default()
        })
        .insert(creature_tag);
}

fn click_play_button(
    button_colors: Res<ButtonColors>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut creatures: Query<&mut Creature>,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                let roll = rand::random::<u32>() % 7 + 1;
                warn!("Dealt a _massive_ {roll} damage!");
                creatures
                    .iter_mut()
                    .for_each(|mut creature| creature.take_damage(roll));
            }
            Interaction::Hovered => {
                *color = button_colors.hovered.into();
            }
            Interaction::None => {
                *color = button_colors.normal.into();
            }
        }
    }
}

fn update_health(mut health_text_query: Query<(&mut Text, &Creature), With<Creature>>) {
    for (mut text, creature) in health_text_query.iter_mut() {
        text.sections[0].value = format!("Name: {}\n", creature.get_name());
        text.sections[1].value = format!("Health: {}", creature.get_health());
    }
}

fn update_health_bar(
    mut healthbar_query: Query<(&mut Style, &HealthBar)>,
    creature_query: Query<&Creature>,
) {
    let creature = creature_query.single();
    let (mut healthbar_style, _) = healthbar_query.single_mut();
    let health_percentage =
        (creature.get_health() as f32 / creature.get_max_health() as f32) * 100.0;
    healthbar_style.size.width = Val::Percent(health_percentage);
}

fn cleanup_menu(mut commands: Commands, button: Query<Entity, With<Button>>) {
    commands.entity(button.single()).despawn_recursive();
}
