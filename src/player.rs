use console::{Term, Key};

use crate::vec2::Vec2;
use crate::Board;

pub struct Player {
    skin: char,
    position: Vec2,
    layer: usize,
    walls: Vec<char>,
}
impl Player {
    pub fn new() -> Player {
        Player {
            skin: '@',
            position: Vec2 {x: 0, y: 0},
            layer: 1,
            walls: vec!['#'],
        }
    }

    pub fn r#move(&mut self, board: &mut Board, position: Vec2) {
        if let Some(char) = board.getchar(0, self.position + position) {
            self.walls.iter().for_each(|wall| if char == *wall {return;});
        }

        if let Err(e) = board.setchar(self.layer, self.position + position, self.skin) {
            println!("{e}");
        } else {
            board.setchar(self.layer, self.position, '.').unwrap();
            self.position += position;
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
