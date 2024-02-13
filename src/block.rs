use macroquad::{color::Color, shapes::draw_rectangle};

use crate::GRID_CONST;

#[derive(Debug, Clone, Copy)]
pub struct Block {
    pub pos: (f32, f32),
    pub color: Color
}

impl Block {
    pub fn draw(&self) {
        if self.pos.0 != 0. && self.pos.1 != 0. {
            draw_rectangle(self.pos.0, self.pos.1, GRID_CONST, GRID_CONST, self.color);
        }

    }
}