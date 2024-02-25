use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Copy, Clone, Debug)]
pub enum Color {
  White,
  Black,
}

#[wasm_bindgen]
#[derive(Copy, Clone, Debug)]
pub enum PieceType {
  Pawn,
  Rook,
  Knight,
  Bishop,
  Queen,
  King,
}

impl PieceType {
  pub fn iter() -> [PieceType; 6] {
    [
      PieceType::Pawn,
      PieceType::Rook,
      PieceType::Knight,
      PieceType::Bishop,
      PieceType::Queen,
      PieceType::King,
    ]
  }
}

#[wasm_bindgen]
#[derive(Copy, Clone, Debug)]
pub struct Piece {
  pub color: Color,
  pub piece_type: PieceType,
}
