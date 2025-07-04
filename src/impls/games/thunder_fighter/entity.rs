use crate::impls::games::entities::{Entity, GameEntity};

//玩家
pub struct Player {
    pub entity: Entity,
    pub health: u16,
}
//敌人
pub struct Enemy {
    pub entity: Entity,
    pub health: u16,
}

impl GameEntity for Player {
    fn position(&self) -> (u16, u16) {
        (self.entity.x, self.entity.y)
    }

    fn move_to(&mut self, x: u16, y: u16) {
        self.entity.x = x;
        self.entity.y = y;
    }

    fn display(&self) -> &str {
        self.entity.display.as_str()
    }
}
impl GameEntity for Enemy {
    fn position(&self) -> (u16, u16) {
        (self.entity.x, self.entity.y)
    }

    fn move_to(&mut self, x: u16, y: u16) {
        self.entity.x = x;
        self.entity.y = y;
    }
    fn display(&self) -> &str {
        self.entity.display.as_str()
    }
}