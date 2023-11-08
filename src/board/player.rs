use std::{thread::{park_timeout, self}, time::Duration};
use bevy::prelude::*;
use console::Key;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

#[derive(Event)]
struct KeyEvent(KeyAction);
enum KeyAction {
    Move(MoveDirection),
}
enum MoveDirection {
    Up,
    Down,
    Left,
    Right,
}

fn init_player(
    mut commands: Commands,
) {
    commands.spawn((
        Player,
        super::Coords {
            x: 5,
            y: 3,
        },
        super::Skin('@'),
        super::Layer(0),
    ));
}

fn handle_keybinds(
    query: Query<&super::TerminalHandler>,
    mut key_event: EventWriter<KeyEvent>,
) {
    let super::TerminalHandler(term) = query.single();
    let input_wait = thread::Builder::new();
    let handler = unsafe {
        input_wait.spawn_unchecked(move || {
            term.read_key().unwrap()
        }).unwrap()
    };
    
    thread::sleep(Duration::from_millis(500));
    if handler.is_finished() == false {
        return;
    }
    match handler.join() {
        Ok(key) => match key {
            Key::Char('d') => key_event.send(KeyEvent(KeyAction::Move(MoveDirection::Right))),
            Key::Char('a') => key_event.send(KeyEvent(KeyAction::Move(MoveDirection::Left))),
            Key::Char('s') => key_event.send(KeyEvent(KeyAction::Move(MoveDirection::Down))),
            Key::Char('w') => key_event.send(KeyEvent(KeyAction::Move(MoveDirection::Up))),
        _ => (),
        },
        Err(_) => ()
    }
}

fn movement(
    mut key_event: EventReader<KeyEvent>,
    mut move_event: EventWriter<super::MoveEvent>,
    mut query: Query<(&mut super::Coords, &super::Layer), With<Player>>,
) {
    let (mut coords_query, super::Layer(layer)) = query.single_mut();
    let coords = coords_query.as_mut();

    for KeyEvent(KeyAction::Move(action)) in key_event.iter() {
        let vector = match action {
            MoveDirection::Right => super::Coords{x: 1, y: 0},
            MoveDirection::Left => super::Coords{x: -1, y: 0},
            MoveDirection::Up => super::Coords{x: 0, y: 1},
            MoveDirection::Down=> super::Coords{x: 0, y: -1},
        };
        move_event.send(super::MoveEvent{
                layer: *layer,
                from: *coords,
                to: *coords + vector,
        });
        *coords = *coords +  vector;
    }
}

fn test (
    mut move_event: EventWriter<super::MoveEvent>,
    mut query: Query<(&mut super::Coords, &super::Layer), With<Player>>,
) {
    let (mut coords_query, super::Layer(layer)) = query.single_mut();
    let coords = coords_query.as_mut();
    move_event.send(super::MoveEvent{
            layer: *layer,
            from: *coords,
            to: *coords + super::Coords{x: 1, y: 0},
    });
    *coords = *coords +  super::Coords{x: 1, y: 0};
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<KeyEvent>()
            .add_systems(PreStartup, init_player)
            .add_systems(Startup, super::spawn::<Player>)
            .add_systems(Update, handle_keybinds)
            .add_systems(Update, test)
            .add_systems(Update, movement.after(handle_keybinds));
    }
}
