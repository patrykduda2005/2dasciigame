use crate::vec2::Vec2;

pub struct Board {
    board: [[char; 50]; 20],
}

impl Board {
    pub fn new() -> Board {
        Board { 
            board: [['.';50];20]
        }
    }

    pub fn render(&self) {
        for y in self.board.iter() {
            for x in y.iter() {
                print!("{x}");
            }
            println!("");
        }
    }

    pub fn setchar(&mut self, position: Vec2, char: char) -> Result<Vec2, &str> {
        if position.x as usize >= self.board[0].len() || position.y as usize >= self.board.len() {
            return Err("Board out of bounds");
        }
        self.board[position.y as usize][position.x as usize] = char;
        return Ok(position);
    }
}
