extern crate qmetaobject;
use qmetaobject::*;

use std::ffi::CStr;

mod implementation;



/*
fn render_player(p: BoardSquare) {
    match p {
        BoardSquare::Empty => print!(" "),
        BoardSquare::Circle => print!("O"),
        BoardSquare::Cross => print!("X")
    }
} 

fn render_board(board: &[[BoardSquare; 3]; 3]) {
    println!("    a\t    b\t    c");
    println!("----------------------------");

    for row in 1..=3 {
        println!(" |\t|\t|\t|");
        print!("{}|   ", row);
        render_player(board[row-1][0]);
        print!("  |   ");
        render_player(board[row-1][1]);
        print!("   |   ");
        render_player(board[row-1][2]);
        println!("   |");
        println!(" |\t|\t|\t|");
        println!("----------------------------");
    }

}*/


/*
fn main() {
    let mut board: [[BoardSquare; 3]; 3] = [[BoardSquare::Empty; 3]; 3];
    let mut playing = true;

    while playing {
        render_board(&board);

        let pb = possible_moves(&board);
        match pb {
            PossibleMoves::PlayerWon(Player::Circle) => {println!("Player circle won"); playing = false},
            PossibleMoves::PlayerWon(Player::Cross) => {println!("Player cross won"); playing = false},
            PossibleMoves::Turn(moves, Player::Circle) => {
                println!("Player circle turn. {} possible moves.", moves.len());
                let loc = get_move(moves);
                board[loc.row][loc.col] = BoardSquare::Circle;
            },
            PossibleMoves::Turn(moves, Player::Cross) => {
                println!("Player cross turn. {} possible moves.", moves.len());
                let loc = get_move(moves);
                board[loc.row][loc.col] = BoardSquare::Cross;
            }
        }
    }

}*/

qrc!(my_ressource,
    "todos/qml" {
      //  "../main.qml" as "main.qml",
        "main.qml",
    },
);

fn main() {
    my_ressource();
    qml_register_type::<implementation::QTicTacToe>(CStr::from_bytes_with_nul(b"TicTacToe\0").unwrap(), 1, 0,
        CStr::from_bytes_with_nul(b"TicTacToe\0").unwrap());
    let mut engine = QmlEngine::new();
    engine.load_file("qrc:/todos/qml/main.qml".into());
    engine.exec();
}

