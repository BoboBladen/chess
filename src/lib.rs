use std::collections::HashMap;

#[derive(Clone)]
pub struct Piece {
    pub name: char,
    moves: Vec<i32>,
    max_steps: usize,
    color: bool,
}
impl Piece {
    fn new(name: char, moves: Vec<i32>, max_steps: usize, color: bool) -> Piece {
        return Piece {
            name,
            moves,
            max_steps,
            color,
        };
    }

    // pub fn get_piece_moves(&self, pos: usize) -> Vec<usize> {
    //     let mut move_positions: Vec<usize> = Vec::new();
    //     let current_row = pos / 8; // Current row (0-7)
    //     let current_col = pos % 8; // Current column (0-7)
    //     for mv in &self.moves {
    //         let new = pos as i32 + mv;
    //         if new >= 0 && new < 64 {
    //             let new_row = (new as usize) / 8;
    //             let new_col = (new as usize) % 8;
    //             let name = self.name.to_ascii_lowercase();
    //             let mut legal: bool = false;
    //             match name {
    //                 'r' => {
    //                     legal = current_row == new_row || current_col == new_col;
    //                 }
    //                 'b' => {
    //                     legal = current_row as i32 - new_row as i32
    //                         == current_col as i32 - new_col as i32;
    //                 }
    //                 'q' => {
    //                     legal = (current_row == new_row || current_col == new_col)
    //                         || (current_row as i32 - new_row as i32
    //                             == current_col as i32 - new_col as i32);
    //                 }
    //                 'n' => legal = true,
    //                 'p' => legal = true,
    //                 'k' => {
    //                     legal = (current_row == new_row || current_col == new_col)
    //                         || (current_row as i32 - new_row as i32
    //                             == current_col as i32 - new_col as i32);
    //                 }
    //                 _ => {}
    //             }
    //             if legal {
    //                 move_positions.push(new as usize);
    //             }
    //         }
    //     }
    //     return move_positions;
    // }
}

pub enum GameState {
    InProgress,
    Check,
    Checkmate,
}

#[derive(Clone)]
pub struct Board {
    pub board: [Option<Piece>; 64],
    pub turn: bool,
    pub selected: Option<usize>,
}
impl Board {
    pub fn new(board: [Option<Piece>; 64], turn: bool) -> Board {
        return Board {
            board,
            turn,
            selected: None,
        };
    }

    pub fn get_game_state() -> GameState {
        let mut game_state = GameState::InProgress;

        return game_state;
    }

    pub fn get_piece(&self, pos: usize) -> Option<Piece> {
        return self.board[pos].clone();
    }
    pub fn get_moves(&self) -> HashMap<usize, Vec<usize>> {
        let mut moves_map: HashMap<usize, Vec<usize>> = HashMap::new();
        for i in 0..64 {
            if let Some(_piece) = &self.board[i] {
                moves_map.insert(i, self.get_valid_moves(i));
            }
        }
        return moves_map;
    }

    pub fn valid_move_in_bounds(&self, pos: usize, to: usize) -> bool {
        let current_row = pos / 8; // Current row (0-7)
        let current_col = pos % 8; // Current column (0-7)
        let mut legal: bool = false;

        let new = to;
        let new_row = (new as usize) / 8;
        let new_col = (new as usize) % 8;
        let name = self.board[pos].clone().unwrap().name.to_ascii_lowercase();

        match name {
            'r' => {
                legal = current_row == new_row || current_col == new_col;
            }
            'b' => {
                legal = current_row as i32 - new_row as i32 == current_col as i32 - new_col as i32;
            }
            'q' => {
                legal = (current_row == new_row || current_col == new_col)
                    || (current_row as i32 - new_row as i32 == current_col as i32 - new_col as i32);
            }
            'n' => {
                let row_diff = (current_row as i32 - new_row as i32).abs();
                let col_diff = (current_col as i32 - new_col as i32).abs();
                legal = (row_diff == 2 && col_diff == 1) || (row_diff == 1 && col_diff == 2);
            }
            'p' => legal = true,
            'k' => {
                legal = (current_row == new_row || current_col == new_col)
                    || (current_row as i32 - new_row as i32 == current_col as i32 - new_col as i32);
            }
            _ => {}
        }
        return legal;
    }

