use macroquad::prelude::{Color, draw_rectangle};

use crate::{JLSTZS, piece::Piece, GRID_HEIGHT, DISTANCE_FROM_WIN, BORDER_THICKNESS, GRID_CONST, GRID_WIDTH};

#[derive(Debug, Clone, Copy)]
pub struct JLSTZ {
    pub block_pos: [(f32, f32); 9],
    color: Color,
    down: i8,
    rdx: i8,
}

impl JLSTZ {
    pub const fn new(block_pos: [(f32, f32); 9], color: Color) -> JLSTZ {
        JLSTZ {
            block_pos,
            color,
            rdx: 3,
            down: -1,
        }
    }

    pub fn move_piece(&mut self) {
        let mut rdx = self.rdx;

        let mut down = self.down;
        for (idx, block) in self.block_pos.iter_mut().enumerate() {
            if idx % 3 == 0 {
                down += 1;
                rdx = self.rdx;
            }
            rdx += 1;    
            
            if block.0 != 0. {
                let x = DISTANCE_FROM_WIN + BORDER_THICKNESS / 2. + rdx as f32*GRID_CONST;
                let y = DISTANCE_FROM_WIN + BORDER_THICKNESS / 2. + down as f32 * GRID_CONST;
                *block = (x, y);
            }
            
        }
    }

    pub fn _draw_rand() {
        let mut piece = JLSTZS[0].clone();
        piece.draw();
    }
}

impl Piece for JLSTZ {
    fn right(&mut self) {
        if self.rdx as f32 >= GRID_WIDTH - 1. {
            return;
        }
        self.rdx += 1;
    }

    fn left(&mut self) {
        if self.rdx < 0 {
            return;
        }
        self.rdx -= 1;
    }

    fn down(&mut self) -> bool {
        /*let mut down = 0;
        for value in self.block_pos {
            if value != 0 {
                down = value;
                break;
            }
        }
        let mut subtract = 2.;
        if self.block_pos[6..].iter().max().unwrap() == &0 {
            subtract = 1.;
        }

        if down as f32 >= GRID_HEIGHT - subtract {
            return true;
        }

        for value in &mut self.block_pos {
            if value.0 != 0. {
                value.0 += GRID_CONST; 
            }
        }
        */
        self.down += 1;
        false
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

    fn draw(&mut self) {
        for block in self.block_pos {
            if block.0 != 0. {
                draw_rectangle(block.0, block.1, GRID_CONST, GRID_CONST, self.color);
            }
            
        }
        /* 
        let mut coord_idx = 0;
        let mut rdx = self.rdx;

        let mut down = -1;
        for (idx, block) in self.block_pos.into_iter().enumerate() {
            if idx % 3 == 0 {
                down += 1;
                rdx = self.rdx;
            }
            rdx += 1;           
            if block != 0 {
                let x = DISTANCE_FROM_WIN + BORDER_THICKNESS / 2. + rdx as f32*GRID_CONST;
                let y = DISTANCE_FROM_WIN + BORDER_THICKNESS / 2. + (down + (block as i8-1)) as f32 * GRID_CONST;
                draw_rectangle(x, y, GRID_CONST, GRID_CONST, self.color);

                coord_idx += 1;
            }
        }
        */
    }

}