use std::collections::HashMap;

#[derive(Clone)]
pub struct Piece {
    pub name: char,
    moves: Vec<i32>,
    max_steps: usize,
    color: bool,
    has_moved: bool,
}
impl Piece {
    fn new(name: char, moves: Vec<i32>, max_steps: usize, color: bool) -> Piece {
        return Piece {
            name,
            moves,
            max_steps,
            color,
            has_moved: false,
        };
    }
}
#[derive(Clone)]
pub enum GameState {
    InProgress,
    Check,
    Checkmate,
    GameOver,
}

#[derive(Clone)]
pub struct Board {
    pub board: [Option<Piece>; 64],
    pub turn: bool,
    pub selected: Option<usize>,
    game_state: GameState,
}
impl Board {
    pub fn new(board: [Option<Piece>; 64], turn: bool) -> Board {
        return Board {
            board,
            turn,
            selected: None,
            game_state: GameState::InProgress,
        };
    }

    pub fn get_game_state(self) -> GameState {
        return self.game_state;
    }

    pub fn get_piece(&self, pos: usize) -> Option<Piece> {
        return self.board[pos].clone();
    }

    //keeps track of all moves
    pub fn get_moves(&mut self) -> HashMap<usize, Vec<usize>> {
        let mut moves_map: HashMap<usize, Vec<usize>> = HashMap::new();
        for i in 0..64 {
            if let Some(_piece) = &self.get_piece(i) {
                moves_map.insert(i, self.get_valid_moves(i));
            }
        }
        return moves_map;
    }

    //Check for bounds
    pub fn valid_move_in_bounds(&self, pos: usize, to: usize) -> bool {
        let current_row = pos / 8; // Current row (0-7)
        let current_col = pos % 8; // Current column (0-7)
        let mut legal: bool = false;

        let new = to;
        let new_row = new / 8;
        let new_col = new % 8;
        let name = self.get_piece(pos).unwrap().name.to_ascii_lowercase();

        match name {
            'r' => {
                legal = current_row == new_row || current_col == new_col;
            }
            'b' => {
                legal = (current_row as i32 - new_row as i32).abs()
                    == (current_col as i32 - new_col as i32).abs();
            }
            'q' => {
                legal = (current_row == new_row || current_col == new_col)
                    || (current_row as i32 - new_row as i32).abs()
                        == (current_col as i32 - new_col as i32).abs();
            }
            'n' => {
                let row_diff = (current_row as i32 - new_row as i32).abs();
                let col_diff = (current_col as i32 - new_col as i32).abs();
                legal = (row_diff == 2 && col_diff == 1) || (row_diff == 1 && col_diff == 2);
            }
            'p' => {
                legal = (current_row as i32 - new_row as i32).abs()
                    == (current_col as i32 - new_col as i32).abs()
                    || current_col == new_col
            }
            'k' => {
                legal = (current_row == new_row || current_col == new_col)
                    || (current_row as i32 - new_row as i32).abs()
                        == (current_col as i32 - new_col as i32).abs();
            }
            _ => {}
        }
        return legal;
    }

    //Returns all valid moves for a certain piece
    pub fn get_valid_moves(&mut self, pos: usize) -> Vec<usize> {
        let mut moves: Vec<usize> = vec![];
        let mut piece = self.get_piece(pos).clone().unwrap();
        if piece.name.to_ascii_lowercase() == 'p' {
            if !piece.has_moved {
                piece.max_steps = 2;
            } else {
                piece.max_steps = 1;
            }
        }
        for mv in piece.moves {
            for i in 1..=piece.max_steps {
                if piece.name.to_ascii_lowercase() == 'p' {
                    // pawn logic with attacks
                    let attack_to: i32 = 1 + pos as i32 + mv as i32;
                    if let Some(target) = self.get_piece(attack_to as usize) {
                        if attack_to > 0
                            && attack_to < 64
                            && self.valid_move_in_bounds(pos, attack_to as usize)
                            && target.color != piece.color
                        {
                            moves.push(attack_to as usize);
                        }
                    }
                    let attack_to: i32 = -1 + pos as i32 + mv as i32;
                    if let Some(target) = self.get_piece(attack_to as usize) {
                        if attack_to > 0
                            && attack_to < 64
                            && self.valid_move_in_bounds(pos, attack_to as usize)
                            && target.color != piece.color
                        {
                            moves.push(attack_to as usize);
                        }
                    }
                    let to: i32 = pos as i32 + mv * i as i32;
                    if to > 0
                        && to < 64
                        && self.valid_move(pos, to as usize) == 1
                        && self.valid_move_in_bounds(pos, to as usize)
                    {
                        moves.push(to as usize);
                    } else {
                        break;
                    }
                } else {
                    // allow for captures on normal moves
                    let to: i32 = pos as i32 + mv * i as i32;
                    if to > 0
                        && to < 64
                        && self.valid_move(pos, to as usize) != 0
                        && self.valid_move_in_bounds(pos, to as usize)
                    {
                        moves.push(to as usize);
                        if self.valid_move(pos, to as usize) == 2 {
                            if self.board[to as usize]
                                .clone()
                                .unwrap()
                                .name
                                .to_ascii_lowercase()
                                == 'k'
                                && self.board[to as usize].clone().unwrap().color
                                    != self.board[pos].clone().unwrap().color
                            {
                                self.game_state = GameState::Check;
                            }
                            break;
                        }
                    } else {
                        break;
                    }
                }
            }
        }
        moves
    }

    //Check if a move is valid, return captures etc.
    fn valid_move(&self, from: usize, to: usize) -> usize {
        let from_piece = self.get_piece(from).unwrap();
        if from_piece.color == self.turn {
            if let Some(to_piece) = &self.get_piece(to) {
                if to_piece.color != from_piece.color {
                    //Capture
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
        let piece_clone;
        if let Some(piece) = &mut self.board[from] {
            piece.has_moved = true;
            piece_clone = piece.clone();
        } else {
            return;
        }

        match self.valid_move(from, to) {
            0 => {
                println!("Invalid move!, {} {}", self.turn, piece_clone.color);
            }
            1 => {
                //Normal move
                self.board[to] = Some(piece_clone.clone());
                self.board[from] = None;
                self.turn = !self.turn;
            }
            2 => {
                //Capture
                self.board[to] = Some(piece_clone.clone());
                self.board[from] = None;
                self.turn = !self.turn;
                if self.board[to].clone().unwrap().name.to_ascii_lowercase() == 'k' {
                    self.game_state = GameState::GameOver;
                }
            }
            _ => {}
        }
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
                        moves = vec![8]; // Black pawn: 1 square forward (8), 2 squares forward (16 for the initial move)
                        color = false; // Black
                    }
                    'P' => {
                        moves = vec![-8]; // White pawn: 1 square forward (-8), 2 squares forward (-16 for the initial move)
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
