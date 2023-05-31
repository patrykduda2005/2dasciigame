use std::process::Command;
use console::Term;

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

    fn setchar(&mut self, position: Vec2, char: char) {
        self.board[position.y][position.x] = char;
    }
}

#[derive(Copy, Clone)]
struct Vec2 {
    x: usize,
    y: usize,
}

struct Player {
    skin: char,
    position: Vec2,
}
impl Player {
    fn new() -> Player {
        Player {
            skin: '@',
            position: Vec2 {x: 3, y: 5},
        }
    }
    fn r#move(&mut self, board: &mut Board, position: Vec2) {
        board.setchar(self.position, '.');
        self.position = position;
        board.setchar(self.position, self.skin);
    }
}

fn main() {
    let mut board = Board::new();
    let mut player = Player::new();

    //let mut child = Command::new("sleep").arg("0.1").spawn().unwrap();
    //let _result = child.wait().unwrap();
    let console = console::Term::stdout();
    loop {
        if console.read_key().unwrap() == console::Key::ArrowRight {
            player.r#move(&mut board, Vec2{x:1, y:0})
        }
        clearscreen::clear().expect("Failed to clear a screen");
        board.render();
    }
}
