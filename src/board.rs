use crate::pieces::{Piece, PieceColor};
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
    selected_piece: ResMut<SelectedPiece>,
    squares_query: Query<&Square>,
) {
    // Only care about running this if the mouse button is pressed
    if !mouse_button_inputs.just_pressed(MouseButton::Left) {
        return;
    }

    // Get the square under the cursor and mark it as selected
    if let Some((square_entity, _intersection)) = pick_state.top(Group::default()) {
        // Ensure the selected square exists
        if squares_query.get(*square_entity).is_ok() {
            selected_square.entity = Some(*square_entity);
        }
    } else {
        // Player clicked outside the board
        deselect_all(selected_square, selected_piece);
    }
}

fn select_piece(
    selected_square: ChangedRes<SelectedSquare>,
    mut selected_piece: ResMut<SelectedPiece>,
    turn: Res<PlayerTurn>,
    squares_query: Query<&Square>,
    pieces_query: Query<(Entity, &Piece)>,
) {
    // Only care if a square is selected
    let square_entity = if let Some(entity) = selected_square.entity {
        entity
    } else {
        return;
    };

    // Find the square, if it exists
    let square = if let Ok(square) = squares_query.get(square_entity) {
        square
    } else {
        return;
    };

    if selected_piece.entity.is_none() {
        // Select the piece in the currently selected square
        for (piece_entity, piece) in pieces_query.iter() {
            if piece.x == square.x && piece.y == square.y && piece.color == turn.0 {
                selected_piece.entity = Some(piece_entity);
                break;
            }
        }
    }
}

fn move_piece(
    commands: &mut Commands,
    selected_square: ChangedRes<SelectedSquare>,
    selected_piece: Res<SelectedPiece>,
    mut turn: ResMut<PlayerTurn>,
    squares_query: Query<&Square>,
    mut pieces_query: Query<(Entity, &mut Piece)>,
) {
    // Only care if a square is selected
    let square_entity = if let Some(entity) = selected_square.entity {
        entity
    } else {
        return;
    };

    // Find the square, if it exists
    let square = if let Ok(square) = squares_query.get(square_entity) {
        square
    } else {
        return;
    };

    if let Some(selected_piece_entity) = selected_piece.entity {
        let pieces_vec = pieces_query
            .iter_mut()
            .map(|(_, piece)| *piece)
            .collect::<Vec<Piece>>();
        let pieces_entity_vec = pieces_query
            .iter_mut()
            .map(|(entity, piece)| (entity, *piece))
            .collect::<Vec<(Entity, Piece)>>();

        // Find the piece, if it exists
        let mut piece = if let Ok((_, piece)) = pieces_query.get_mut(selected_piece_entity) {
            piece
        } else {
            return;
        };

        // Move the piece to the selected square, if valid
        if piece.is_move_valid((square.x, square.y), &pieces_vec) {
            // Capture opposing pieces, if present
            for (other_entity, other_piece) in pieces_entity_vec {
                if other_piece.x == square.x
                    && other_piece.y == square.y
                    && other_piece.color != piece.color
                {
                    commands.despawn_recursive(other_entity);
                }
            }

            // Move piece
            piece.x = square.x;
            piece.y = square.y;

            // Change turn
            turn.change();
        }
    }
}

pub struct PlayerTurn(pub PieceColor);

impl Default for PlayerTurn {
    fn default() -> Self {
        Self(PieceColor::White)
    }
}

impl PlayerTurn {
    fn change(&mut self) {
        self.0 = match self.0 {
            PieceColor::White => PieceColor::Black,
            PieceColor::Black => PieceColor::White,
        }
    }
}

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<SelectedSquare>()
            .init_resource::<SelectedPiece>()
            .init_resource::<PlayerTurn>()
            .add_startup_system(create_board.system())
            .add_system(color_squares.system())
            .add_system(select_square.system())
            .add_system(select_piece.system())
            .add_system(move_piece.system());
    }
}
