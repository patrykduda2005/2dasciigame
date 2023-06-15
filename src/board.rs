use console::Term;

use crate::vec2::Vec2;

pub struct Board {
    board: Vec<[[char; 400]; 200]>,
    //board: Vec<Vec<Vec<char>>>,
    fov: (f64,f64),
}

impl Board {
    pub fn new(fov: (f64,f64)) -> Board {
        Board { 
            board: vec![
                //vec![vec!['.'; (size.1 - 1) as usize]; (size.0 - 1) as usize],
                //vec![vec!['.'; (size.1 - 1) as usize]; (size.0 - 1) as usize],
                [['.';400];200], //layer 0 na struktury stale
                [['.';400];200], //layer 1 na entity
                [['.';400];200], //layer 2 na entity
            ],
            fov: fov,
        }
    }
    
    pub fn getboardsize(&self, mut scale: f64) -> (i32,i32) {
        scale = 1.0 / scale;
        (self.board[0].len() as i32 / scale as i32 , self.board[0][0].len() as i32 / scale as i32)
    }

    pub fn render(&self, term: &Term, center: Vec2) {
        for (i, layer) in self.board.iter().enumerate() {
            let mut top_edge = center.y - (self.fov.0 as f64/2.0).floor() as i32;
            let mut bottom_edge = center.y as u16 + (self.fov.0 as f64/2.0).ceil() as u16;
            if top_edge < 0 {top_edge = 0}
            if bottom_edge > layer.len() as u16 {bottom_edge = layer.len() as u16}
            for y in &layer[top_edge as usize..bottom_edge as usize] {
                let mut left_edge = center.x - (self.fov.1 as f64/2.0).floor() as i32;
                let mut right_edge = center.x as u16 + (self.fov.1 as f64/2.0).ceil() as u16;
                if left_edge < 0 {left_edge = 0}
                if right_edge > y.len() as u16 {right_edge = y.len() as u16}
                for x in &y[left_edge as usize..right_edge as usize] {
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
    
    pub fn drawline(&mut self, layer: usize, mut frompos: Vec2, mut topos: Vec2, char: char) {
        //function Adds 1 to longer axis and a ratio to shorter axis
        if frompos.x > topos.x {let tmp = topos.x; topos.x = frompos.x; frompos.x = tmp;}
        if frompos.y > topos.y {let tmp = topos.y; topos.y = frompos.y; frompos.y = tmp;}
        let is_x_longer = (frompos.x-topos.x).abs() > (frompos.y-topos.y).abs();
        let xdiff: f64 = frompos.x as f64 - topos.x as f64;
        let ydiff: f64 = frompos.y as f64 - topos.y as f64;
        let mut y = frompos.y as f64;
        let mut x = frompos.x as f64;
        while x != topos.x as f64 || y != topos.y as f64 {
            self.setchar(layer, Vec2 { x: x.floor() as i32, y: y.floor() as i32 }, char).unwrap();
            if is_x_longer {y += ydiff / xdiff} else {y += 1.0};
            if is_x_longer {x+=1.0} else {x += xdiff / ydiff};
        }
    }
}
