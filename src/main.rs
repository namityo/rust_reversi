use std::io;

mod board;
mod tile;

fn main() {
    let mut board = board::Board::new(8, 8);

    let mut piece_type = tile::PieceType::Black;

    // 画面表示
    board.print();

    loop {
        println!("{:?} の番です", piece_type);

        let mut x = String::new();
        let mut y = String::new();
        println!("x座標を入力してください");
        match io::stdin().read_line(&mut x) {
            Ok(_) => (),
            Err(error) => println!("error: {}", error),
        }
        let x: usize = match x.trim().parse() {
            Ok(v) => v,
            Err(_) => {
                println!("xの値が不正です");
                continue;
            },
        };
        println!("y座標を入力してください");
        match io::stdin().read_line(&mut y) {
            Ok(_) => (),
            Err(error) => println!("error: {}", error),
        }
        let y: usize = match y.trim().parse() {
            Ok(v) => v,
            Err(_) => {
                println!("yの値が不正です");
                continue;
            }
        };
        let point = board::Point::new(x, y);

        if board.can_put_piece(piece_type, point) {
            board = board.put_piece(piece_type, point);
            piece_type = tile::PieceType::White;

            println!("{:?} に置きました", point);
            board.print();
        } else {
            println!("{:?} には置けません", point);
        }
    }
}
