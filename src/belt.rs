use crate::{vec2::Vec2, board::Board, Destroy};

#[allow(dead_code)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[allow(dead_code)]
pub struct Belt {
    skin: char,
    position: Vec2,
    layer: usize,
    dir: Direction,
}
impl Belt {
    pub fn new(pos: Vec2, dir: Direction) -> Self {
        let skin = match dir {
            Direction::Up => '^',
            Direction::Down => 'v',
            Direction::Left => '<',
            Direction::Right => '>',
        };
        Belt {
            skin: skin,
            position: pos,
            layer: 0,
            dir: dir,
        }
    }
    
    pub fn spawn(&self, board: &mut Board) {
        board.setchar(self.layer, self.position, self.skin).unwrap();
    }

    pub fn update(&self, board: &mut Board) -> Vec2 {

    }
}

impl Destroy for Belt {
    fn destroy(self, board: &mut Board) {
        board.setchar(self.layer, self.position, '.').unwrap();
    }
}
