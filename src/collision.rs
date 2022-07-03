use macroquad::prelude::{screen_height, screen_width};
use crate::{piece::Piece, DISTANCE_FROM_WIN, GRID_CONST};

pub fn new_piece_collision(pieces: &[Box<dyn Piece>], check_for: &Box<dyn Piece>) -> bool {
    for block in check_for.blocks() {
        let y = screen_height() - DISTANCE_FROM_WIN;
        if block.1 + GRID_CONST >= y {
            return true;
        }
    }
    check_piece_collision(pieces, check_for.block_pos(), 0.)
}

pub fn check_piece_collision(pieces: &[Box<dyn Piece>], block_pos: &[(f32, f32)], x_offset: f32) -> bool {
    for piece in pieces {
        let blocks = piece.blocks();
        
        for x in block_pos {
            let x = (x.0 + x_offset, x.1 + GRID_CONST);
            if blocks.contains(&x) {
                return true;
            }
        }
    }
    false
}

pub fn check_right_wall_collision(block_pos: &[(f32, f32)], x_offset: f32) -> bool {
    for block in block_pos {
        let x = screen_width() - DISTANCE_FROM_WIN;
        if block.0 + x_offset + GRID_CONST >= x && block.0 != 0. {
            return true;
        }
    }
    false
}

pub fn check_left_wall_collision(block_pos: &[(f32, f32)], x_offset: f32) -> bool {
    for block in block_pos {
        let x = DISTANCE_FROM_WIN;
        if block.0 + x_offset - GRID_CONST <= x && block.0 != 0. {
            return true;
        }
    }
    false
}