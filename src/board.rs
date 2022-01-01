use crate::point::Point;
use crate::tile_type::TileType;
use crate::piece_type::PieceType;
use crate::tile::Tile;
use std::cmp::Ordering;

/// # Board 構造体
/// 
/// オセロの盤面を表している構造体
/// 
pub struct Board {
    tiles: Tile,
    x_size: usize,
    y_size: usize,
}

impl Board {
    /// 新しく盤面を生成します。`x_size`と`y_size`は通常8を指定します。
    /// 
    pub fn new(x_size: usize, y_size: usize) -> Board {
        Board {
            tiles: Tile::new(x_size, y_size),
            x_size,
            y_size,
        }
    }

    /// コンソール画面に盤面を表示します。
    /// 
    pub fn print(&self) {
        println!(" |0|1|2|3|4|5|6|7|8|9|");
        for y in 0..=(self.y_size + 1) {
            print!("{}|", y);
            for x in 0..=(self.x_size + 1) {
                if let Some(tile) = self.tiles.get(&Point::new(x, y)) {
                    print!("{}|", tile);
                } else {
                    print!("?");
                }
            }
            println!();
        }
    }

    /// ゲームが終了しているか判定します。
    /// ゲームが終了していたらtrueを返します。
    /// 
    pub fn is_end(&self) -> bool {
        self.is_end_nosquare() || self.is_end_one_color()
    }

    /// ゲームの勝者を返却します。
    /// ゲームが終了していない場合はNoneが返却されます。
    /// 
    pub fn get_winner(&self) -> Option<PieceType> {
        // 白または黒一色か？
        let mut white_count = 0;
        let mut black_count = 0;

        for tile_type in self.tiles.iter_tile() {
            match tile_type {
                TileType::Piece(PieceType::White) => white_count += 1,
                TileType::Piece(PieceType::Black) => black_count += 1,
                _ => (),
            }
        }

        match white_count.cmp(&black_count) {
            Ordering::Greater => Some(PieceType::White),
            Ordering::Less => Some(PieceType::Black),
            Ordering::Equal => None,
        }
    }

    /// 駒が打てない場合にtrueを返します。
    /// 
    pub fn is_skip(&self, piece_type: PieceType) -> bool {
        let mut result = true;

        for point in self.tiles.iter_point() {
            if self.can_put_piece(piece_type, &point) {
                result = false;
                break;
            }
        }

        // 置ける場所が無い場合
        result
    }

    fn is_end_nosquare(&self) -> bool {
        // 全部埋まっているか？
        // つまり、Pieceを置けるSquareが存在していない場合trueを返す。
        !self.tiles.iter_tile().any(|&t| t == TileType::Square)
    }

    fn is_end_one_color(&self) -> bool {
        // 白または黒一色か？
        let mut white_count = 0;
        let mut black_count = 0;

        for tile in self.tiles.iter_tile() {
            match tile {
                TileType::Piece(PieceType::White) => white_count += 1,
                TileType::Piece(PieceType::Black) => black_count += 1,
                _ => (),
            }
            if (white_count != 0) && (black_count != 0) {
                return false;
            }
        }

        true
    }

    /// 置こうとした場所に駒が置けるか判定します。
    /// `Point`構造体の位置に駒が置ける場合はtrueを返します。
    /// 
    pub fn can_put_piece(&self, piece_type: PieceType, point: &Point) -> bool {
        // 置こうとした場所は有効な場所か？
        if self.is_square(point) {
            // 隣接してない所は置けない
            if self.is_next_to_piece(point.x, point.y) {
                // 返せない所は置けない
                let change_piece_line = self.can_change_piece_line(piece_type, point.x, point.y);
                !change_piece_line.is_empty()
            } else {
                false
            }
        } else {
            false
        }
    }

