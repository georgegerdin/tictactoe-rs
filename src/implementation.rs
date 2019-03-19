use qmetaobject::*;
use std;
use std::collections::HashMap;

#[derive(Copy, Clone, PartialEq)]
enum BoardSquare {
    Empty,
    Circle,
    Cross
}

impl Default for BoardSquare {
    fn default() -> BoardSquare {
        BoardSquare::Empty
    }
}

#[derive(Copy, Clone, PartialEq)]
enum Player {
    Circle,
    Cross
}

#[derive(Clone)]
enum Wins {
    WonHorizontal(Player, usize),
    WonVertical(Player, usize),
    WonDiagonal(Player, bool)
}

enum PossibleMoves {
    PlayerWon(Wins),
    Turn(Vec<BoardPosition>, Player)
}

#[derive(PartialEq)]
struct BoardPosition {
    row: usize,
    col: usize
}

fn is_same(square: BoardSquare, player: Player) -> bool {
    match (square, player) {
        (BoardSquare::Circle, Player::Circle) => true,
        (BoardSquare::Cross, Player::Cross) => true,
        (_, _) => false,
    }
}

fn possible_moves(board: &[[BoardSquare; 3]; 3]) -> PossibleMoves {
    let mut to_move = 0;
    let mut empty_squares = Vec::new();
    
    //Check for horizontal wins
    for row in 0..=2 {
        let mut circle = 0;
        let mut cross = 0;
        for col in 0..=2 {
            match board[row][col] {
                BoardSquare::Empty => empty_squares.push(BoardPosition{row, col}),
                BoardSquare::Circle =>  {
                    circle = circle + 1;
                    to_move = to_move + 1;
                },
                BoardSquare::Cross => {
                    cross = cross + 1;
                    to_move = to_move + 1;
                }
            }
        }

        if circle == 3 {
            return PossibleMoves::PlayerWon(Wins::WonHorizontal(Player::Circle, row));
        }
        if cross == 3 {
            return PossibleMoves::PlayerWon(Wins::WonHorizontal(Player::Cross, row));
        }
    }

    let player_to_move = if to_move % 2 == 0 {
            Player::Circle
        }
        else {
            Player::Cross
        };
    

    //Check for vertical wins
    for col in 0..=2 {
        let mut circle = 0;
        let mut cross = 0;
        for row in 0..=2 {
            match board[row][col] {
                BoardSquare::Empty => (),
                BoardSquare::Circle => circle = circle + 1,
                BoardSquare::Cross => cross = cross + 1
            }
        }

        if circle == 3 {
            return PossibleMoves::PlayerWon(Wins::WonVertical(Player::Circle, col));
        }
        if cross == 3 {
            return PossibleMoves::PlayerWon(Wins::WonVertical(Player::Cross, col));
        }
    }

    //Check for diagonal wins
    if is_same(board[0][0], Player::Circle) && is_same(board[1][1], Player::Circle) && is_same(board[2][2], Player::Circle) {
        return PossibleMoves::PlayerWon(Wins::WonDiagonal(Player::Circle, false));
    }
    if is_same(board[0][2], Player::Circle) && is_same(board[1][1], Player::Circle) && is_same(board[2][0], Player::Circle) {
        return PossibleMoves::PlayerWon(Wins::WonDiagonal(Player::Circle, true));
    }
    if is_same(board[0][0], Player::Cross) && is_same(board[1][1], Player::Cross) && is_same(board[2][2], Player::Cross) {
        return PossibleMoves::PlayerWon(Wins::WonDiagonal(Player::Cross, false));
    }
    if is_same(board[0][2], Player::Cross) && is_same(board[1][1], Player::Cross) && is_same(board[2][0], Player::Cross) {
        return PossibleMoves::PlayerWon(Wins::WonDiagonal(Player::Cross, true));
    }

    PossibleMoves::Turn(empty_squares, player_to_move)
}

#[derive(Default)]
struct TicTacToe {
    board: [[BoardSquare; 3]; 3],
    possible_moves: Option<PossibleMoves>
}

impl TicTacToe {
    fn do_move(&mut self, row: usize, col: usize) -> Option<Player> {
        if self.possible_moves.is_none() {
            self.possible_moves = Some(possible_moves(&self.board));
        };
        
        let pb = self.possible_moves.take().unwrap();
        
        let result = match pb {
            PossibleMoves::PlayerWon(_) => None,
            PossibleMoves::Turn(pb, player) => {
                //Find the move in the possible moves list
                let found = pb.iter().find(|&x| *x == BoardPosition{row, col});
                match found {
                    Some(_) => {
                        if player == Player::Circle {
                            self.board[row][col] = BoardSquare::Circle;
                        }
                        else {
                            self.board[row][col] = BoardSquare::Cross;
                        }
                        Some(player)
                    },
                    None => None
                }
            }
        };

        self.possible_moves = Some(possible_moves(&self.board));

        result
    }

    fn check_if_won(&self) -> Option<Wins> {
        match self.possible_moves {
            Some(PossibleMoves::PlayerWon(ref win)) => Some(win.clone()),
            _ => None
        }
    }
}

#[allow(non_snake_case)]
#[derive(Default, QObject)]
pub struct QTicTacToe {
    base: qt_base_class!(trait QObject),
    game: TicTacToe,
    drawMove: qt_signal!(square: usize, player: usize),
    doMove: qt_method!(fn(&mut self, row: usize, col: usize))
}

impl QTicTacToe {
    #[allow(non_snake_case)]
    fn doMove(&mut self, row: usize, col: usize) {
        let mv = self.game.do_move(row, col);
        let player = match mv {
            Some(player) =>  {
                match player {
                    Player::Circle => 1,
                    Player::Cross => 2
                }
            },
            None => return
        };


        //Draw move on window
        match (row, col) {
            (0, 0) => self.drawMove(0, player),
            (0, 1) => self.drawMove(1, player),
            (0, 2) => self.drawMove(2, player),
            (1, 0) => self.drawMove(3, player),
            (1, 1) => self.drawMove(4, player),
            (1, 2) => self.drawMove(5, player),
            (2, 0) => self.drawMove(6, player),
            (2, 1) => self.drawMove(7, player),
            (2, 2) => self.drawMove(8, player),
            (_, _) => ()
        }

        if let Some(win) =  self.game.check_if_won() {
            match win {
                Wins::WonHorizontal(Player::Circle, row) => {
                    println!("Circle won horizontal row {}", row + 1);
                },
                Wins::WonHorizontal(Player::Cross, row) => {
                    println!("Cross won horizontal row {}", row + 1);
                },
                Wins::WonVertical(Player::Circle, col) => {
                    println!("Circle won vertical column {}", col + 1);
                },
                Wins::WonVertical(Player::Cross, col) => {
                    println!("Cross won vertical column {}", col + 1);
                },
                Wins::WonDiagonal(Player::Circle, _) => {
                    println!("Circle won diagonal");
                },
                Wins::WonDiagonal(Player::Cross, _) => {
                    println!("Cross won diagonal");
                }
            }
        }

    }
}
