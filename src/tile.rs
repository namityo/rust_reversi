use std::fmt;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum PieceType {
    Black,
    White,
}

impl PieceType {
    pub fn change(self) -> PieceType {
        match self {
            PieceType::Black => PieceType::White,
            PieceType::White => PieceType::Black,
        }
    }
}

impl fmt::Display for PieceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = match self {
            PieceType::Black => "●",
            PieceType::White => "○",
        };
        write!(f, "{}", str)
    }
}

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
