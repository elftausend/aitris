
pub trait Piece {
    fn right(&mut self);
    fn left(&mut self);
    fn down(&mut self) -> bool;
    fn rotate(&mut self);
    fn draw(&mut self);
}