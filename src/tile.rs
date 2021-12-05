#[derive(Clone, Copy, PartialEq, Debug)]
pub enum PieceType {
    Black,
    White,
}

#[derive(Debug)]
pub enum TileType {
    Piece(PieceType),
    Square,
    None,
}

impl TileType {
    pub fn to_string(&self) -> &str {
        match self {
            TileType::Piece(t) => match t {
                PieceType::Black => "●",
                PieceType::White => "○",
            },
            TileType::Square => " ",
            TileType::None => "×",
        }
    }
}