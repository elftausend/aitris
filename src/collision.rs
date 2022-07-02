use macroquad::prelude::{screen_height, screen_width};

use crate::{piece::Piece, DISTANCE_FROM_WIN, GRID_CONST};

pub fn new_piece_collision(pieces: &[Box<dyn Piece>], check_for: &dyn Piece) -> bool {
    for block in check_for.blocks() {
        let y = screen_height() - DISTANCE_FROM_WIN;
        if block.1 + GRID_CONST >= y {
            return true;
        }
    }

    for piece in pieces {
        let blocks = piece.blocks();
        
        for x in check_for.blocks() {
            let x = (x.0, x.1 + GRID_CONST);
            if blocks.contains(&x) {
                return true;
            }
        }
    }
    false
}

pub fn check_right_wall_collision(block_pos: &[(f32, f32)]) -> bool {
    let right_wall = screen_width()-DISTANCE_FROM_WIN - GRID_CONST;
    let a = block_pos[2];
    let b = block_pos[5];
    let c = block_pos[8];
    if a.0 == 0. && b.0 == 0. && c.0 == 0. {
        if block_pos[4].0 >= right_wall {
            return true;
        }
    }
    if a.0 >= right_wall || b.0 >= right_wall || c.0 >= right_wall {
        return true;
    }
    false
}

pub fn check_left_wall_collision(block_pos: &[(f32, f32)]) -> bool {
    let left_wall = DISTANCE_FROM_WIN + GRID_CONST;
    let a = block_pos[0];
    let b = block_pos[3];
    let c = block_pos[6];

    if a.0 == 0. && b.0 == 0. && c.0 == 0. {
        if block_pos[4].0 <= left_wall {
            return true;
        }
    }
    if a.0 <= left_wall && a.0 != 0. || b.0 <= left_wall && b.0 != 0. || c.0 <= left_wall && c.0 != 0. {
        return true;
    }
    false
}