#[cfg(test)]
mod tests;
mod piece;
mod collision;
mod oi;
mod jlstz;
mod block;

use std::collections::HashMap;

use block::Block;
use collision::new_piece_collision;
use jlstz::JLSTZ;
use macroquad::{prelude::{clear_background, draw_line, get_time, is_key_pressed, next_frame, screen_height, screen_width, Conf, KeyCode, BLUE, GREEN, LIGHTGRAY, ORANGE, PINK, RED, WHITE}, shapes::draw_rectangle};
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
    let mut rng = thread_rng();

    let mut piece: Box<dyn Piece> = Box::new(LINE);
    let mut _game_over = false;
    let mut last_update = get_time();

    let mut blocks = vec![];

    loop {
        clear_background(WHITE);

        draw_line(DISTANCE_FROM_WIN, DISTANCE_FROM_WIN, screen_width()-DISTANCE_FROM_WIN + BORDER_THICKNESS, DISTANCE_FROM_WIN, BORDER_THICKNESS, LIGHTGRAY);
        draw_line(DISTANCE_FROM_WIN, DISTANCE_FROM_WIN, DISTANCE_FROM_WIN, screen_height() - DISTANCE_FROM_WIN + BORDER_THICKNESS, BORDER_THICKNESS, LIGHTGRAY);
        draw_line(DISTANCE_FROM_WIN, screen_height() - DISTANCE_FROM_WIN + BORDER_THICKNESS, screen_width()-DISTANCE_FROM_WIN, screen_height() - DISTANCE_FROM_WIN + BORDER_THICKNESS, BORDER_THICKNESS, LIGHTGRAY);
        draw_line(screen_width()-DISTANCE_FROM_WIN+BORDER_THICKNESS, DISTANCE_FROM_WIN, screen_width() - DISTANCE_FROM_WIN, screen_height() - DISTANCE_FROM_WIN + BORDER_THICKNESS, BORDER_THICKNESS, LIGHTGRAY);
        
        if piece_move(&mut piece, &mut last_update, &mut blocks) {
            for pos in piece.blocks() {
                blocks.push(Block { pos, color: piece.color() });
            }
            let idx = rng.gen_range(0..7);
            // let idx = 5;
            if idx == 5 {
                piece = Box::new(LINE);
            } else if idx == 6 {
                piece = Box::new(SQUARE);
            } else {
                piece = Box::new(JLSTZS[idx]);
            }
            piece.update();
            if check_piece_collision(&blocks, piece.block_pos(), 0.) {
                blocks.clear();
            }
        }

        for block in &blocks {
            block.draw()
        }

        next_frame().await;
    }
}

pub fn piece_move(piece: &mut Box<dyn Piece>, last_update: &mut f64, blocks: &mut Vec<Block>) -> bool {
    if is_key_pressed(KeyCode::A) 
    && !check_piece_collision(blocks, piece.block_pos(), -GRID_CONST) {
            piece.left();    
    }
    if is_key_pressed(KeyCode::D)
    && !check_piece_collision(blocks, piece.block_pos(), GRID_CONST) {
        piece.right()
    }
    if is_key_pressed(KeyCode::W) {
        let old_pos = piece.block_pos().to_vec();
        let old_rdx = *piece.rdx_mut();
        piece.rotate();
        if check_piece_collision(blocks, piece.block_pos(), GRID_CONST) 
            || check_piece_collision(blocks, piece.block_pos(), -GRID_CONST) 
        {
            piece.block_pos_mut().copy_from_slice(&old_pos);
            *piece.rdx_mut() = old_rdx;
        }
    }

    if is_key_pressed(KeyCode::Space) {
        for _ in 0..24 {
            if new_piece_collision(blocks, piece) {
                return true;
            }           
            piece.down();
            piece.update(); 
        }
    }

    if get_time() - *last_update > 0.3 {
        *last_update = get_time();
        piece.down();
    }
    piece.update();
    let move_down = check_clear_row(blocks);
    for move_down in move_down {
        for block in blocks.iter_mut() {
            let down = ((block.pos.1 - (DISTANCE_FROM_WIN + BORDER_THICKNESS / 2.)) / GRID_CONST) as i8;
            if move_down as i8 > down {
                block.pos.1 += GRID_CONST;
            }
        }
    }
    if new_piece_collision(blocks, piece) {
        return true;
    }
    piece.draw();
    false
}

fn range_delete(range_idxs: &[usize], values: &[Block]) -> Vec<Block> {
    let mut new_values = values.iter().map(|x| Some(x)).collect::<Vec<_>>();
    for range_idx in range_idxs {
        new_values[*range_idx] = None;
    }
    new_values.into_iter().flatten().copied().collect()
}

pub fn check_clear_row(blocks: &mut Vec<Block>) -> Vec<usize> {
    let mut move_down = vec![];
    let mut delete_blocks = vec![];
    for down in 0..GRID_HEIGHT as usize {
        let y = DISTANCE_FROM_WIN + BORDER_THICKNESS / 2. + down as f32 * GRID_CONST;

        let mut local_delete_blocks = vec![];
        for (idx, block) in blocks.iter().enumerate() {
            let pos = block.pos;
            if y == pos.1 {
                local_delete_blocks.push(idx);
            }  
        }
        if local_delete_blocks.len() == GRID_WIDTH as usize {
            move_down.push(down);
            delete_blocks.append(&mut local_delete_blocks)
        }
    }

    *blocks = range_delete(&delete_blocks, blocks);

    move_down
}