use crate::Vec2;
use crate::Move;

pub struct Metal {
    skin: char,
    position: Vec2,
}
impl Metal {
    pub fn new(pos: Vec2) -> Self {
        Metal {
            skin: '%',
            position: pos,
        }
    }
}

impl Move for Metal {
    fn r#move(&mut self, board: &mut crate::board::Board, position: Vec2) {

    }
}
