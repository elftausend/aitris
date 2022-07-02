use crate::piece::Piece;

pub struct Line {
    block_pos: [(f32, f32); 16],
    down: i8,
    rdx: i8,
}

impl Line {
    pub const fn new(block_pos: [(f32, f32); 16]) -> Line {
        Line {
            block_pos,
            down: -1,
            rdx: 3,
        }
    }
}

impl Piece for Line {
    fn rotate(&mut self) {
        todo!()
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
    
}