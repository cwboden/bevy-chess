use bevy::prelude::*;
use bevy_mod_picking::{DebugPickingPlugin, PickSource, PickingPlugin};

mod board;
mod pieces;
use board::create_board;
use pieces::create_pieces;

fn setup(commands: &mut Commands) {
    commands
        // Camera
        .spawn(Camera3dBundle {
            transform: Transform::from_matrix(Mat4::from_rotation_translation(
                Quat::from_xyzw(-0.3, -0.5, -0.3, 0.5).normalize(),
                Vec3::new(-7.0, 20.0, 4.0),
            )),
            ..Default::default()
        })
        .with(PickSource::default())
        // Light
        .spawn(LightBundle {
            transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
            ..Default::default()
        });
}

fn main() {
    App::build()
        .add_resource(Msaa { samples: 4 })
        .add_resource(WindowDescriptor {
            title: "Chess".to_string(),
            width: 800.,
            height: 800.,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(PickingPlugin)
        .add_plugin(DebugPickingPlugin)
        .add_startup_system(setup.system())
        .add_startup_system(create_board.system())
        .add_startup_system(create_pieces.system())
        .run();
}
