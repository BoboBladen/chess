use glutin_window::GlutinWindow;

use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::{EventLoop, RenderEvent, WindowSettings};

pub use crate::lib::create_board;
pub use crate::lib::Board;
pub use crate::lib::Piece;

mod lib;
mod render;

// Screen size constants
const SCREEN_WIDTH: usize = 600;
const SCREEN_HEIGHT: usize = 600;

fn main() {
    const FEN_STRING: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let theboard = create_board(Some(FEN_STRING)).unwrap();

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

    while let Some(e) = event.next(&mut window) {
        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, g| {
                // clear the screen
                use graphics::clear;
                clear([1.0; 4], g);
                // draw board
                render::draw_board(&c, g, theboard.clone());
            });
        }
    }

    // theboard.print_board();
    // let moves_map = theboard.get_moves();
    // for pos in 0..moves_map.len() {
    //     println!("{}", pos);
    //     if let Some(moves) = moves_map.get(&pos) {
    //         for mov in moves {
    //             println!(" {}", mov);
    //         }
    //     }
    // }

    // let piece = theboard.board[0].clone().unwrap();
    // let moves = piece.get_piece_moves(0);

    // println!("{}", piece.name);
    // let mut movetomake: usize = 0;
    // for i in moves {
    //     println!("{}", i);
    //     movetomake = i;
    // }
    // theboard.move_piece(0, movetomake);
    // theboard.print_board();
}
