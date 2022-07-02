
pub trait Piece {
    fn right(&mut self);
    fn left(&mut self);
    fn down(&mut self);
    fn update(&mut self);
    fn rotate(&mut self);
    fn draw(&mut self);
    fn blocks(&self) -> [(f32, f32); 4];
}