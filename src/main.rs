use glutin_window::GlutinWindow;
use std::cmp;

use opengl_graphics::{GlGraphics, GlyphCache, OpenGL, TextureSettings};
use piston::event_loop::{EventSettings, Events};
use piston::input::{Button, MouseButton};
use piston::{EventLoop, RenderEvent, WindowSettings};
use piston::{MouseCursorEvent, PressEvent};

pub use crate::lib::create_board;
pub use crate::lib::Board;
pub use crate::lib::Piece;

mod lib;
mod render;

// Screen size constants
const SCREEN_WIDTH: usize = 600;
const SCREEN_HEIGHT: usize = 600;
const CELL_SIZE: usize = SCREEN_HEIGHT / 8;

fn main() {
    const FEN_STRING: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let mut theboard = create_board(Some(FEN_STRING)).unwrap();

    let opengl = OpenGL::V3_2;
    // The piston window
    let mut window: GlutinWindow =
        WindowSettings::new("Chess", [SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32])
            .graphics_api(opengl)
            .exit_on_esc(true)
            .build()
            .unwrap();
    // the event and render structs from piston
    let mut event = Events::new(EventSettings::new().lazy(false));
    let mut gl = GlGraphics::new(opengl);
    let mut mouse_position: [f64; 2] = [0.0, 0.0];
    // import the font
    let font = "fonts/DejaVuSansCondensed-Bold.ttf";
    // Create a piston specific 'Glyphs' from the font
    let mut glyphs =
        GlyphCache::new(font, (), TextureSettings::new()).expect("Could not load font");

    while let Some(e) = event.next(&mut window) {
        if let Some(position) = e.mouse_cursor_args() {
            mouse_position = position;
        }

        if let Some(Button::Mouse(MouseButton::Left)) = e.press_args() {
            let x: usize = cmp::min(
                (mouse_position[0] / CELL_SIZE as f64) as usize,
                SCREEN_WIDTH - 1,
            );
            let y: usize = cmp::min(
                (mouse_position[1] / CELL_SIZE as f64) as usize,
                SCREEN_HEIGHT - 1,
            );
            let pos = x + 8 * y;
            if let Some(sq) = theboard.selected {
                if let Some(_sq2) = &theboard.board[sq] {
                    if theboard.get_valid_moves(sq).contains(&pos) {
                        theboard.move_piece(sq, pos);
                        theboard.selected = None;
                    } else {
                        theboard.selected = Some(pos);
                    }
                } else {
                    theboard.selected = Some(pos);
                }
            } else {
                theboard.selected = Some(pos);
            }
        }

        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, g| {
                // clear the screen
                use graphics::clear;
                clear([1.0; 4], g);
                // draw board
                render::draw_board(&c, g, theboard.clone(), &mut glyphs);
            });
        }
    }
}
