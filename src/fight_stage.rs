use crate::creature::{Creature, CreatureTag, WIGGLE_MAX_ANGLE};
use crate::loading::{FontAssets, TextureAssets};
use crate::GameState;
use bevy::prelude::*;
use std::f32::consts::PI;

pub struct MenuPlugin;

/// This plugin is responsible for the game menu (containing only one button...)
/// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ButtonColors>()
            .init_resource::<Time>()
            .add_system(setup_menu.in_schedule(OnEnter(GameState::Fight)))
            .add_system(click_play_button.in_set(OnUpdate(GameState::Fight)))
            .add_system(update_health.in_set(OnUpdate(GameState::Fight)))
            .add_system(wiggle_spriteses.in_set(OnUpdate(GameState::Fight)))
            .add_system(update_health_bar.in_set(OnUpdate(GameState::Fight)));
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

fn create_creature_plaque(
    parent: &mut ChildBuilder,
    plaque_margins: UiRect,
    font: Handle<Font>,
    associated_creature: Creature,
) {
    let creature_tag = associated_creature.get_creature_tag();
    parent
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(200.0), Val::Auto),
                margin: plaque_margins,
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
                font,
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
                .insert(associated_creature);

            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Px(20.0)),
                        margin: UiRect::top(Val::Px(5.0)),
                        flex_wrap: FlexWrap::Wrap,
                        ..default()
                    },
                    background_color: BackgroundColor::from(Color::rgb_u8(99, 10, 10)),
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                                ..default()
                            },
                            background_color: BackgroundColor::from(Color::rgb_u8(198, 33, 33)),
                            ..default()
                        })
                        .insert(creature_tag)
                        .insert(HealthBar {});
                });
        });
}

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
    let opponent_creature = Creature::new("Bilbo".to_string());
    let creature_tag = creature.get_creature_tag();
    let opponent_creature_tag = opponent_creature.get_creature_tag();

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
                    create_creature_plaque(
                        parent,
                        UiRect {
                            top: Val::Auto,
                            right: Val::Auto,
                            left: Val::Px(20.0),
                            bottom: Val::Px(20.0),
                        },
                        font_assets.fira_sans.clone(),
                        creature,
                    );
                    create_creature_plaque(
                        parent,
                        UiRect {
                            top: Val::Auto,
                            left: Val::Auto,
                            right: Val::Px(20.0),
                            bottom: Val::Px(20.0),
                        },
                        font_assets.fira_sans.clone(),
                        opponent_creature,
                    );
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
            transform: Transform::from_translation(Vec3::new(-200.0, 50.0, 0.1))
                .with_scale(Vec3::new(-1., 1., 1.)),
            visibility: Visibility::Visible,
            ..default()
        })
        .insert(creature_tag);
    commands
        .spawn(SpriteBundle {
            texture: texture_assets.gargoylemon.clone(),
            transform: Transform::from_translation(Vec3::new(200.0, 50.0, 0.1)),
            visibility: Visibility::Visible,
            ..default()
        })
        .insert(opponent_creature_tag);
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
                creatures.iter_mut().for_each(|mut creature| {
                    let roll = rand::random::<u32>() % 7 + 1;
                    creature.set_wiggle_period(f32::max(rand::random::<f32>(), 0.2) + 0.05);
                    warn!("Dealt a _massive_ {roll} damage!");
                    creature.take_damage(roll)
                });
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
    mut healthbar_query: Query<(&mut Style, &HealthBar, &CreatureTag)>,
    creature_query: Query<&Creature>,
) {
    for (mut style, _, creature_tag) in healthbar_query.iter_mut() {
        let creature = creature_query
            .iter()
            .find(|creature| &creature.get_creature_tag() == creature_tag)
            .expect("Healthbar doesn't have associated creature!");
        let health_percentage =
            (creature.get_health() as f32 / creature.get_max_health() as f32) * 100.0;
        style.size.width = Val::Percent(health_percentage);
    }
}

/// My precious
fn wiggle_spriteses(
    time: Res<Time>,
    mut sprite_query: Query<(&mut Transform, &CreatureTag), Without<HealthBar>>,
    creature_query: Query<&Creature>,
) {
    for (mut transform, tag) in sprite_query.iter_mut() {
        let creature = creature_query
            .iter()
            .find(|creature| &creature.get_creature_tag() == tag)
            .expect("Sprite doesn't have an associated creature!");

        let rotation = ((time.elapsed_seconds() / creature.get_wiggle_period()) * (2. * PI)).cos()
            * WIGGLE_MAX_ANGLE;

        info!(
            "Creature called \"{}\"'s rotation is {}!",
            creature.get_name(),
            rotation
        );

        transform.rotation = Quat::from_rotation_z(rotation);
    }
}
