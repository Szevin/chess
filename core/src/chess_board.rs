use wasm_bindgen::prelude::wasm_bindgen;
use crate::piece::{self, Piece, PieceType};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
#[derive(Copy, Clone, Debug)]
pub struct ChessBoard {
  _board: [[Option<Piece>; 8]; 8],
  pub turn: piece::Color,
}

#[wasm_bindgen]
impl ChessBoard {
  pub fn new() -> ChessBoard {
    let mut board = [[None; 8]; 8];
    for piece in PieceType::iter() {
      let color = piece::Color::White;
      let piece = Piece { color, piece_type: piece };
      board[0][0] = Some(piece);
    }

    ChessBoard { _board: board, turn: piece::Color::White}
  }

  pub fn print(&self) -> String {
    format!("{:?}", self)
  }
}
