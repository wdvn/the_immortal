mod player;

use bevy::prelude::*;
use bevy_editor_pls::prelude::*;

fn main() {
    App::new()
        .add_plugin(EditorPlugin::default())
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, player::register_player)
        .run()
}

fn setup(mut command: Commands, asset_server: Res<AssetServer>) {
    command.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 10000.,
            shadows_enabled: true,
            ..Default::default()
        },
        ..default()
    });
    command.spawn(Camera3dBundle {
        transform: Transform::from_xyz(335., 335., 335.).looking_at(Vec3::new(0., 0., 0.), Vec3::Y),
        ..default()
    });

    command.spawn(SceneBundle {
        scene: asset_server.load("Bot.glb#Scene0"),
        ..default()
    });
}
