use bevy::prelude::*;
use console::Key;
use super::board::Coords;
use super::board;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

#[derive(Event)]
struct KeyEvent(KeyAction);
enum KeyAction {
    Move(MoveDirection),
}
enum MoveDirection {
    Vertical(i32),
    Horizontal(i32),
}

fn init_player(
    mut commands: Commands,
) {
    commands.spawn((
        Player,
        Coords {
            x: 5,
            y: 3,
        },
        board::Skin('@'),
        board::Layer(1),
    ));
}

fn handle_keybinds(
    query: Query<&board::TerminalHandler>,
    mut key_event: EventWriter<KeyEvent>,
) {
    let board::TerminalHandler(term) = query.single();
    //let input_wait = thread::Builder::new();
    match term.read_key().unwrap() {
        Key::Char('d') => key_event.send(KeyEvent(KeyAction::Move(MoveDirection::Horizontal(1)))),
        Key::Char('a') => key_event.send(KeyEvent(KeyAction::Move(MoveDirection::Horizontal(-1)))),
        Key::Char('s') => key_event.send(KeyEvent(KeyAction::Move(MoveDirection::Vertical(-1)))),
        Key::Char('w') => key_event.send(KeyEvent(KeyAction::Move(MoveDirection::Vertical(1)))),
        Key::Char('D') => key_event.send(KeyEvent(KeyAction::Move(MoveDirection::Horizontal(5)))),
        Key::Char('A') => key_event.send(KeyEvent(KeyAction::Move(MoveDirection::Horizontal(-5)))),
        Key::Char('S') => key_event.send(KeyEvent(KeyAction::Move(MoveDirection::Vertical(-5)))),
        Key::Char('W') => key_event.send(KeyEvent(KeyAction::Move(MoveDirection::Vertical(5)))),
        _ => (),
    }
}

fn movement(
    mut key_event: EventReader<KeyEvent>,
    mut move_event: EventWriter<board::MoveEvent>,
    mut query: Query<(&mut Coords, &board::Layer), With<Player>>,
    board_query: Query<&board::BoardSize>,
) {
    let (mut coords_query, board::Layer(layer)) = query.single_mut();
    let coords = coords_query.as_mut();
    let bs = board_query.single();

    for KeyEvent(KeyAction::Move(action)) in key_event.iter() {
        let vector = match action {
            MoveDirection::Horizontal(n) => Coords{x: *n, y: 0},
            MoveDirection::Vertical(n) => Coords{x: 0, y: *n},
        };
        let bs_size = Coords{x: (bs.width as i32) - 1, y: (bs.height as i32) - 1};
        let new_coords = *coords + vector;
        if new_coords.x > bs_size.x || new_coords.y > bs_size.y || new_coords.x < 0 || new_coords.y < 0 {
            continue;
        }
        move_event.send(board::MoveEvent{
                layer: *layer,
                from: *coords,
                to: new_coords,
        });
        *coords = new_coords;
    }
}

fn test (
    mut move_event: EventWriter<board::MoveEvent>,
    mut query: Query<(&mut Coords, &board::Layer), With<Player>>,
) {
    let (mut coords_query, board::Layer(layer)) = query.single_mut();
    let coords = coords_query.as_mut();
    move_event.send(board::MoveEvent{
            layer: *layer,
            from: *coords,
            to: *coords + Coords{x: 1, y: 0},
    });
    *coords = *coords +  Coords{x: 1, y: 0};
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<KeyEvent>()
            .add_systems(PreStartup, init_player)
            .add_systems(Startup, board::spawn::<Player>)
            .add_systems(Update, handle_keybinds)
            //.add_systems(Update, test)
            .add_systems(Update, movement.after(handle_keybinds));
    }
}
