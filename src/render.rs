use graphics::{Context, Graphics};
use graphics::{Rectangle, Transformed};
use opengl_graphics::*;

pub use crate::lib::Board;
use crate::SCREEN_HEIGHT;

const COLOR1: [f32; 4] = [0.4, 0.35, 0.35, 1.0];
const COLOR2: [f32; 4] = [0.9, 0.8, 0.75, 1.0];
const CELL_SIZE: usize = SCREEN_HEIGHT / 8;

pub fn draw_board<G: Graphics<Texture = Texture>>(c: &Context, g: &mut G, board: Board) {
    let mut transform = c
        .transform
        .trans((CELL_SIZE / 2) as f64, (CELL_SIZE / 2) as f64);
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
            let name: &str = &piece.name.to_string();

            // import the font
            let font = "fonts/OpenSans-Regular.ttf";
            // Create a piston specific 'Glyphs' from the font
            let mut glyphs =
                GlyphCache::new(font, (), TextureSettings::new()).expect("Could not load font");
            // A transform to move the placement of the text
            graphics::text::Text::new(32)
                .draw(name, &mut glyphs, &c.draw_state, transform, g)
                .unwrap();

            // For some reason this is needed for the text to render correctly
            // draws a space in another color
            graphics::text::Text::new_color([1.0; 4], 32)
                .draw(" ", &mut glyphs, &c.draw_state, c.transform, g)
                .unwrap();
        }
        if i % 8 == 7 {
            transform = transform.trans((-7 * CELL_SIZE as i32) as f64, CELL_SIZE as f64);
        } else {
            transform = transform.trans((CELL_SIZE) as f64, 0.0);
        }
    }
}
