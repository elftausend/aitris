use macroquad::prelude::{Color, draw_rectangle};

use crate::{JLSTZS, piece::Piece, GRID_HEIGHT, DISTANCE_FROM_WIN, BORDER_THICKNESS, GRID_CONST};

#[derive(Debug, Clone, Copy)]
pub struct JLSTZ {
    pub block_pos: [u8; 9],
    color: Color,
    rdx: u8
}

impl JLSTZ {
    pub const fn new(block_pos: [u8; 9], color: Color) -> JLSTZ {
        JLSTZ {
            block_pos,
            color,
            rdx: 3
        }
    }

    pub fn _draw_rand() {
        let piece = JLSTZS[0];
        piece.draw();
    }
}

impl Piece for JLSTZ {
    fn right(&mut self) {
        self.rdx += 1;
    }

    fn left(&mut self) {
        self.rdx -= 1;
    }

    fn down(&mut self) -> bool {
        let down = self.block_pos.iter().max().unwrap();
        if *down as f32 >= GRID_HEIGHT - 1. {
            return false;
        }

        for value in &mut self.block_pos {
            if *value != 0 {
                *value += 1; 
            }
        }
        true
    }

    fn rotate(&mut self) {
        let arr = self.block_pos;
        let col1 = [arr[6], arr[3], arr[0]];
        let col2 = [arr[7], arr[4], arr[1]];
        let col3 = [arr[8], arr[5], arr[2]];
        self.block_pos = [col1[0], col1[1], col1[2],
                          col2[0], col2[1], col2[2],
                          col3[0], col3[1], col3[2]];
    }

    fn draw(&self) {
        let mut rdx = self.rdx;

        let mut down = -1;
        for (idx, block) in self.block_pos.into_iter().enumerate() {
            if idx % 3 == 0 {
                down += 1;
                rdx = self.rdx;
            }
            rdx += 1;           
            if block != 0 {
                draw_rectangle(DISTANCE_FROM_WIN + BORDER_THICKNESS / 2. + rdx as f32*GRID_CONST, DISTANCE_FROM_WIN + BORDER_THICKNESS / 2. + (down + (block as i8-1)) as f32 * GRID_CONST, GRID_CONST, GRID_CONST, self.color);
            }
        }
    }
}