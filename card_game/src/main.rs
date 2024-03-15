use bevy::{prelude::*, window::WindowResolution};

mod features;

use features::{cards::*, deck::deck_plugin::DeckPlugin};

fn main() {
    App::default()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(1500., 900.),
                position: WindowPosition::Centered(MonitorSelection::Index(0)),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(DeckPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    for (color, variant, transform) in [
        (
            CardColor::Blue,
            CardVariant::Number(9),
            Transform::from_xyz(-300., 0., 0.),
        ),
        (
            CardColor::Wild,
            CardVariant::Wild,
            Transform::from_xyz(-100., 0., 0.),
        ),
        (
            CardColor::Wild,
            CardVariant::PlusFour,
            Transform::from_xyz(100., 0., 0.),
        ),
        (
            CardColor::Yellow,
            CardVariant::Invert,
            Transform::from_xyz(300., 0., 0.),
        ),
    ] {
        let texture_path = CardBundle::texture_path(color, variant);

        let texture = asset_server.load(texture_path);
        commands.spawn((
            CardBundle { color, variant },
            SpriteBundle {
                texture,
                transform,
                ..default()
            },
        ));
    }
}
