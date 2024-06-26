//! # Basic FPS counter

use bevy::diagnostic::DiagnosticsStore;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;

use bevy::prelude::*;

#[derive(Component)]
pub struct FpsCounter;

pub struct FpsCounterPlugin;

impl Plugin for FpsCounterPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FrameTimeDiagnosticsPlugin::default());
        app.add_systems(Startup, setup_ui);
        app.add_systems(Update, update_ui);
    }
}

fn setup_ui(mut commands: Commands) {
    commands.spawn(NodeBundle {
        background_color: Color::rgba(0.0, 0.0, 0.0, 0.6).into(),
        style: Style {
            padding: UiRect {
                top: Val::Px(3.0),
                bottom: Val::Px(3.0),
                left: Val::Px(3.0),
                right: Val::Px(3.0),
            },
            margin: UiRect {
                top: Val::Px(10.0),
                bottom: Val::Px(3.0),
                left: Val::Px(10.0),
                right: Val::Px(3.0),
            },
            ..Default::default()
        },
        ..Default::default()
    })
    .with_children(|parent| {
        parent.spawn((
                TextBundle::from_sections([
                    TextSection::new(
                        "FPS: ",
                        TextStyle {
                            font_size: 20.0,
                            ..Default::default()
                        }),
                    TextSection::new(
                        "-",
                        TextStyle {
                            font_size: 20.0,
                            ..Default::default()
                        }),
                ]),
                FpsCounter,
        ));
    });
}

fn update_ui(
    mut fps_query: Query<&mut Text, With<FpsCounter>>,
    diagnostics: Res<DiagnosticsStore>,
) {
    for mut counter in fps_query.iter_mut() {
        if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                counter.sections[1].value = format!("{value:.0}");

                match value {
                    0.0..=25.0 => counter.sections[1].style.color = Color::RED,
                    25.0..=30.0 => counter.sections[1].style.color = Color::YELLOW,
                    30.0.. => counter.sections[1].style.color = Color::GREEN,
                    _ => counter.sections[1].style.color = Color::WHITE,
                }
            }
        }
    }
}