    /// 駒を置きます。
    /// 置いたコマによって既に置かれた駒が変わるため、新しい`Board`構造体が返却されます。
    /// 
    pub fn put_piece(self, piece_type: PieceType, point: Point) -> Board {
        // 置こうとした場所は有効な場所か？
        let lines = if self.is_square(&point) {
            if self.is_next_to_piece(point.x, point.y) {
                self.can_change_piece_line(piece_type, point.x, point.y)
            } else {
                Vec::new()
            }
        } else {
            Vec::new()
        };
        
        let mut board = self;

        if !lines.is_empty() {
            board.tiles.set(point, TileType::Piece(piece_type));

            for line in lines {
                let tiles = Board::can_change_tiles(&line, piece_type);
                let tiles = tiles.iter().map(|v| v.0).collect::<Vec<_>>();
                board = board.change_tiles(&tiles, piece_type);
            }
        }

        board
    }

    fn is_square(&self, point: &Point) -> bool {
        matches!(self.get_tile(point), Some(TileType::Square))
    }

    fn is_next_to_piece(&self, x: usize, y: usize) -> bool {
        let target = [
            self.get_tile((x - 1, y - 1)), self.get_tile((x, y - 1)), self.get_tile((x + 1, y - 1)),
            self.get_tile((x - 1, y)), self.get_tile((x + 1, y)),
            self.get_tile((x - 1, y + 1)), self.get_tile((x, y + 1)), self.get_tile((x + 1, y + 1)),
            ];

        // 隣接する場所に駒があるか
        target.iter().any(|t| matches!(t, Some(TileType::Piece(_))))
    }

    fn can_change_piece_line(&self, piece_type: PieceType, x: usize, y: usize) -> Vec<Vec<(Point, PieceType)>> {

        let targets = [
            (-1_isize, -1_isize), (-1, 0), (-1, 1),
            (0, -1), (0, 1),
            (1, -1), (1, 0), (1, 1),
        ];

        let mut can_change_lines: Vec<Vec<(Point, PieceType)>> = Vec::new();

        for target in targets {
            let extract_line = self.extract_line(x, y, target.0, target.1);
            let change_tiles = Board::can_change_tiles(&extract_line, piece_type);
            if !change_tiles.is_empty() {
                can_change_lines.push(extract_line);
            }
        }
    
        can_change_lines
    }

    fn extract_line(&self, x: usize, y:usize, dx: isize, dy: isize) -> Vec<(Point, PieceType)> {
        let mut result: Vec<(Point, PieceType)> = Vec::new();
        let mut index = 1;
        loop {
            let xt = x as isize + (dx * index);
            let yt = y as isize + (dy * index);
            let tile = self.get_tile((xt, yt));

            match tile {
                Some(TileType::Piece(piece_type)) => result.push((Point::new(xt as usize, yt as usize), *piece_type)),
                _ => break,
            }
            index += 1;
        }

        result
    }

    fn change_tiles(self, change_points: &[Point], piece_type: PieceType) -> Board {
        let mut tiles = self.tiles;

        for point in change_points {
            tiles.set(*point, TileType::Piece(piece_type));
        }

        Board {
            tiles,
            ..self
        }
    }

    fn can_change_tiles(line: &[(Point, PieceType)], piece_type: PieceType) -> Vec<(Point, PieceType)> {
        let mut result: Vec<(Point, PieceType)> = Vec::new();
        let mut is_opposed = false;
        let mut can_change = false;
    
        for v in line {
            let this_type = v.1;

            if is_opposed {
                if this_type == piece_type {
                    // 以前に反対の駒が存在していて、自分の駒と同じタイプが出現したら終了
                    can_change = true;
                    break;
                } else {
                    // 以前に反対の駒が存在していて、自分の駒と違うタイプが出現
                    result.push((v.0, v.1));
                }
            } else {
                if this_type == piece_type {
                    // 初回の判定で自分と同じタイプの駒が出た場合
                    break;
                } else {
                    // 自分の駒と別のタイプが出たらフラグを変えて継続
                    is_opposed = true;
                    result.push((v.0, v.1));
                }
            }
        }
    
        if can_change {
            result
        } else {
            Vec::new()
        }
    }
}


