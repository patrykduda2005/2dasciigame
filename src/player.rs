use console::{Term, Key};

use crate::vec2::Vec2;
use crate::{Board, Destroy};
use crate::Move;

pub struct Player {
    skin: char,
    pub position: Vec2,
    layer: usize,
}
impl Player {
    pub fn new(spawn_location: (i32, i32)) -> Player {
        Player {
            skin: '@',
            position: Vec2 {x: spawn_location.1, y: spawn_location.0},
            layer: 2,
        }
    }

    pub fn updatemovement(&mut self, term: &Term, mut board: &mut Board) {
        match term.read_key().unwrap() {
            Key::Char('d') => self.r#move(&mut board, Vec2 { x: 1, y: 0 }),
            Key::Char('a') => self.r#move(&mut board, Vec2 { x: -1, y: 0 }),
            Key::Char('s') => self.r#move(&mut board, Vec2 { x: 0, y: 1 }),
            Key::Char('w') => self.r#move(&mut board, Vec2 { x: 0, y: -1 }),
            _ => (),
        }
    }
}

impl Destroy for Player {
    fn destroy(self, board: &mut Board) {
        todo!()
    }
}

impl Move for Player {
    fn r#move(&mut self, board: &mut Board, position: Vec2) {
        if let Err(e) = board.setchar(self.layer, self.position + position, self.skin) {
            println!("{e}");
        } else {
            board.setchar(self.layer, self.position, '.').unwrap();
            self.position += position;
        }
    }
}
