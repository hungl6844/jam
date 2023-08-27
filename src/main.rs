use bevy::{
    prelude::{
        App, AssetServer, Camera3dBundle, Commands, DirectionalLight, DirectionalLightBundle, Res,
        Startup, Transform, Vec3, Handle, AnimationClip, Resource, Update, AnimationPlayer, Added, Query, PluginGroup,
    },
    scene::SceneBundle,
    DefaultPlugins, window::{Window, WindowPlugin}, diagnostic::FrameTimeDiagnosticsPlugin,
};

#[derive(Resource)]
pub struct Animations(Vec<Handle<AnimationClip>>);

fn main() {
    App::new()
    .add_plugins((
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "jam".into(),
                ..Default::default()
            }),
            ..Default::default()
        }),
        FrameTimeDiagnosticsPlugin,
    ))
        .add_systems(Startup, setup)
        .add_systems(Update, run_animation)
        .run();
}

fn run_animation(animations: Res<Animations>, mut players: Query<&mut AnimationPlayer, Added<AnimationPlayer>>) {
    for mut player in &mut players {

        // this plays the animation
        player.play(animations.0[0].clone_weak()).repeat();
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {

    // loads the animation here. the animations are ordered from 0 corresponding to the order of the animations in blender
    commands.insert_resource(Animations(vec![asset_server.load("character.glb#Animation0")]));

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            illuminance: 50000.0,
            ..Default::default()
        },
        ..Default::default()
    });

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(35.0, 35.0, 35.0)
            .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
        ..Default::default()
    });

    commands.spawn(SceneBundle {
        scene: asset_server.load("character.glb#Scene0"),
        ..Default::default()
    });
}
