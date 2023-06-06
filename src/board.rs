use console::Term;

use crate::vec2::Vec2;

pub struct Board {
    board: Vec<[[char; 50]; 20]>,
}

impl Board {
    pub fn new() -> Board {
        Board { 
            board: vec![
                [['.';50];20], //layer 0 na struktury stale
                [['.';50];20], //layer 1 na entity
            ]
        }
    }

    pub fn render(&self, term: &Term) {
        for (i, layer) in self.board.iter().enumerate() {
            for y in layer.iter() {
                for x in y.iter() {
                    if i > 0 && *x == '.' {
                        term.move_cursor_right(1).unwrap();
                    } else {
                        print!("{x}");
                    }
                }
                println!("");
            }
            term.move_cursor_to(0, 0).unwrap();
        }
    }

    fn isoutofbounds(&self, layer: usize, position: Vec2) -> bool {
        if position.x as usize >= self.board[layer][0].len() || position.y as usize >= self.board[layer].len() {
            return true;
        } else {
            return false;
        }
    }

    pub fn getchar(&self, layer: usize, position: Vec2) -> Option<char> {
        if self.isoutofbounds(layer, position) {
            return None;
        } else {
            return Some(self.board[layer][position.y as usize][position.x as usize]);
        }
    }

    pub fn setchar(&mut self, layer: usize, position: Vec2, char: char) -> Result<Vec2, &str> {
        if self.isoutofbounds(layer, position) {
            return Err("Board out of bounds");
        }
        self.board[layer][position.y as usize][position.x as usize] = char;
        return Ok(position);
    }
}
