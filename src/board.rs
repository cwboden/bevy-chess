use crate::pieces::Piece;
use bevy::prelude::*;
use bevy_mod_picking::{Group, PickState, PickableMesh};

pub struct Square {
    pub x: u8,
    pub y: u8,
}

impl Square {
    fn is_white(&self) -> bool {
        (self.x + self.y + 1) % 2 == 0
    }
}

#[derive(Default)]
struct SelectedSquare {
    entity: Option<Entity>,
}

#[derive(Default)]
struct SelectedPiece {
    entity: Option<Entity>,
}

fn create_board(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(Mesh::from(shape::Plane { size: 1.0 }));

    // Create the 64 squares
    for x in 0..8 {
        for y in 0..8 {
            let square = Square { x, y };
            commands
                .spawn(PbrBundle {
                    mesh: mesh.clone(),
                    material: if square.is_white() {
                        materials.add(Color::rgb(1., 0.9, 0.9).into())
                    } else {
                        materials.add(Color::rgb(0., 0.1, 0.1).into())
                    },
                    transform: Transform::from_translation(Vec3::new(x as f32, 0., y as f32)),
                    ..Default::default()
                })
                .with(PickableMesh::default())
                .with(square);
        }
    }
}

fn color_squares(
    pick_state: Res<PickState>,
    selected_square: Res<SelectedSquare>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<(Entity, &Square, &Handle<StandardMaterial>)>,
) {
    let top_entity = if let Some((entity, _intersection)) = pick_state.top(Group::default()) {
        Some(*entity)
    } else {
        None
    };

    for (entity, square, material_handle) in query.iter() {
        // Get the material
        let material = materials.get_mut(material_handle).unwrap();

        // Change the color of the selected square
        material.albedo = if Some(entity) == selected_square.entity {
            Color::rgb(0.8, 0.6, 0.1)
        } else if Some(entity) == top_entity {
            Color::rgb(0.8, 0.3, 0.3)
        } else if square.is_white() {
            Color::rgb(1., 0.9, 0.9)
        } else {
            Color::rgb(0., 0.1, 0.1)
        };
    }
}

fn deselect_all(
    mut selected_square: ResMut<SelectedSquare>,
    mut selected_piece: ResMut<SelectedPiece>,
) {
    selected_square.entity = None;
    selected_piece.entity = None;
}

fn select_square(
    pick_state: Res<PickState>,
    mouse_button_inputs: Res<Input<MouseButton>>,
    mut selected_square: ResMut<SelectedSquare>,
    mut selected_piece: ResMut<SelectedPiece>,
    squares_query: Query<&Square>,
    mut pieces_query: Query<(Entity, &mut Piece)>,
) {
    // Only care about running this if the mouse button is pressed
    if !mouse_button_inputs.just_pressed(MouseButton::Left) {
        return;
    }

    if let Some((square_entity, _intersection)) = pick_state.top(Group::default()) {
        if let Ok(square) = squares_query.get(*square_entity) {
            // Mark clicked square as selected
            selected_square.entity = Some(*square_entity);

            // If a piece is currently selected, move it to chosen square
            if let Some(selected_piece_entity) = selected_piece.entity {
                let pieces_vec: Vec<Piece> =
                    pieces_query.iter_mut().map(|(_, piece)| *piece).collect();

                if let Ok((_piece_entity, mut piece)) = pieces_query.get_mut(selected_piece_entity)
                {
                    if piece.is_move_valid((square.x, square.y), &pieces_vec) {
                        piece.x = square.x;
                        piece.y = square.y;
                    }
                }

                // Move confirmed, deselect everything
                deselect_all(selected_square, selected_piece);
            } else {
                // Otherwise, select piece in chosen square (if it exists)
                for (piece_entity, piece) in pieces_query.iter_mut() {
                    if piece.x == square.x && piece.y == square.y {
                        selected_piece.entity = Some(piece_entity);
                        break;
                    }
                }
            }
        }
    } else {
        // Clicked outside board, deselect all
        deselect_all(selected_square, selected_piece);
    }
}

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<SelectedSquare>()
            .init_resource::<SelectedPiece>()
            .add_startup_system(create_board.system())
            .add_system(color_squares.system())
            .add_system(select_square.system());
    }
}
