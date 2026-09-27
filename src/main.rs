use std::io;


mod board;

use board::Board;
use board::Direction::*;

fn init() -> Board{
    let mut board = Board::new();
    board.add_rand_tile();
    board
}

fn main() {
    let mut board = init();
    let mut running = true;
    let mut input_string = String::new();

    while running{
        board.print_board();
        println!("Input a direction: ");
        input_string.clear(); // clear to avoid adding to the string

        // Get the stdin from the user, and put it in read_string    
        io::stdin().read_line(&mut input_string).unwrap();

        input_string = input_string.trim().to_lowercase();

        if input_string == "up"{
            running = board.make_move(Up);
        }else if input_string == "down"{
            running = board.make_move(Down);
        }else if input_string == "left"{
            running = board.make_move(Left);
        }else if input_string == "right"{
            running = board.make_move(Right);
        }else if input_string == "q"{
            break;
        }


        if !running {
            println!("you lost :(");
        }
    }
    // for i in 0..16{
    //     board.set_ind(i, i);
    // }
    // println!("{}", board.get(2, 3));
    // println!("board: ");
    // board.print_board();

    // board.set_ind(2, 7);
    // board.set_ind(1, 7);
    // board.set_ind(0, 7);
    // board.set_ind(3, 7);
    // board.set_ind(6, 7);
    // board.set_ind(10, 4);
    // board.set_ind(14, 5);
    // // println!("{}", board.get_ind(2));
    // println!("next board:");
    // board.print_board();

    // board.make_move(Direction::Right);
    // // println!("{}, {}", board.get(2, 3), board.get(1, 3));
    // println!("another board:");
    // board.print_board();
}
