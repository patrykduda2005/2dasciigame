use std::process::Command;
use termion::event::Key;
use termion::event::Key::*;
use termion::input::TermRead;
use core::ops::AddAssign;
use core::ops::Add;

use clearscreen;

struct Board {
    board: [[char; 50]; 20],
}

impl Board {
    fn new() -> Board {
        Board { board: [['.';50];20] }
    }

    fn render(&self) {
        for y in self.board.iter() {
            for x in y.iter() {
                print!("{x}");
            }
            println!("");
        }
    }
    fn setchar(&mut self, position: Vec2, char: char) -> Result<Vec2, &str> {
        if position.x >= self.board[0].len() || position.y >= self.board.len() {
            return Err("Board out of bounds");
        }
        self.board[position.y][position.x] = char;
        return Ok(position);
    }
}

#[derive(Copy, Clone)]
struct Vec2 {
    x: usize,
    y: usize,
}
impl Add for Vec2 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
impl AddAssign for Vec2 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}
struct Player {
    skin: char,
    position: Vec2,
}
impl Player {
    fn new() -> Player {
        Player {
            skin: '@',
            position: Vec2 {x: 0, y: 0},
        }
    }
    fn r#move(&mut self, board: &mut Board, position: Vec2) {
        if let Err(e) = board.setchar(self.position + position, self.skin) {
            println!("{e}");
        } else {
            board.setchar(self.position, '.').unwrap();
            self.position += position;
        }
    }
}

fn main() {
    let mut board = Board::new();
    let mut player = Player::new();
    let stdout = console::Term::buffered_stdout();
    loop {
        stdout.flush().unwrap();
        if let Ok(chara) = stdout.read_char() {
            match chara {
                'd' => player.r#move(&mut board, Vec2{x:1,y:0}),
                _ =>(),
            }
        }


        let mut child = Command::new("sleep").arg("0.1").spawn().unwrap();
        let _result = child.wait().unwrap();
        clearscreen::clear().expect("Failed to clear a screen");
        board.render();
    }
}
