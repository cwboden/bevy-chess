use bevy::prelude::*;

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
        // Light
        .spawn(LightBundle {
            transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
            ..Default::default()
        });
}

fn create_board(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(Mesh::from(shape::Plane { size: 1.0 }));
    let white_material = materials.add(Color::rgb(1., 0.9, 0.9).into());
    let black_material = materials.add(Color::rgb(0., 0.1, 0.1).into());

    // Create the 64 squares
    for x in 0..8 {
        for y in 0..8 {
            commands.spawn(PbrBundle {
                mesh: mesh.clone(),
                material: if (x + y + 1) % 2 == 0 {
                    white_material.clone()
                } else {
                    black_material.clone()
                },
                transform: Transform::from_translation(Vec3::new(x as f32, 0., y as f32)),
                ..Default::default()
            });
        }
    }
}

fn create_pieces(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Load meshes
    let king_handle: Handle<Mesh> = asset_server.load("models/pieces.glb#Mesh0/Primitive0");
    let king_cross_handle: Handle<Mesh> = asset_server.load("models/pieces.glb#Mesh1/Primitive0");
    let _pawn_handle: Handle<Mesh> = asset_server.load("models/pieces.glb#Mesh2/Primitive0");
    let _knight_1_handle: Handle<Mesh> = asset_server.load("models/pieces.glb#Mesh3/Primitive0");
    let _knight_2_handle: Handle<Mesh> = asset_server.load("models/pieces.glb#Mesh4/Primitive0");
    let _rook_handle: Handle<Mesh> = asset_server.load("models/pieces.glb#Mesh5/Primitive0");
    let _bishop_handle: Handle<Mesh> = asset_server.load("models/pieces.glb#Mesh6/Primitive0");
    let _queen_handle: Handle<Mesh> = asset_server.load("models/pieces.glb#Mesh7/Primitive0");

    // Create materials
    let white_material = materials.add(Color::rgb(1., 0.8, 0.8).into());
    let _black_material = materials.add(Color::rgb(0., 0.2, 0.2).into());

    commands
        .spawn(PbrBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 4.0)),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn(PbrBundle {
                mesh: king_handle.clone(),
                material: white_material.clone(),
                transform: {
                    let mut transform = Transform::from_translation(Vec3::new(-0.2, 0., -1.9));
                    transform.apply_non_uniform_scale(Vec3::new(0.2, 0.2, 0.2));
                    transform
                },
                ..Default::default()
            });
            parent.spawn(PbrBundle {
                mesh: king_cross_handle.clone(),
                material: white_material.clone(),
                transform: {
                    let mut transform = Transform::from_translation(Vec3::new(-0.2, 0., -1.9));
                    transform.apply_non_uniform_scale(Vec3::new(0.2, 0.2, 0.2));
                    transform
                },
                ..Default::default()
            });
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
        .add_startup_system(setup.system())
        .add_startup_system(create_board.system())
        .add_startup_system(create_pieces.system())
        .run();
}
