#[cfg(test)]
mod tests;

use macroquad::prelude::{clear_background, WHITE, next_frame, draw_line, screen_width, LIGHTGRAY, screen_height, Conf, draw_rectangle, GREEN};

const DISTANCE_FROM_WIN: f32 = 40.;
const GRID_CONST: f32 = 30.;
const BORDER_THICKNESS: f32 = 3.4;

#[allow(dead_code)]
const JLSTZS: [JLSTZ; 5] = [
    JLSTZ::new([
        0, 0, 1,
        1, 1, 1,
        0, 0, 0
    ]),
    JLSTZ::new([
        1, 0, 0,
        1, 1, 1,
        0, 0, 0
    ]),
    JLSTZ::new([
        0, 1, 1,
        1, 1, 0,
        0, 0, 0
    ]),
    JLSTZ::new([
        0, 1, 0,
        1, 1, 1,
        0, 0, 0
    ]),
    JLSTZ::new([
        1, 1, 0,
        0, 1, 1,
        0, 0, 0
    ])
];

#[derive(Debug, Clone, Copy)]
pub struct JLSTZ {
    block_pos: [u8; 9]
}

impl JLSTZ {
    pub const fn new(block_pos: [u8; 9]) -> JLSTZ {
        JLSTZ {
            block_pos
        }
    }
    pub fn rotate(&mut self) {
        let arr = self.block_pos;
        let col1 = [arr[6], arr[3], arr[0]];
        let col2 = [arr[7], arr[4], arr[1]];
        let col3 = [arr[8], arr[5], arr[2]];
        self.block_pos = [col1[0], col1[1], col1[2],
                          col2[0], col2[1], col2[2],
                          col3[0], col3[1], col3[2]];
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Aitris".to_owned(),
        // window is still resizeable
        window_resizable: false,
        window_width: (GRID_CONST * 10.0 + DISTANCE_FROM_WIN * 2.) as i32,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {

    loop {
        clear_background(WHITE);

        draw_line(DISTANCE_FROM_WIN, DISTANCE_FROM_WIN, screen_width()-DISTANCE_FROM_WIN + BORDER_THICKNESS, DISTANCE_FROM_WIN, BORDER_THICKNESS, LIGHTGRAY);
        draw_line(DISTANCE_FROM_WIN, DISTANCE_FROM_WIN, DISTANCE_FROM_WIN, screen_height() - DISTANCE_FROM_WIN, BORDER_THICKNESS, LIGHTGRAY);
        draw_line(DISTANCE_FROM_WIN, screen_height() - DISTANCE_FROM_WIN, screen_width()-DISTANCE_FROM_WIN, screen_height() - DISTANCE_FROM_WIN, BORDER_THICKNESS, LIGHTGRAY);
        draw_line(screen_width()-DISTANCE_FROM_WIN+BORDER_THICKNESS, DISTANCE_FROM_WIN, screen_width() - DISTANCE_FROM_WIN, screen_height() - DISTANCE_FROM_WIN, BORDER_THICKNESS, LIGHTGRAY);

        let piece = JLSTZS[0];

        for block in piece.block_pos {
            
        }

        for idx in 0..10 {
            draw_rectangle(DISTANCE_FROM_WIN + BORDER_THICKNESS / 2. + idx as f32*GRID_CONST, DISTANCE_FROM_WIN + BORDER_THICKNESS / 2., GRID_CONST, GRID_CONST, GREEN);
        }
        
        next_frame().await;
    }
}
