use crate::tile::TileType;
use crate::tile::PieceType;
use std::collections::HashMap;

pub struct Board {
    tiles: HashMap<Point, TileType>,
    x_size: usize,
    y_size: usize,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Point {
    x: usize,
    y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Point {
        return Point {
            x,
            y,
        }
    }
}

impl Board {
    pub fn new(x_size: usize, y_size: usize) -> Board {
        let board = Board {
            tiles: Board::initialize_tiles(x_size, y_size),
            x_size,
            y_size,
        };

        return board;
    }

    pub fn print(&self) {
        println!(" |0|1|2|3|4|5|6|7|8|9|");
        for y in 0..=(self.y_size + 1) {
            print!("{}|", y);
            for x in 0..=(self.x_size + 1) {
                if let Some(tile) = self.tiles.get(&Point::new(x, y)) {
                    print!("{}|", tile.to_string());
                } else {
                    print!("?");
                }
            }
            println!();
        }
    }

    pub fn is_end(&self) -> bool {
        return self.is_end_nosquare() || self.is_end_one_color();
    }

    pub fn get_winner(&self) -> Option<PieceType> {
        // 白または黒一色か？
        let mut white_count = 0;
        let mut black_count = 0;

        for (_, tile) in self.tiles.iter() {
            match tile {
                TileType::Piece(t) => match t {
                    PieceType::White => white_count += 1,
                    PieceType::Black => black_count += 1,
                },
                _ => (),
            }
        }
        if white_count > black_count {
            return Some(PieceType::White);
        } if white_count < black_count {
            return Some(PieceType::Black);
        } else {
            return None;
        }
    }

    pub fn is_skip(&self, piece_type: PieceType) -> bool {
        for (point, _) in self.tiles.iter() {
            if self.can_put_piece(piece_type, point) {
                return false;
            }
        }

        // 置ける場所が無い場合
        return true;
    }

    fn is_end_nosquare(&self) -> bool {
        // 全部埋まっているか？
        for (_, tile) in self.tiles.iter() {
            match tile {
                TileType::Square => return false,
                _ => (),
            }
        }

        return true;
    }

    fn is_end_one_color(&self) -> bool {
        // 白または黒一色か？
        let mut white_count = 0;
        let mut black_count = 0;

        for (_, tile) in self.tiles.iter() {
            match tile {
                TileType::Piece(t) => match t {
                    PieceType::White => white_count += 1,
                    PieceType::Black => black_count += 1,
                },
                _ => (),
            }
            if (white_count != 0) && (black_count != 0) {
                return false;
            }
        }

        return true;
    }

    fn initialize_tiles(x_size: usize, y_size: usize) -> HashMap<Point, TileType> {
        let mut tiles: HashMap<Point, TileType> = HashMap::new();
    
        // 最初と最後の列行は全部空
        for x in 0..=(x_size + 1) {
            tiles.insert(Point::new(x, 0), TileType::None);
            tiles.insert(Point::new(x, y_size + 1), TileType::None);
        }
        for y in 0..=(y_size + 1) {
            tiles.insert(Point::new(0, y), TileType::None);
            tiles.insert(Point::new(x_size + 1, y), TileType::None);
        }


        for y in 1..=y_size {
            for x in 1..=x_size {
                tiles.insert(Point::new(x, y), TileType::Square);
            }
        }
        
        // 中心に白と黒を配置
        let x_center = x_size / 2;
        let y_center = y_size / 2;
        tiles.insert(Point::new(x_center, y_center), TileType::Piece(PieceType::White));
        tiles.insert(Point::new(x_center + 1, y_center), TileType::Piece(PieceType::Black));
        tiles.insert(Point::new(x_center, y_center + 1), TileType::Piece(PieceType::Black));
        tiles.insert(Point::new(x_center + 1, y_center + 1), TileType::Piece(PieceType::White));

        return tiles;
    }

