mod board;
use board::Board;
mod player;
use player::Player;
pub mod vec2;


fn main() {
    let mut board = Board::new();
    board.setchar(0, vec2::Vec2{x: 10, y: 5}, '#').unwrap();
    let mut player = Player::new();
    let term = console::Term::stdout();
    term.hide_cursor().unwrap();
    loop {
        player.updatemovement(&term, &mut board);
        term.clear_screen().unwrap();
        board.render(&term);
    }
}
