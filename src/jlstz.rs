use macroquad::prelude::Color;

use crate::{JLSTZS, piece::Piece};

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
        let piece = JLSTZS[0];
        piece.draw();
    }
}

impl Piece for JLSTZ {
    fn rotate(&mut self) {
        let a = self.block_pos[0];
        let b = self.block_pos[3];
        let c = self.block_pos[6];

        if a.0 == 0. && b.0 == 0. && c.0 == 0. {
            self.right();
        }

        let a = self.block_pos[2];
        let b = self.block_pos[5];
        let c = self.block_pos[8];

        if a.0 == 0. && b.0 == 0. && c.0 == 0. {
            self.left();
        }

        let arr = self.block_pos;
        let col1 = [arr[6], arr[3], arr[0]];
        let col2 = [arr[7], arr[4], arr[1]];
        let col3 = [arr[8], arr[5], arr[2]];
        self.block_pos = [col1[0], col1[1], col1[2],
                          col2[0], col2[1], col2[2],
                          col3[0], col3[1], col3[2]];
    }

    fn rdx_mut(&mut self) -> &mut i8 {
        &mut self.rdx
    }

    fn down_mut(&mut self) -> &mut i8 {
        &mut self.down
    }

    fn block_pos_mut(&mut self) -> &mut [(f32, f32)] {
        &mut self.block_pos
    }

    fn block_pos(&self) -> &[(f32, f32)] {
        &self.block_pos
    }
    
    fn color(&self) -> Color {
        self.color
    }

}