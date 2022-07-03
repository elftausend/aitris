use std::fmt::Debug;
use macroquad::prelude::{draw_rectangle, Color};
use crate::{collision::{check_left_wall_collision, check_right_wall_collision}, DISTANCE_FROM_WIN, BORDER_THICKNESS, GRID_CONST};

pub trait Piece: Debug {
    fn right(&mut self) {
        if check_right_wall_collision(self.block_pos_mut(), 0.) {
            return;
        }
        *self.rdx_mut() += 1;
    }
    fn left(&mut self) {
        if check_left_wall_collision(self.block_pos_mut(), 0.) {
            return;
        }
        *self.rdx_mut() -= 1;
    }
    fn down(&mut self) {
        *self.down_mut() += 1;
    }
    fn update(&mut self) {
        let divider = self.divider();
        let init_rdx = *self.rdx_mut();
        let mut rdx = init_rdx;

        let mut down = *self.down_mut();
        for (idx, block) in self.block_pos_mut().iter_mut().enumerate() {
            if idx % divider == 0 {
                down += 1;
                rdx = init_rdx;
            }
            rdx += 1;    
            
            if block.0 != 0. {
                let x = DISTANCE_FROM_WIN + BORDER_THICKNESS / 2. + rdx as f32*GRID_CONST;
                let y = DISTANCE_FROM_WIN + BORDER_THICKNESS / 2. + down as f32 * GRID_CONST;

                *block = (x, y);
            }
            
        }
    }
    fn rotate(&mut self) {}
    fn draw(&self) {
        for block in self.block_pos() {
            if block.0 != 0. {
                draw_rectangle(block.0, block.1, GRID_CONST, GRID_CONST, self.color());
            }
            
        }
    }
    fn blocks(&self) -> [(f32, f32); 4] {
        let mut blocks = [(0., 0.); 4];
        let mut idx = 0;
        for block in self.block_pos() {
            if block.0 != 0. {
                blocks[idx] = *block;
                idx += 1;
            }
        }
        blocks
    }
    fn rdx_mut(&mut self) -> &mut i8;
    fn down_mut(&mut self) -> &mut i8;
    fn block_pos_mut(&mut self) -> &mut [(f32, f32)];
    fn block_pos(&self) -> &[(f32, f32)];
    fn color(&self) -> Color {
        Color { r: 0.01, g: 0.9, b: 0.9, a: 1.0 }
    }
    fn divider(&self) -> usize {
        4
    }
}