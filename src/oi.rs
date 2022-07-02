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
        let a = self.block_pos[0];
        let b = self.block_pos[4];
        let c = self.block_pos[8];
        let d = self.block_pos[12];

        if a.0 == 0. && b.0 == 0. && c.0 == 0. && d.0 == 0. {
            self.right();
        }

        let a = self.block_pos[3];
        let b = self.block_pos[7];
        let b = self.block_pos[11];
        let c = self.block_pos[15];

        if a.0 == 0. && b.0 == 0. && c.0 == 0. && d.0 == 0. {
            self.left();
        }

        let arr = self.block_pos;
        let col1 = [arr[12], arr[8], arr[4], arr[0]];
        let col2 = [arr[13], arr[9], arr[5], arr[1]];
        let col3 = [arr[12], arr[10], arr[6], arr[2]];
        let col4 = [arr[15], arr[11], arr[7], arr[3]];
        self.block_pos = [col1[0], col1[1], col1[2], col1[3],
                          col2[0], col2[1], col2[2], col2[3],
                          col3[0], col3[1], col3[2], col3[3],
                          col4[0], col4[1], col4[2], col4[3]];
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