    pub fn can_put_piece(&self, piece_type: PieceType, point: &Point) -> bool {
        // 置こうとした場所は有効な場所か？
        return if self.is_square(point.x, point.y) {
            // 隣接してない所は置けない
            if self.is_next_to_piece(point.x, point.y) {
                // 返せない所は置けない
                if self.can_change_piece_line(piece_type, point.x, point.y).len() > 0 {
                    true
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        };
    }

    pub fn put_piece(self, piece_type: PieceType, point: Point) -> Board {
        // 置こうとした場所は有効な場所か？
        let lines = if self.is_square(point.x, point.y) {
            if self.is_next_to_piece(point.x, point.y) {
                self.can_change_piece_line(piece_type, point.x, point.y)
            } else {
                Vec::new()
            }
        } else {
            Vec::new()
        };
        
        let mut board = self;

        if lines.len() > 0 {
            board.tiles.insert(point, TileType::Piece(piece_type));

            for line in lines {
                let tiles = Board::can_change_tiles(&line, piece_type);
                let tiles = tiles.iter().map(|v| v.0).collect::<Vec<_>>();
                board = board.change_tiles(&tiles, piece_type);
            }
        }

        return board;
    }

    fn is_square(&self, x: usize, y: usize) -> bool {
        return match self.get_tile((x, y)) {
            Some(tile) => match tile {
                TileType::Square => true,
                _ => false,
            },
            _ => false,
        };
    }

    fn is_next_to_piece(&self, x: usize, y: usize) -> bool {
        let target = [
            self.get_tile((x - 1, y - 1)), self.get_tile((x, y - 1)), self.get_tile((x + 1, y - 1)),
            self.get_tile((x - 1, y)), self.get_tile((x + 1, y)),
            self.get_tile((x - 1, y + 1)), self.get_tile((x, y + 1)), self.get_tile((x + 1, y + 1)),
            ];

        for t in target {
            match t {
                Some(tile) => {
                    match tile {
                        TileType::Piece(_) => return true,
                        _ => (),
                    }
                },
                _ => (),
            }
        }

        return false;
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
            if Board::can_change_tiles(&extract_line, piece_type).len() > 0 {
                can_change_lines.push(extract_line);
            }
        }
    
        return can_change_lines;
    }

    fn extract_line(&self, x: usize, y:usize, dx: isize, dy: isize) -> Vec<(Point, PieceType)> {
        let mut result: Vec<(Point, PieceType)> = Vec::new();
        let mut index = 1;
        loop {
            let xt = x as isize + (dx * index);
            let yt = y as isize + (dy * index);
            let tile = self.get_tile((xt, yt));

            match tile {
                Some(tile) => match tile {
                    TileType::Piece(piece_type) => match piece_type {
                        PieceType::Black => result.push((Point::new(xt as usize, yt as usize), PieceType::Black)),
                        PieceType::White => result.push((Point::new(xt as usize, yt as usize), PieceType::White)),
                    },
                    _ => break,
                },
                _ => break,
            }
            index += 1;
        }

        return result;
    }

    fn change_tiles(self, change_points: &Vec<Point>, piece_type: PieceType) -> Board {
        let mut tiles = self.tiles;

        for point in change_points {
            tiles.insert(*point, TileType::Piece(piece_type));
        }

        return Board {
            tiles: tiles,
            ..self
        };
    }

    fn can_change_tiles(line: &Vec<(Point, PieceType)>, piece_type: PieceType) -> Vec<(Point, PieceType)> {
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
    
        return if can_change {
            result
        } else {
            Vec::new()
        };
    }
}


trait XYPoint<T> {
    fn get_tile(&self, point: T) -> Option<&TileType>;
}

impl XYPoint<&Point> for Board {
    fn get_tile(&self, point: &Point) -> Option<&TileType> {
        return self.tiles.get(point);
    }
}

impl XYPoint<(isize, isize)> for Board {
    fn get_tile(&self, (x, y): (isize, isize)) -> Option<&TileType> {
        if y < 0 {
            return None;
        }
        if x < 0 {
            return None;
        }
        return self.get_tile(&Point::new(x as usize, y as usize));
    }
}

impl XYPoint<(usize, usize)> for Board {
    fn get_tile(&self, (x, y): (usize, usize)) -> Option<&TileType> {
        return self.tiles.get(&Point::new(x, y));
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use crate::tile::PieceType;

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

        board.tiles.insert(Point::new(4, 5), TileType::Piece(PieceType::Black));
        board.tiles.insert(Point::new(4, 6), TileType::Piece(PieceType::Black));
        board.tiles.insert(Point::new(4, 7), TileType::Piece(PieceType::Black));
        board.tiles.insert(Point::new(4, 8), TileType::Piece(PieceType::Black));
        for x in 5..=8 {
            for y in 1..=8 {
                board.tiles.insert(Point::new(x, y), TileType::Piece(PieceType::Black));
            }
        }

        // 白は置けない
        assert_eq!(board.is_skip(PieceType::White), true);
    }

    #[test]
    fn test_ng_is_skip() {
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

        // 白は置ける
        assert_eq!(board.is_skip(PieceType::White), false);
    }
}