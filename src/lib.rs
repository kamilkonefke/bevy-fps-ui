//! # Basic FPS counter

use bevy::diagnostic::DiagnosticsStore;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;

use bevy::color::palettes::css::{RED, LIME, YELLOW, WHITE};

#[derive(Component)]
pub struct FpsCounter;

pub struct FpsCounterPlugin;

impl Plugin for FpsCounterPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FrameTimeDiagnosticsPlugin);
        app.add_systems(Startup, setup_ui);
        app.add_systems(Update, update_ui);
    }
}

fn setup_ui(mut commands: Commands) {
    commands.spawn((
        Node {
            padding: UiRect::all(Val::Px(3.0)),
            margin: UiRect::all(Val::Px(5.0)),
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..Default::default()
        },
        BorderRadius::all(Val::Px(5.0)),
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.6).into()),
    ))
    .with_children(|parent| {
        parent.spawn((
            Text::new("FPS: "),
            TextFont {
                font_size: 32.0,
                ..Default::default()
            },
            TextColor(Color::WHITE),
        )).with_children(|parent| {
            parent.spawn((
                    TextSpan::default(),
                    TextFont {
                        font_size: 32.0,
                        ..Default::default()
                    },
                    TextColor(Color::WHITE),
                    FpsCounter,
            ));
        });
    });
}


fn update_ui(
    mut fps_text_query: Query<&mut TextSpan, With<FpsCounter>>,
    mut fps_color_query: Query<&mut TextColor, With<FpsCounter>>,
    diagnostics: Res<DiagnosticsStore>,
) {
    for mut counter in &mut fps_text_query {
        if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                **counter = format!("{value:.0}");
                for mut color in &mut fps_color_query {
                    match value {
                        0.0..=25.0 => color.0 = RED.into(),
                        25.0..=30.0 => color.0 = YELLOW.into(),
                        30.0.. => color.0 = LIME.into(),
                        _ => color.0 = WHITE.into(),
                    }
                }
            }
        }
    }
}
