use std::io::{self, Write};

extern crate rust_reversi;
use rust_reversi::board::Board;
use rust_reversi::piece_type::PieceType;
use rust_reversi::point::Point;

fn main() {
    let mut board = Board::new(8, 8);

    let mut piece_type = PieceType::Black;

    // 画面表示
    board.print();

    loop {
        if board.is_end() {
            break;
        }

        if board.is_skip(piece_type) {
            println!("{} の打てる場所がありません", piece_type);
            piece_type = piece_type.change();
        }

        println!("{} の番です", piece_type);

        let point = match input_xy() {
            Ok(point) => point,
            Err(err) => {
                println!("{}", err);
                continue;
            },
        };

        if board.can_put_piece(piece_type, &point) {
            board = board.put_piece(piece_type, point);
            piece_type = piece_type.change();

            println!("{} に置きました", point);
            board.print();
        } else {
            println!("{} には置けません", point);
        }
    }

    println!("ゲーム終了");

    if let Some(piece_type) = board.get_winner() {
        println!("{} の勝利!", piece_type)
    } else {
        println!("同点")
    }
}

fn input_xy() -> Result<Point, String> {
    let x = input_axes("x")?;
    let y = input_axes("y")?;
    Ok(Point::new(x, y))
}

fn input_axes(axes: &str) -> Result<usize, String> {
    print!("{}座標を入力してください : ", axes);
    io::stdout().flush().unwrap();

    let mut val = String::new();
    match io::stdin().read_line(&mut val) {
        Ok(_) => (),
        Err(error) => println!("error: {}", error),
    }
    let val: usize = match val.trim().parse() {
        Ok(v) => v,
        Err(_) => {
            return Err(format!("{}の値が不正です", axes));
        }
    };

    Ok(val)
}