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
            .add_system(update_health.in_set(OnUpdate(GameState::Menu)));
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

    commands
        .spawn(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(120.0), Val::Px(50.0)),
                margin: button_position,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            background_color: button_colors.normal.into(),
            ..Default::default()
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

    // commands.spawn(
    //     ImageBundle {
    //         image: UiImage { texture: texture_assets.gargoylemon.clone(), ..default() },
    //         ..default()
    //     }
    // );

    let creature = Creature::new("Jeff".to_string());
    let creature_tag = creature.get_creature_tag();

    commands
        .spawn(TextBundle::from_section(
            "Jeff",
            TextStyle {
                font: font_assets.fira_sans.clone(),
                font_size: 40.0,
                color: Color::rgb(0.9, 0.9, 0.9),
            },
        ))
        .insert(creature);

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
                creatures
                    .iter_mut()
                    .for_each(|mut creature| creature.take_damage(10));
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
        text.sections[0].value = format!("Health: {}", creature.get_health())
    }
}

fn cleanup_menu(mut commands: Commands, button: Query<Entity, With<Button>>) {
    commands.entity(button.single()).despawn_recursive();
}
