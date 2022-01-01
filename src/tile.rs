use crate::point::Point;
use crate::piece_type::PieceType;
use crate::tile_type::TileType;

use std::collections::HashMap;

pub struct Tile {
    tiles: HashMap<Point, TileType>,
    x_size: usize,
    y_size: usize,
}

pub struct TileIter<'a> {
    x: usize,
    y: usize,
    tile: &'a Tile
}

pub struct PointIter {
    x: usize,
    y: usize,
    x_size: usize,
    y_size: usize,
}

impl Tile {
    pub fn new(x_size: usize, y_size: usize) -> Tile {
        Tile {
            tiles: Tile::initialize_tiles(x_size, y_size),
            x_size,
            y_size,
        }
    }

    pub fn get(&self, k: &Point) -> Option<&TileType> {
        self.tiles.get(k)
    }

    pub fn set(&mut self, point: Point, tile_type: TileType) {
        self.tiles.insert(point, tile_type);
    }

    pub fn iter_tile(&self) -> TileIter {
        TileIter {
            x: 0,
            y: 0,
            tile: self,
        }
    }

    pub fn iter_point(&self) -> PointIter {
        PointIter {
            x: 0,
            y: 0,
            x_size: self.x_size,
            y_size: self.y_size,
        }
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

        tiles
    }
}

impl<'a> Iterator for TileIter<'a> {
    type Item = &'a TileType;

    fn next(&mut self) -> Option<Self::Item> {
        let point = Point::new(self.x, self.y);

        if self.tile.x_size + 2 > self.x + 1 {
            self.x += 1;
        } else {
            self.x = 0;
            if self.tile.y_size + 2 > self.y + 1 {
                self.y += 1;
            } else {
                return None;
            }
        }

        self.tile.get(&point)
    }
}

impl Iterator for PointIter {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        let point = Point::new(self.x, self.y);

        if self.x_size + 2 > self.x + 1 {
            self.x += 1;
        } else {
            self.x = 0;
            if self.y_size + 2 > self.y + 1 {
                self.y += 1;
            } else {
                return None;
            }
        }

        Some(point)
    }
}
