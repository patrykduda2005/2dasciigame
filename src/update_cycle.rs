use crate::belt::Belt;
use crate::board::Board;
pub struct UpdateCycle {
    belts: Vec<Belt>,
}
impl UpdateCycle {
    pub fn new() -> Self {
        UpdateCycle {
            belts: vec![],
        }
    }

    pub fn add_belt(&mut self, belt: Belt, board: &mut Board) {
        belt.spawn(board);
        self.belts.push(belt);
    }

    fn update_belts(&self) {

    }

    pub fn update(&self) {
        self.update_belts();
    }
}
