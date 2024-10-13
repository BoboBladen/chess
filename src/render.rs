use graphics::{Context, Graphics};
use graphics::{Rectangle, Transformed};
use opengl_graphics::*;

pub use crate::lib::Board;
use crate::SCREEN_HEIGHT;

const COLOR1: [f32; 4] = [0.45, 0.4, 0.4, 1.0];
const COLOR2: [f32; 4] = [0.9, 0.8, 0.75, 1.0];
const CELL_SIZE: usize = SCREEN_HEIGHT / 8;

pub fn draw_board<G: Graphics<Texture = Texture>>(
    c: &Context,
    g: &mut G,
    board: Board,
    glyphs: &mut GlyphCache,
) {
    let mut cnt = 0;
    for i in 0..64 {
        let mut color: [f32; 4] = COLOR1;
        if (i + cnt) % 2 == 0 {
            color = COLOR2;
        }
        if let Some(sq) = board.selected {
            if let Some(_sq2) = &board.board[sq] {
                if board.get_valid_moves(sq).contains(&i) {
                    color = [0.7, 0.7, 0.7, 1.0];
                }
            }
            if sq == i {
                color = [0.6, 0.3, 0.3, 1.0];
            }
        }
        if (i % 8) == 7 {
            cnt += 1;
        }
        let x = (i % 8) * CELL_SIZE;
        let y = (i / 8) * CELL_SIZE;

        Rectangle::new(color).draw(
            [x as f64, y as f64, CELL_SIZE as f64, CELL_SIZE as f64],
            &c.draw_state,
            c.transform,
            g,
        );

        if let Some(piece) = &board.board[i] {
            let name;
            match piece.name.to_string().as_str() {
                // White pieces
                "P" => name = '\u{2659}', // White Pawn
                "R" => name = '\u{2656}', // White Rook
                "N" => name = '\u{2658}', // White Knight
                "B" => name = '\u{2657}', // White Bishop
                "Q" => name = '\u{2655}', // White Queen
                "K" => name = '\u{2654}', // White King

                // Black pieces
                "p" => name = '\u{265F}', // Black Pawn
                "r" => name = '\u{265C}', // Black Rook
                "n" => name = '\u{265E}', // Black Knight
                "b" => name = '\u{265D}', // Black Bishop
                "q" => name = '\u{265B}', // Black Queen
                "k" => name = '\u{265A}', // Black King
                _ => name = ' ',
            }
            let piece_transform = c.transform.trans(
                x as f64 + CELL_SIZE as f64 / 2.0 - 22.0, // 22 is about half the width of the character
                y as f64 + CELL_SIZE as f64 / 2.0 + 22.0,
            );
            // A transform to move the placement of the text
            graphics::text::Text::new(50)
                .draw(&name.to_string(), glyphs, &c.draw_state, piece_transform, g)
                .unwrap();

            // For some reason this is needed for the text to render correctly
            // draws a space in another color
            graphics::text::Text::new_color([1.0; 4], 32)
                .draw(" ", glyphs, &c.draw_state, c.transform, g)
                .unwrap();
        }
    }
}
