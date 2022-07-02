use macroquad::prelude::{Color, draw_rectangle, screen_width};

use crate::{JLSTZS, piece::Piece, DISTANCE_FROM_WIN, BORDER_THICKNESS, GRID_CONST};

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

    pub fn _draw_rand() {
        let mut piece = JLSTZS[0].clone();
        piece.draw();
    }
}

impl Piece for JLSTZ {
    fn right(&mut self) {
        if check_right_wall_collision(&self.block_pos) {
            return;
        }
        self.rdx += 1;
    }

    fn left(&mut self) {
        if check_left_wall_collision(&self.block_pos) {
            return;
        }
        self.rdx -= 1;
    }

    fn down(&mut self) {
        self.down += 1;
    }

    fn update(&mut self) {
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

    fn rotate(&mut self) {
        if check_left_wall_collision(&self.block_pos) || check_right_wall_collision(&self.block_pos) {
            return;
        }
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
    }

    fn blocks(&self) -> [(f32, f32); 4] {
        let mut blocks = [(0., 0.); 4];
        let mut idx = 0;
        for block in self.block_pos {
            if block.0 != 0. {
                blocks[idx] = block;
                idx += 1;
            }
        }
        blocks
    }

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