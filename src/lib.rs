/// # rust_reversi
/// 
/// Rust製のオセロクレート
/// 
/// # 使い方
/// 
/// ```
/// extern crate rust_reversi;
/// use rust_reversi::board::Board;
/// use rust_reversi::piece_type::PieceType;
/// use rust_reversi::point::Point;
/// 
/// let mut board = Board::new(8, 8);
/// 
/// let point = Point::new(4, 3);
/// let piece_type = PieceType::Black;
/// 
/// if board.can_put_piece(piece_type, &point) {
///     board = board.put_piece(piece_type, point);
///     board.print();
/// }
/// 
/// if let Some(piece_type) = board.get_winner() {
///     println!("{} の勝利!", piece_type)
/// } else {
///     println!("同点")
/// }
/// ```
/// 

pub mod board;
pub mod piece_type;
mod tile_type;
pub mod point;
mod tile;
