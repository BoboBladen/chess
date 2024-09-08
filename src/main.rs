use std::collections::HashMap;

#[derive(Clone)]
pub struct Piece {
    name: char,
    moves: Vec<i32>,
    color: bool,
}
impl Piece {
    fn new(name: char, moves: Vec<i32>, color: bool) -> Piece {
        return Piece { name, moves, color };
    }

    fn get_piece_moves(&self, pos: usize) -> Vec<usize> {
        let mut move_positions: Vec<usize> = Vec::new();
        for mv in &self.moves {
            let new = pos as i32 + mv;
            if new >= 0 && new < 64 {
                move_positions.push(new as usize);
            }
        }
        return move_positions;
    }
}

pub struct Board {
    board: [Option<Piece>; 64],
    turn: bool,
}
impl Board {
    fn new(board: [Option<Piece>; 64], turn: bool) -> Board {
        return Board { board, turn };
    }
    fn get_piece(&self, pos: usize) -> Option<Piece> {
        return self.board[pos].clone();
    }
    fn get_moves(&self) -> HashMap<usize, Vec<usize>> {
        let mut moves_map: HashMap<usize, Vec<usize>> = HashMap::new();
        for i in 0..64 {
            if let Some(piece) = &self.board[i] {
                moves_map.insert(i, piece.get_piece_moves(i));
            }
        }
        return moves_map;
    }

    //     if (self.turn && piece.name.is_uppercase())
    //     || (!self.turn && piece.name.is_lowercase()) && to <= 64
    // {
    //     if let Some(new_pos) = &self.board[to] {
    //         if piece.name.is_uppercase() {
    //             if new_pos.name.is_lowercase() {
    //                 self.board[to] = Some(piece.clone());
    //                 self.board[from] = None;
    //             }
    //         } else if piece.name.is_lowercase() {
    //             if new_pos.name.is_uppercase() {
    //                 self.board[to] = Some(piece.clone());
    //                 self.board[from] = None;
    //             }
    //         }
    //     }
    fn valid_move(&self, from: usize, to: usize) -> usize {
        let from_piece = self.board[from].clone().unwrap();
        if from_piece.color && self.turn {
            if let Some(to_piece) = &self.board[to] {
                if to_piece.color != from_piece.color {
                    //check for checkmate and etc.
                    return 2;
                }
            } else {
                //check for checkmate and etc.
                return 1;
            }
        }

        return 0;
    }

    fn move_piece(&mut self, from: usize, to: usize) {
        if let Some(piece) = &self.board[from] {
            match self.valid_move(from, to) {
                0 => {
                    println!("Invalid move!");
                }
                1 => {
                    //Normal move
                    self.board[to] = Some(piece.clone());
                    self.board[from] = None;
                    self.turn = !self.turn;
                }
                2 => {
                    //Capture
                    self.board[to] = Some(piece.clone());
                    self.board[from] = None;
                    self.turn = !self.turn;
                }
                _ => {}
            }
        }
    }

    fn print_board(&self) {
        for i in 0..64 {
            if i % 8 == 0 {
                println!();
            }
            if let Some(piece) = self.get_piece(i) {
                print!("{} ", piece.name);
            } else {
                print!("- ");
            }
        }
        println!();
        println!("{}", self.turn);
    }
}

fn main() {
    const FEN_STRING: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let mut theboard = create_board(Some(FEN_STRING)).unwrap();
    theboard.print_board();

    theboard.move_piece(10, 32);
    theboard.print_board();

    // let moves = theboard.get_moves();
    // for (i, j) in moves {
    //     for k in j {
    //         println!("{}", k);
    //     }
    // }
    for i in theboard.board[11].clone().unwrap().get_piece_moves(11) {
        println!("{}", i);
    }
}

pub fn create_board(fen_string: Option<&str>) -> Result<Board, String> {
    let mut fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    if fen_string.is_some() {
        fen = fen_string.unwrap();
    }
    let fen_parts: Vec<&str> = fen.split_whitespace().collect();
    if fen_parts.len() != 6 {
        return Err("Invalid FEN-string: There should be 6 parts".to_string());
    }
    const ARRAY_REPEAT_VALUE: Option<Piece> = None;
    let mut board: [Option<Piece>; 64] = [ARRAY_REPEAT_VALUE; 64];
    let board_fen = fen_parts[0];

    let mut width = 0;
    let mut height = 0;
    for c in board_fen.chars() {
        match c {
            '/' => {
                if width != 8 {
                    return Err(
                        "Invalid FEN-string: Incorrect number of squares in rank".to_string()
                    );
                }
                height += 1;
                width = 0;
            }
            '1'..='8' => {
                let count = c.to_digit(10).unwrap();
                let pos = width + height * 8;
                for _ in 0..count {
                    board[pos] = None;
                    width += 1;
                }
            }
            _ => {
                let moves: Vec<i32>;
                let mut color: bool = true;
                match c {
                    'p' => {
                        moves = vec![10, 20, 11, 9];
                        color = false;
                    } // Black pawn moves: 1 square forward, 2 squares forward (initial move), capture left, capture right
                    'P' => moves = vec![-10, -20, -11, -9], // White pawn moves: 1 square forward, 2 squares forward (initial move), capture left, capture right

                    'r' => {
                        moves = vec![10, -10, 1, -1];
                        color = false;
                    } // Black rook moves: vertically or horizontally
                    'R' => moves = vec![10, -10, 1, -1], // White rook moves: vertically or horizontally

                    'n' => {
                        moves = vec![21, 19, 12, 8, -21, -19, -12, -8];
                        color = false;
                    } // Black knight moves: "L" shapes in all directions
                    'N' => moves = vec![21, 19, 12, 8, -21, -19, -12, -8], // White knight moves: "L" shapes in all directions

                    'b' => {
                        moves = vec![11, -11, 9, -9];
                        color = false;
                    } // Black bishop moves: diagonally
                    'B' => moves = vec![11, -11, 9, -9], // White bishop moves: diagonally

                    'q' => {
                        moves = vec![10, -10, 1, -1, 11, -11, 9, -9];
                        color = false;
                    } // Black queen moves: combination of rook and bishop
                    'Q' => moves = vec![10, -10, 1, -1, 11, -11, 9, -9], // White queen moves: combination of rook and bishop

                    'k' => {
                        moves = vec![10, -10, 1, -1, 11, -11, 9, -9];
                        color = false;
                    } // Black king moves: one square in any direction
                    'K' => moves = vec![10, -10, 1, -1, 11, -11, 9, -9], // White king moves: one square in any direction
                    _ => moves = Vec::new(),
                }
                let pos = width + height * 8;
                let piece = Piece::new(c, moves, color);
                board[pos] = Some(piece);
                width += 1;
            }
        }
    }
    let turn_part = fen_parts[1].chars().next().unwrap();
    let mut turn: bool = true;
    match turn_part {
        'w' => turn = true,
        'b' => turn = false,
        _ => {}
    }

    return Ok(Board::new(board, turn));
}
