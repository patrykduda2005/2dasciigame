use std::ops::Add;

use bevy::prelude::*;
use console::Term;
mod player;

pub struct BoardPlugin;

#[derive(Component)]
struct Skin(char);

#[derive(Component)]
struct Board(Vec<Vec<Vec<char>>>);

#[derive(Component)]
struct TerminalHandler(Term);

#[derive(Component)]
struct BoardSize{
    layers: usize,
    height: usize,
    width: usize,
}

#[derive(Component, Clone, Copy)]
pub struct Coords{
    x: i32,
    y: i32,
}
impl Add for Coords {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[derive(Component)]
pub struct Layer(usize);

#[derive(Component)]
pub struct New;


#[derive(Event)]
struct MoveEvent{
    layer: usize,
    from: Coords,
    to: Coords,
}

pub enum AddRemove {
    Add,
    Remove,
}

#[derive(Event)]
struct AddRemoveEvent{
    layer: usize,
    coords: Coords,
    skin: char,
    action: AddRemove,
}

fn init_board (
    mut commands: Commands,
) {
    let bs = BoardSize{
        layers: 1,
        height: 20,
        width: 30,
    };
    commands.spawn((
        Board(
            vec![
                vec![
                    vec![
                        '.'; bs.width
                    ]; bs.height
                ]; bs.layers
            ]
        ),
        bs,
        TerminalHandler(
            console::Term::stdout(),
        )
    ));
}

fn prep_term(
    query: Query<&TerminalHandler>
) {
    let TerminalHandler(term) = query.get_single().unwrap();
    term.hide_cursor().unwrap();
}

fn handle_addremove(
    mut addremove_event: EventReader<AddRemoveEvent>,
    mut query: Query<(&mut Board, &BoardSize)>,
) {
    let (mut board, bs) = query.single_mut();
    let Board(map) = board.as_mut();

    for spawn in addremove_event.iter() {
        if spawn.layer > (bs.layers - 1) || spawn.coords.x as usize > (bs.width - 1) || spawn.coords.y as usize > (bs.height - 1) {
            continue;
        }
        
        let skin = match spawn.action {
            AddRemove::Add => spawn.skin,
            AddRemove::Remove => '.',
        };
        map[spawn.layer][spawn.coords.y as usize][spawn.coords.x as usize] = skin;
    }
}

fn handle_move(
    mut move_event: EventReader<MoveEvent>,
    mut query: Query<(&mut Board, &BoardSize)>
) {
    let (mut board, bs) = query.single_mut();
    let Board(map) = board.as_mut();

    for r#move in move_event.iter() {
        if r#move.layer > (bs.layers - 1) || r#move.to.x as usize > (bs.width - 1) || r#move.to.y as usize > (bs.height - 1) {
            continue;
        }
        let skin = map[r#move.layer][r#move.from.y as usize][r#move.from.x as usize];
        map[r#move.layer][r#move.from.y as usize][r#move.from.x as usize] = '.';
        map[r#move.layer][r#move.to.y as usize][r#move.to.x as usize] = skin;
    }
}

fn render (
    query: Query<(&Board, &TerminalHandler, &BoardSize)>,
) {
    let (Board(map), TerminalHandler(term), bs) = query.single();

    term.clear_screen().unwrap();
    for layer in map.iter() {
        term.move_cursor_to(0, bs.height).unwrap();
        for y in layer.iter() {
            for cell in y.iter() {
                print!("{}", cell);
            }
            term.move_cursor_up(1).unwrap();
            term.move_cursor_left(bs.width).unwrap();
        }
        break;
    }
}

fn spawn <T: Component>(
    mut query: Query<(&Coords, &Layer, &Skin), With<T>>,
    mut spawn_event: EventWriter<AddRemoveEvent>,
) {
    for (coords, layer, skin) in query.iter_mut() {
        spawn_event.send(AddRemoveEvent{
            action: AddRemove::Add,
            coords: *coords,
            layer: layer.0,
            skin: skin.0,
        });
    }
}

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<MoveEvent>()
            .add_event::<AddRemoveEvent>()
            .add_systems(PreStartup, init_board)
            .add_systems(Startup, prep_term)
            .add_plugins(player::PlayerPlugin)
            .add_systems(Update, handle_move)
            .add_systems(Update, handle_addremove)
            .add_systems(Update, render.after(handle_move).after(handle_addremove));
    }
}
