#[cfg(test)]
mod tests;
mod piece;
mod collision;
mod oi;
mod jlstz;

use collision::new_piece_collision;
use jlstz::JLSTZ;
use macroquad::prelude::{clear_background, WHITE, next_frame, draw_line, screen_width, LIGHTGRAY, screen_height, Conf, GREEN, BLUE, ORANGE, PINK, RED, is_key_pressed, KeyCode, get_time};
use oi::{Line, Square};
use piece::Piece;
use rand::{thread_rng, Rng};

use crate::collision::check_piece_collision;

const DISTANCE_FROM_WIN: f32 = 40.;
const GRID_CONST: f32 = 25.;
const BORDER_THICKNESS: f32 = 3.4;
const GRID_HEIGHT: f32 = 24.;
const GRID_WIDTH: f32 = 10.;

const SQUARE: Square = Square::new([
    (0., 0.), (1., 1.), (1., 1.), (0., 0.),
    (0., 0.), (1., 1.), (1., 1.), (0., 0.),
    (0., 0.), (0., 0.), (0., 0.), (0., 0.),
]);

const LINE: Line = Line::new([
    (0., 0.), (0., 0.), (0., 0.), (0., 0.),
    (1., 1.), (1., 1.), (1., 1.), (1., 1.),
    (0., 0.), (0., 0.), (0., 0.), (0., 0.),
    (0., 0.), (0., 0.), (0., 0.), (0., 0.),
]);

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
    let mut rng = thread_rng();

    let mut piece: Box<dyn Piece> = Box::new(LINE);

    let mut last_update = get_time();
    

    loop {
        clear_background(WHITE);

        draw_line(DISTANCE_FROM_WIN, DISTANCE_FROM_WIN, screen_width()-DISTANCE_FROM_WIN + BORDER_THICKNESS, DISTANCE_FROM_WIN, BORDER_THICKNESS, LIGHTGRAY);
        draw_line(DISTANCE_FROM_WIN, DISTANCE_FROM_WIN, DISTANCE_FROM_WIN, screen_height() - DISTANCE_FROM_WIN + BORDER_THICKNESS, BORDER_THICKNESS, LIGHTGRAY);
        draw_line(DISTANCE_FROM_WIN, screen_height() - DISTANCE_FROM_WIN + BORDER_THICKNESS, screen_width()-DISTANCE_FROM_WIN, screen_height() - DISTANCE_FROM_WIN + BORDER_THICKNESS, BORDER_THICKNESS, LIGHTGRAY);
        draw_line(screen_width()-DISTANCE_FROM_WIN+BORDER_THICKNESS, DISTANCE_FROM_WIN, screen_width() - DISTANCE_FROM_WIN, screen_height() - DISTANCE_FROM_WIN + BORDER_THICKNESS, BORDER_THICKNESS, LIGHTGRAY);
        
    
        if piece_move(&mut piece, &mut last_update, &pieces) {
            pieces.push(piece);
            let idx = rng.gen_range(0..7);
            if idx == 5 {
                piece = Box::new(LINE);
            } else if idx == 6 {
                piece = Box::new(SQUARE);
            } else {
                piece = Box::new(JLSTZS[idx]);
            }            
            
        }

        for piece in &mut pieces {
            piece.draw();
        }

        next_frame().await;
    }
}

pub fn piece_move(piece: &mut Box<dyn Piece>, last_update: &mut f64, pieces: &[Box<dyn Piece>]) -> bool {

    if is_key_pressed(KeyCode::A) 
    && !check_piece_collision(pieces, piece.block_pos(), -GRID_CONST) {
            piece.left();    
    }
    if is_key_pressed(KeyCode::D)
    && !check_piece_collision(pieces, piece.block_pos(), GRID_CONST) {
        piece.right()
    }
    if is_key_pressed(KeyCode::W) 
    // not optimal
    && !check_piece_collision(pieces, piece.block_pos(), -GRID_CONST)
    && !check_piece_collision(pieces, piece.block_pos(), GRID_CONST) {
        piece.rotate();
    }

    if is_key_pressed(KeyCode::Space) {

    }

    if get_time() - *last_update > 0.2 {
        *last_update = get_time();
        piece.down();
    }
    piece.update();
    if new_piece_collision(pieces, piece) {
        return true;
    }
    /*if check_left_wall_collision(piece.block_pos(), piece.divider()) 
    || check_right_wall_collision(piece.block_pos(), piece.divider()) {
        return false;
    }*/
    piece.draw();
    false
}