trait XYPoint<T> {
    fn get_tile(&self, point: T) -> Option<&TileType>;
}

impl XYPoint<&Point> for Board {
    fn get_tile(&self, point: &Point) -> Option<&TileType> {
        self.tiles.get(point)
    }
}

impl XYPoint<(isize, isize)> for Board {
    fn get_tile(&self, (x, y): (isize, isize)) -> Option<&TileType> {
        if y < 0 || x < 0 {
            None
        } else {
            self.get_tile(&Point::new(x as usize, y as usize))
        }
    }
}

impl XYPoint<(usize, usize)> for Board {
    fn get_tile(&self, (x, y): (usize, usize)) -> Option<&TileType> {
        self.tiles.get(&Point::new(x, y))
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use crate::piece_type::PieceType;

    #[test]
    fn test_ok_is_skip() {
        let mut board = Board::new(8, 8);

        //  |0|1|2|3|4|5|6|7|8|9|
        // 0|×|×|×|×|×|×|×|×|×|×|
        // 1|×| | | | |●|●|●|●|×|
        // 2|×| | | | |●|●|●|●|×|
        // 3|×| | | | |●|●|●|●|×|
        // 4|×| | | |○|●|●|●|●|×|
        // 5|×| | | |●|●|●|●|●|×|
        // 6|×| | | |●|●|●|●|●|×|
        // 7|×| | | |●|●|●|●|●|×|
        // 8|×| | | |●|●|●|●|●|×|
        // 9|×|×|×|×|×|×|×|×|×|×|

        board.tiles.set(Point::new(4, 5), TileType::Piece(PieceType::Black));
        board.tiles.set(Point::new(4, 6), TileType::Piece(PieceType::Black));
        board.tiles.set(Point::new(4, 7), TileType::Piece(PieceType::Black));
        board.tiles.set(Point::new(4, 8), TileType::Piece(PieceType::Black));
        for x in 5..=8 {
            for y in 1..=8 {
                board.tiles.set(Point::new(x, y), TileType::Piece(PieceType::Black));
            }
        }

        // 白は置けない
        assert_eq!(board.is_skip(PieceType::White), true);
    }

    #[test]
    fn test_ng_is_skip() {
        let mut board = Board::new(8, 8);

        //  |0|1|2|3|4|5|6|7|8|9|
        // 0|×|×|×|×|×|×|×|×|×|×|
        // 1|×| | |●|●|●|●|●|●|×|
        // 2|×| |●|●|●|●|●|●|●|×|
        // 3|×| |●|●|●|●|●|○|●|×|
        // 4|×| |●|●|●|●|●|○|●|×|
        // 5|×| | |●|●|●|○|○|●|×|
        // 6|×| | |●| |●|●|○| |×|
        // 7|×| | | | |●|●|○|●|×|
        // 8|×| | | | | | | | |×|
        // 9|×|×|×|×|×|×|×|×|×|×|
        
        for x in 3..=8 {
            board.tiles.set(Point::new(x, 1), TileType::Piece(PieceType::Black));
            board.tiles.set(Point::new(x, 5), TileType::Piece(PieceType::Black));
            board.tiles.set(Point::new(x, 6), TileType::Piece(PieceType::Black));
        }
        for x in 2..=8 {
            board.tiles.set(Point::new(x, 2), TileType::Piece(PieceType::Black));
            board.tiles.set(Point::new(x, 3), TileType::Piece(PieceType::Black));
            board.tiles.set(Point::new(x, 4), TileType::Piece(PieceType::Black));
        }
        for x in 5..=8 {
            board.tiles.set(Point::new(x, 7), TileType::Piece(PieceType::Black));
        }
        for y in 3..=7 {
            board.tiles.set(Point::new(7, y), TileType::Piece(PieceType::White));
        }
        board.tiles.set(Point::new(6, 5), TileType::Piece(PieceType::White));
        board.tiles.set(Point::new(7, 5), TileType::Piece(PieceType::White));
        board.tiles.set(Point::new(4, 6), TileType::Square);
        board.tiles.set(Point::new(8, 6), TileType::Square);

        // 黒は置ける
        assert_eq!(false, board.is_skip(PieceType::Black));
    }

    #[test]
    fn test_can_put_piece() {
        let board = Board::new(8, 8);

        //  |0|1|2|3|4|5|6|7|8|9|
        // 0|×|×|×|×|×|×|×|×|×|×|
        // 1|×| | | | | | | | |×|
        // 2|×| | | | | | | | |×|
        // 3|×| | | | | | | | |×|
        // 4|×| | | |○|●| | | |×|
        // 5|×| | | |●|○| | | |×|
        // 6|×| | | | | | | | |×|
        // 7|×| | | | | | | | |×|
        // 8|×| | | | | | | | |×|
        // 9|×|×|×|×|×|×|×|×|×|×|

        // 1,1 に白も黒も置けない
        assert_eq!(board.can_put_piece(PieceType::White, &Point::new(1, 1)), false);
        assert_eq!(board.can_put_piece(PieceType::Black, &Point::new(1, 1)), false);

        // 3,4 に白は置けない、黒は置ける
        assert_eq!(board.can_put_piece(PieceType::White, &Point::new(3, 4)), false);
        assert_eq!(board.can_put_piece(PieceType::Black, &Point::new(3, 4)), true);

        // 5,3 に白は置ける、黒は置けない
        assert_eq!(board.can_put_piece(PieceType::White, &Point::new(5, 3)), true);
        assert_eq!(board.can_put_piece(PieceType::Black, &Point::new(5, 3)), false);
    }

    #[test]
    fn test_can_put_piece2() {
        let mut board = Board::new(8, 8);

        //  |0|1|2|3|4|5|6|7|8|9|
        // 0|×|×|×|×|×|×|×|×|×|×|
        // 1|×| | |●|●|●|●|●|●|×|
        // 2|×| |●|●|●|●|●|●|●|×|
        // 3|×| |●|●|●|●|●|○|●|×|
        // 4|×| |●|●|●|●|●|○|●|×|
        // 5|×| | |●|●|●|○|○|●|×|
        // 6|×| | |●| |●|●|○| |×|
        // 7|×| | | | |●|●|○|●|×|
        // 8|×| | | | | | | | |×|
        // 9|×|×|×|×|×|×|×|×|×|×|
        
        for x in 3..=8 {
            board.tiles.set(Point::new(x, 1), TileType::Piece(PieceType::Black));
            board.tiles.set(Point::new(x, 5), TileType::Piece(PieceType::Black));
            board.tiles.set(Point::new(x, 6), TileType::Piece(PieceType::Black));
        }
        for x in 2..=8 {
            board.tiles.set(Point::new(x, 2), TileType::Piece(PieceType::Black));
            board.tiles.set(Point::new(x, 3), TileType::Piece(PieceType::Black));
            board.tiles.set(Point::new(x, 4), TileType::Piece(PieceType::Black));
        }
        for x in 5..=8 {
            board.tiles.set(Point::new(x, 7), TileType::Piece(PieceType::Black));
        }
        for y in 3..=7 {
            board.tiles.set(Point::new(7, y), TileType::Piece(PieceType::White));
        }
        board.tiles.set(Point::new(6, 5), TileType::Piece(PieceType::White));
        board.tiles.set(Point::new(7, 5), TileType::Piece(PieceType::White));
        board.tiles.set(Point::new(4, 6), TileType::Square);
        board.tiles.set(Point::new(8, 6), TileType::Square);

        assert_eq!(true, board.can_put_piece(PieceType::Black, &Point::new(8,6)));
    }

    #[test]
    fn test_get_winner_none() {
        let board = Board::new(8, 8);

        //  |0|1|2|3|4|5|6|7|8|9|
        // 0|×|×|×|×|×|×|×|×|×|×|
        // 1|×| | | | | | | | |×|
        // 2|×| | | | | | | | |×|
        // 3|×| | | | | | | | |×|
        // 4|×| | | |○|●| | | |×|
        // 5|×| | | |●|○| | | |×|
        // 6|×| | | | | | | | |×|
        // 7|×| | | | | | | | |×|
        // 8|×| | | | | | | | |×|
        // 9|×|×|×|×|×|×|×|×|×|×|

        // 同枚数は勝者なし
        assert_eq!(None, board.get_winner());
    }

    #[test]
    fn test_get_winner_white() {
        let mut board = Board::new(8, 8);
        board.tiles.set(Point::new(4, 6), TileType::Piece(PieceType::White));

        //  |0|1|2|3|4|5|6|7|8|9|
        // 0|×|×|×|×|×|×|×|×|×|×|
        // 1|×| | | | | | | | |×|
        // 2|×| | | | | | | | |×|
        // 3|×| | | | | | | | |×|
        // 4|×| | | |○|●| | | |×|
        // 5|×| | | |●|○| | | |×|
        // 6|×| | | |○| | | | |×|
        // 7|×| | | | | | | | |×|
        // 8|×| | | | | | | | |×|
        // 9|×|×|×|×|×|×|×|×|×|×|

        // 白の勝ち
        assert_eq!(PieceType::White, board.get_winner().unwrap());
    }

    #[test]
    fn test_get_winner_black() {
        let mut board = Board::new(8, 8);
        board.tiles.set(Point::new(4, 6), TileType::Piece(PieceType::Black));
        
        //  |0|1|2|3|4|5|6|7|8|9|
        // 0|×|×|×|×|×|×|×|×|×|×|
        // 1|×| | | | | | | | |×|
        // 2|×| | | | | | | | |×|
        // 3|×| | | | | | | | |×|
        // 4|×| | | |○|●| | | |×|
        // 5|×| | | |●|○| | | |×|
        // 6|×| | | |●| | | | |×|
        // 7|×| | | | | | | | |×|
        // 8|×| | | | | | | | |×|
        // 9|×|×|×|×|×|×|×|×|×|×|

        // 白の勝ち
        assert_eq!(PieceType::Black, board.get_winner().unwrap());
    }

    #[test]
    fn test_is_end_nosquare_true() {
        let mut board = Board::new(8, 8);

        //  |0|1|2|3|4|5|6|7|8|9|
        // 0|×|×|×|×|×|×|×|×|×|×|
        // 1|×|●|●|●|●|●|●|●|●|×|
        // 2|×|●|●|●|●|●|●|●|●|×|
        // 3|×|●|●|●|●|●|●|●|●|×|
        // 4|×|●|●|●|●|●|●|●|●|×|
        // 5|×|●|●|●|●|●|●|●|●|×|
        // 6|×|●|●|●|●|●|●|●|●|×|
        // 7|×|●|●|●|●|●|●|●|●|×|
        // 8|×|●|●|●|●|●|●|●|●|×|
        // 9|×|×|×|×|×|×|×|×|×|×|

        for x in 1..=8 {
            for y in 1..=8 {
                board.tiles.set(Point::new(x, y), TileType::Piece(PieceType::Black));
            }
        }

        // タイルが無い
        assert_eq!(true, board.is_end_nosquare());
    }

    #[test]
    fn test_is_end_nosquare_false() {
        let mut board = Board::new(8, 8);

        //  |0|1|2|3|4|5|6|7|8|9|
        // 0|×|×|×|×|×|×|×|×|×|×|
        // 1|×|●|●|●|●|●|●|●|●|×|
        // 2|×|●|●|●|●|●|●|●|●|×|
        // 3|×|●|●|●|●|●|●|●|●|×|
        // 4|×|●|●|●|●|●|●|●|●|×|
        // 5|×|●|●|●|●|●|●|●|●|×|
        // 6|×|●|●|●|●|●|●|●|●|×|
        // 7|×|●|●|●|●|●|●|●|●|×|
        // 8|×|●|●|●|●|●|●|●|●|×|
        // 9|×|×|×|×|×|×|×|×|×|×|

        for x in 1..=8 {
            for y in 1..=8 {
                board.tiles.set(Point::new(x, y), TileType::Piece(PieceType::Black));
            }
        }
        board.tiles.set(Point::new(1, 1), TileType::Square);

        // タイルが無い
        assert_eq!(false, board.is_end_nosquare());
    }

    #[test]
    fn test_is_end_one_color_true() {
        let mut board = Board::new(8, 8);
        
        //  |0|1|2|3|4|5|6|7|8|9|
        // 0|×|×|×|×|×|×|×|×|×|×|
        // 1|×| | | | | | | | |×|
        // 2|×| | | | | | | | |×|
        // 3|×| | | | | | | | |×|
        // 4|×| | | |●|●| | | |×|
        // 5|×| | | |●|●| | | |×|
        // 6|×| | | | | | | | |×|
        // 7|×| | | | | | | | |×|
        // 8|×| | | | | | | | |×|
        // 9|×|×|×|×|×|×|×|×|×|×|

        // 黒一色
        board.tiles.set(Point::new(4, 4), TileType::Piece(PieceType::Black));
        board.tiles.set(Point::new(5, 5), TileType::Piece(PieceType::Black));
        assert_eq!(true, board.is_end_one_color());

        // 白一色
        board.tiles.set(Point::new(4, 4), TileType::Piece(PieceType::White));
        board.tiles.set(Point::new(4, 5), TileType::Piece(PieceType::White));
        board.tiles.set(Point::new(5, 4), TileType::Piece(PieceType::White));
        board.tiles.set(Point::new(5, 5), TileType::Piece(PieceType::White));
        assert_eq!(true, board.is_end_one_color());
    }

    #[test]
    fn test_is_end_one_color_false() {
        let mut board = Board::new(8, 8);
        board.tiles.set(Point::new(5, 5), TileType::Piece(PieceType::Black));
        
        //  |0|1|2|3|4|5|6|7|8|9|
        // 0|×|×|×|×|×|×|×|×|×|×|
        // 1|×| | | | | | | | |×|
        // 2|×| | | | | | | | |×|
        // 3|×| | | | | | | | |×|
        // 4|×| | | |○|●| | | |×|
        // 5|×| | | |●|●| | | |×|
        // 6|×| | | | | | | | |×|
        // 7|×| | | | | | | | |×|
        // 8|×| | | | | | | | |×|
        // 9|×|×|×|×|×|×|×|×|×|×|

        // どちらか一色でもない
        assert_eq!(false, board.is_end_one_color());
    }

    #[test]
    fn test_is_next_to_piece() {
        let mut board = Board::new(8, 8);

        assert_eq!(false, board.is_next_to_piece(2, 4));
        assert_eq!(true, board.is_next_to_piece(3, 4));
        
        board.tiles.set(Point::new(3, 4), TileType::Piece(PieceType::White));
        
        assert_eq!(false, board.is_next_to_piece(1, 4));
        assert_eq!(true, board.is_next_to_piece(2, 4));
    }

    #[test]
    fn test_can_change_piece_line() {
        let board = Board::new(8, 8);

        assert_eq!(false, board.can_change_piece_line(PieceType::Black, 2, 4).len() > 0);
        assert_eq!(true, board.can_change_piece_line(PieceType::Black, 3, 4).len() > 0);

        let board = board.put_piece(PieceType::Black, Point::new(3, 4));

        assert_eq!(false, board.can_change_piece_line(PieceType::White, 3, 2).len() > 0);
        assert_eq!(true, board.can_change_piece_line(PieceType::White, 3, 3).len() > 0);
    }
}