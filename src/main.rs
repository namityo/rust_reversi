use std::io;

mod board;
mod tile;

fn main() {
    let mut board = board::Board::new(8, 8);

    let mut piece_type = tile::PieceType::Black;

    // 画面表示
    board.print();

    loop {
        if board.is_end() {
            println!("ゲーム終了");
            break;
        }

        println!("{:?} の番です", piece_type);

        let point = match input_xy() {
            Ok(point) => point,
            Err(err) => {
                println!("{}", err);
                continue;
            },
        };

        if board.can_put_piece(piece_type, point) {
            board = board.put_piece(piece_type, point);
            piece_type = piece_type.change();

            println!("{:?} に置きました", point);
            board.print();
        } else {
            println!("{:?} には置けません", point);
        }
    }
}

fn input_xy() -> Result<board::Point, String> {
    let x = input_axes("x")?;
    let y = input_axes("y")?;
    return Ok(board::Point::new(x, y));
}

fn input_axes(axes: &str) -> Result<usize, String> {
    println!("{}座標を入力してください", axes);

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

    return Ok(val);
}