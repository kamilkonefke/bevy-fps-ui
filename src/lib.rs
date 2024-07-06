//! # Basic FPS counter

use bevy::diagnostic::DiagnosticsStore;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;

use bevy::color::palettes::css::{RED, LIME, YELLOW, WHITE};
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
        background_color: Srgba::new(0.0, 0.0, 0.0, 0.6).into(),
        border_radius: BorderRadius::all(Val::Px(5.0)),
        style: Style {
            padding: UiRect::all(Val::Px(3.0)),
            margin: UiRect::all(Val::Px(5.0)),
            justify_self: JustifySelf::End,
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
                    0.0..=25.0 => counter.sections[1].style.color = RED.into(),
                    25.0..=30.0 => counter.sections[1].style.color = YELLOW.into(),
                    30.0.. => counter.sections[1].style.color = LIME.into(),
                    _ => counter.sections[1].style.color = WHITE.into(),
                }
            }
        }
    }
}
