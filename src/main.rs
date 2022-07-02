#[cfg(test)]
mod tests;
mod piece;
mod jlstz;

use jlstz::JLSTZ;
use macroquad::prelude::{clear_background, WHITE, next_frame, draw_line, screen_width, LIGHTGRAY, screen_height, Conf, GREEN, BLUE, ORANGE, PINK, RED, is_key_pressed, KeyCode, get_time};
use piece::Piece;

const DISTANCE_FROM_WIN: f32 = 40.;
const GRID_CONST: f32 = 30.;
const BORDER_THICKNESS: f32 = 3.4;
const GRID_HEIGHT: f32 = 20.;
const GRID_WIDTH: f32 = 10.;

#[allow(dead_code)]
const JLSTZS: [JLSTZ; 5] = [
    JLSTZ::new([
        (0., 0.), (0., 0.), (1., 1.),
        (1., 1.), (1., 1.), (1., 1.),
        (0., 0.), (0., 0.), (0., 0.)
    ], BLUE),
    JLSTZ::new([
        (1., 1.), (0., 0.), (0., 0.),
        (1., 1.), (1., 1.), (1., 1.),
        (0., 0.), (0., 0.), (0., 0.)
    ], ORANGE),
    JLSTZ::new([
        (0., 0.), (1., 1.), (1., 1.),
        (1., 1.), (1., 1.), (0., 0.),
        (0., 0.), (0., 0.), (0., 0.)
    ], GREEN),
    JLSTZ::new([
        (0., 0.), (1., 1.), (0., 0.),
        (1., 1.), (1., 1.), (1., 1.),
        (0., 0.), (0., 0.), (0., 0.)
    ], PINK),
    JLSTZ::new([
        (1., 1.), (1., 1.), (0., 0.),
        (0., 0.), (1., 1.), (1., 1.),
        (0., 0.), (0., 0.), (0., 0.)
    ], RED)
];

fn window_conf() -> Conf {
    Conf {
        window_title: "Aitris".to_owned(),
        // window is still resizeable
        window_resizable: false,
        window_width: (GRID_CONST * GRID_WIDTH + DISTANCE_FROM_WIN * 2.) as i32,
        window_height: (GRID_CONST * GRID_HEIGHT + DISTANCE_FROM_WIN * 2.) as i32,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {

    let mut pieces: Vec<Box<dyn Piece>> = vec![];

    let mut piece = JLSTZS[0];
    let mut last_update = get_time();
    

    loop {
        clear_background(WHITE);

        draw_line(DISTANCE_FROM_WIN, DISTANCE_FROM_WIN, screen_width()-DISTANCE_FROM_WIN + BORDER_THICKNESS, DISTANCE_FROM_WIN, BORDER_THICKNESS, LIGHTGRAY);
        draw_line(DISTANCE_FROM_WIN, DISTANCE_FROM_WIN, DISTANCE_FROM_WIN, screen_height() - DISTANCE_FROM_WIN + BORDER_THICKNESS, BORDER_THICKNESS, LIGHTGRAY);
        draw_line(DISTANCE_FROM_WIN, screen_height() - DISTANCE_FROM_WIN + BORDER_THICKNESS, screen_width()-DISTANCE_FROM_WIN, screen_height() - DISTANCE_FROM_WIN + BORDER_THICKNESS, BORDER_THICKNESS, LIGHTGRAY);
        draw_line(screen_width()-DISTANCE_FROM_WIN+BORDER_THICKNESS, DISTANCE_FROM_WIN, screen_width() - DISTANCE_FROM_WIN, screen_height() - DISTANCE_FROM_WIN + BORDER_THICKNESS, BORDER_THICKNESS, LIGHTGRAY);
        
    
        if piece_move(&mut piece, &mut last_update, &pieces) {
            pieces.push(Box::new(piece));
            //piece = JLSTZS[1];
            println!("new piece")
            
        }

        for piece in &mut pieces {
            piece.draw();
        }

        next_frame().await;
    }
}

pub fn piece_move(piece: &mut dyn Piece, last_update: &mut f64, pieces: &[Box<dyn Piece>]) -> bool {

    if is_key_pressed(KeyCode::A) {
        piece.left()
    }
    if is_key_pressed(KeyCode::D) {
        piece.right()
    }
    if is_key_pressed(KeyCode::W) {
        piece.rotate();
    }

    if get_time() - *last_update > 0.5 {
        *last_update = get_time();
        piece.down();
    }
    piece.update();
    if check_collision(pieces, piece) {
        return true;
    }
    piece.draw();
    false
}

pub fn check_collision(pieces: &[Box<dyn Piece>], check_for: &dyn Piece) -> bool {
    for piece in pieces {
        let blocks = piece.blocks();
        let check_for_blocks = check_for.blocks();
        for (x, y) in blocks.into_iter().zip(check_for_blocks) {
            if x == y {
                return true;
            }
        }
    }
    false
}
