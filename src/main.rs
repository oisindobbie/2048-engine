mod board;

use board::{Board, Direction};

fn main() {
    let mut board = Board::new();
    // for i in 0..16{
    //     board.set_ind(i, i);
    // }
    // println!("{}", board.get(2, 3));
    // println!("board: ");
    // board.print_board();

    board.set_ind(2, 7);
    board.set_ind(1, 7);
    board.set_ind(0, 7);
    board.set_ind(3, 7);
    board.set_ind(6, 7);
    board.set_ind(10, 4);
    board.set_ind(14, 4);
    // println!("{}", board.get_ind(2));
    println!("next board:");
    board.print_board();

    board.make_move(Direction::Up);
    // println!("{}, {}", board.get(2, 3), board.get(1, 3));
    println!("another board:");
    board.print_board();
}
