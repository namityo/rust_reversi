use crate::piece_type::PieceType;
use std::fmt;

#[derive(Clone, Copy, PartialEq)]
pub enum TileType {
    Piece(PieceType),
    Square,
    None,
}

impl fmt::Display for TileType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TileType::Piece(t) => write!(f, "{}", t),
            TileType::Square => write!(f, " "),
            TileType::None => write!(f, "×"),
        }
    }
}
