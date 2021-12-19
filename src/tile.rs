use std::fmt;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum PieceType {
    Black,
    White,
}

impl PieceType {
    pub fn change(self) -> PieceType {
        return match self {
            PieceType::Black => PieceType::White,
            PieceType::White => PieceType::Black,
        };
    }
}

#[derive(Debug)]
pub enum TileType {
    Piece(PieceType),
    Square,
    None,
}

impl fmt::Display for TileType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = match self {
            TileType::Piece(t) => match t {
                PieceType::Black => "●",
                PieceType::White => "○",
            },
            TileType::Square => " ",
            TileType::None => "×",
        };
        return write!(f, "{}", str);
    }
}
