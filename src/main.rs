mod board;
use board::Board;
mod player;
use player::Player;
pub mod vec2;


fn main() {
    let mut board = Board::new();
    let mut player = Player::new();
    let term = console::Term::stdout();
    loop {
        player.updatemovement(&term, &mut board);
        term.clear_screen().unwrap();
        board.render();
    }
}
