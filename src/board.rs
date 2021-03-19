use bevy::prelude::*;
use bevy_mod_picking::PickableMesh;

pub fn create_board(
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
            commands
                .spawn(PbrBundle {
                    mesh: mesh.clone(),
                    material: if (x + y + 1) % 2 == 0 {
                        white_material.clone()
                    } else {
                        black_material.clone()
                    },
                    transform: Transform::from_translation(Vec3::new(x as f32, 0., y as f32)),
                    ..Default::default()
                })
                .with(PickableMesh::default());
        }
    }
}