    pub fn get_valid_moves(&self, pos: usize) -> Vec<usize> {
        let mut moves: Vec<usize> = vec![];
        let piece = self.board[pos].clone().unwrap();
        for mv in piece.moves {
            for i in 1..=piece.max_steps {
                let to: i32 = pos as i32 + mv * i as i32;
                if to > 0
                    && to < 64
                    && self.valid_move(pos, to as usize) != 0
                    && self.valid_move_in_bounds(pos, to as usize)
                {
                    moves.push(to as usize);
                } else {
                    break;
                }
            }
        }
        moves
    }

    fn valid_move(&self, from: usize, to: usize) -> usize {
        let from_piece = self.board[from].clone().unwrap();
        if from_piece.color == self.turn {
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

    pub fn move_piece(&mut self, from: usize, to: usize) {
        if let Some(piece) = &self.board[from] {
            match self.valid_move(from, to) {
                0 => {
                    println!("Invalid move!, {} {}", self.turn, piece.color);
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

    pub fn print_board(&self) {
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
                let mut max_steps = 1;
                match c {
                    'p' => {
                        moves = vec![8, 16]; // Black pawn: 1 square forward (8), 2 squares forward (16 for the initial move)
                                             // attacks = vec![7, 9]; // Black pawn: diagonal captures (left 7, right 9)
                        color = false; // Black
                    }
                    'P' => {
                        moves = vec![-8, -16]; // White pawn: 1 square forward (-8), 2 squares forward (-16 for the initial move)
                                               // attacks = vec![-9, -7]; // White pawn: diagonal captures (left -9, right -7)
                        color = true; // White
                    }

                    'r' => {
                        moves = vec![8, -8, 1, -1]; // Black rook: vertically (±8) or horizontally (±1)
                        max_steps = 8; // Rook can move up to 8 squares in any direction
                        color = false; // Black
                    }
                    'R' => {
                        moves = vec![8, -8, 1, -1]; // White rook: vertically (±8) or horizontally (±1)
                        max_steps = 8; // Rook can move up to 8 squares in any direction
                        color = true; // White
                    }

                    'n' => {
                        moves = vec![17, 15, 10, 6, -17, -15, -10, -6]; // Black knight: "L" shapes
                        max_steps = 1; // Knight jumps, so max_steps is 1
                        color = false; // Black
                    }
                    'N' => {
                        moves = vec![17, 15, 10, 6, -17, -15, -10, -6]; // White knight: "L" shapes
                        max_steps = 1; // Knight jumps, so max_steps is 1
                        color = true; // White
                    }

                    'b' => {
                        moves = vec![9, 7, -9, -7]; // Black bishop: diagonally (±9, ±7)
                        max_steps = 8; // Bishop can move up to 8 squares diagonally
                        color = false; // Black
                    }
                    'B' => {
                        moves = vec![9, 7, -9, -7]; // White bishop: diagonally (±9, ±7)
                        max_steps = 8; // Bishop can move up to 8 squares diagonally
                        color = true; // White
                    }

                    'q' => {
                        moves = vec![8, -8, 1, -1, 9, 7, -9, -7]; // Black queen: combination of rook and bishop
                        max_steps = 8; // Queen can move up to 8 squares in any direction
                        color = false; // Black
                    }
                    'Q' => {
                        moves = vec![8, -8, 1, -1, 9, 7, -9, -7]; // White queen: combination of rook and bishop
                        max_steps = 8; // Queen can move up to 8 squares in any direction
                        color = true; // White
                    }

                    'k' => {
                        moves = vec![8, -8, 1, -1, 9, 7, -9, -7]; // Black king: one square in any direction
                        max_steps = 1; // King can only move 1 square
                        color = false; // Black
                    }
                    'K' => {
                        moves = vec![8, -8, 1, -1, 9, 7, -9, -7]; // White king: one square in any direction
                        max_steps = 1; // King can only move 1 square
                        color = true; // White
                    }
                    _ => moves = Vec::new(),
                }
                let pos = width + height * 8;
                let piece = Piece::new(c, moves, max_steps, color);
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
