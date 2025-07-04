


pub struct Entity {
    pub x: u16,
    pub y: u16,
    pub display: String,
}
pub trait GameEntity{
    fn position(&self) -> (u16, u16);
    fn move_to(&mut self, x: u16, y: u16);

    fn display(&self) -> &str;
}