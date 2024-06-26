use bevy::prelude::*;
use bevy_fps_ui::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, FpsCounterPlugin))
        .add_systems(Startup, setup_camera)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
