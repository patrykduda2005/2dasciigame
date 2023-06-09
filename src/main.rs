use std::{fs::*, thread, time};
mod board;
use board::Board;
mod player;
use player::Player;
use vec2::Vec2;
mod belt;
use belt::{Belt,Direction};
pub mod vec2;
mod update_cycle;
use update_cycle::UpdateCycle;
mod metal;
use metal::Metal;

pub trait Move {
    fn r#move(&mut self, board: &mut Board, position: Vec2) {
        if let Err(e) = board.setchar(self.layer, self.position + position, self.skin) {
            println!("{e}");
        } else {
            board.setchar(self.layer, self.position, '.').unwrap();
            self.position += position;
        }
    }
}

pub trait Destroy {
    fn destroy(self, board: &mut Board);
}

fn getfov(maxsize: (u16,u16)) -> (f64, f64) {
    let mut output = (maxsize.0 as f64, maxsize.1 as f64);
    let mut input = String::new();
    println!("Chcesz maksymalny fov czy customowy(max/custom)?");
    std::io::stdin().read_line(&mut input).expect("Nie udalo sie wziac inputa");
    match input.trim() {
        "max" => return (maxsize.0 as f64 - 1.0, maxsize.1 as f64 - 1.0),
        _ => (),
    }
    output.1 = loop {
        println!("Jak szeroki chcesz miec fov");
        input = String::new();
        std::io::stdin().read_line(&mut input).expect("Nie udalo sie wziac inputa");
        match input.trim().parse::<f64>() {
            Ok(v) if v < (maxsize.1 - 1) as f64 => break v,
            Err(_) => println!("To nie liczba"),
            Ok(_) => println!("Liczba jest wieksza niz twoj monitor"),
        }
    };
    output.0 = loop {
        println!("Jak dlugi chcesz miec fov");
        input = String::new();
        std::io::stdin().read_line(&mut input).expect("Nie udalo sie wziac inputa");
        match input.trim().parse::<f64>() {
            Ok(v) if v < (maxsize.0 - 1) as f64 => break v,
            Err(_) => println!("To nie liczba"),
            Ok(_) => println!("Liczba jest wieksza niz twoj monitor"),
        }
    };
    output
}

fn main() {
    let term = console::Term::stdout();
    let fov = getfov(term.size());
    let mut board = Board::new(fov);
    let mut player = Player::new(board.getboardsize(0.5));
    let mut update_cycle = UpdateCycle::new();
    update_cycle.add_belt(Belt::new(Vec2{x:200,y:100}, Direction::Up), &mut board);
    board.setchar(0, vec2::Vec2{x: 10, y: 5}, '#').unwrap();
    term.hide_cursor().unwrap();
    loop {
        player.updatemovement(&term, &mut board);
        term.clear_screen().unwrap();
        board.render(&term, player.position);
    }
}
