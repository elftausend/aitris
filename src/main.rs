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
    let mut _game_over = false;
    let mut last_update = get_time();

    loop {
        clear_background(WHITE);

        draw_line(DISTANCE_FROM_WIN, DISTANCE_FROM_WIN, screen_width()-DISTANCE_FROM_WIN + BORDER_THICKNESS, DISTANCE_FROM_WIN, BORDER_THICKNESS, LIGHTGRAY);
        draw_line(DISTANCE_FROM_WIN, DISTANCE_FROM_WIN, DISTANCE_FROM_WIN, screen_height() - DISTANCE_FROM_WIN + BORDER_THICKNESS, BORDER_THICKNESS, LIGHTGRAY);
        draw_line(DISTANCE_FROM_WIN, screen_height() - DISTANCE_FROM_WIN + BORDER_THICKNESS, screen_width()-DISTANCE_FROM_WIN, screen_height() - DISTANCE_FROM_WIN + BORDER_THICKNESS, BORDER_THICKNESS, LIGHTGRAY);
        draw_line(screen_width()-DISTANCE_FROM_WIN+BORDER_THICKNESS, DISTANCE_FROM_WIN, screen_width() - DISTANCE_FROM_WIN, screen_height() - DISTANCE_FROM_WIN + BORDER_THICKNESS, BORDER_THICKNESS, LIGHTGRAY);
        
        if piece_move(&mut piece, &mut last_update, &mut pieces) {
            pieces.push(piece);
            let idx = rng.gen_range(0..7);
            if idx == 5 {
                piece = Box::new(LINE);
            } else if idx == 6 {
                piece = Box::new(SQUARE);
            } else {
                piece = Box::new(JLSTZS[idx]);
            }
            piece.update();
            if check_piece_collision(&pieces, piece.block_pos(), 0.) {
                pieces.clear();
            }
        }

        for piece in &mut pieces {
            piece.draw();
        }
        next_frame().await;
    }
}

pub fn piece_move(piece: &mut Box<dyn Piece>, last_update: &mut f64, pieces: &mut [Box<dyn Piece>]) -> bool {
    if is_key_pressed(KeyCode::A) 
    && !check_piece_collision(pieces, piece.block_pos(), -GRID_CONST) {
            piece.left();    
    }
    if is_key_pressed(KeyCode::D)
    && !check_piece_collision(pieces, piece.block_pos(), GRID_CONST) {
        piece.right()
    }
    if is_key_pressed(KeyCode::W) {
        let old_pos = piece.block_pos().to_vec();
        let old_rdx = *piece.rdx_mut();
        piece.rotate();
        if check_piece_collision(pieces, piece.block_pos(), GRID_CONST) 
            || check_piece_collision(pieces, piece.block_pos(), -GRID_CONST) 
        {
            piece.block_pos_mut().copy_from_slice(&old_pos);
            *piece.rdx_mut() = old_rdx;
        }
    }

    if is_key_pressed(KeyCode::Space) {
        for _ in 0..24 {
            if new_piece_collision(pieces, piece) {
                return true;
            }           
            piece.down();
            piece.update(); 
        }
    }

    if get_time() - *last_update > 0.2 {
        *last_update = get_time();
        piece.down();
    }
    piece.update();
    check_clear_row(pieces);
    if new_piece_collision(pieces, piece) {
        return true;
    }
    piece.draw();
    false
}


pub fn check_clear_row(pieces: &mut [Box<dyn Piece>]) {
    for down in 0..GRID_HEIGHT as usize {
        let mut blocks_per_row = 0;
        let y = DISTANCE_FROM_WIN + BORDER_THICKNESS / 2. + down as f32 * GRID_CONST;
        for piece in &mut *pieces {
            for pos in piece.block_pos() {
                if y == pos.1 {
                    blocks_per_row += 1;
                }  
            }
        }
        if blocks_per_row == GRID_WIDTH as i32 {
            for piece in &mut *pieces {
                for pos in piece.block_pos_mut() {
                    if y == pos.1 {
                        *pos = (0., 0.);
                    }
                }
                piece.update();
            }   
        }
    }
}