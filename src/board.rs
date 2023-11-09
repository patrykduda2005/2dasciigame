use std::ops::Add;

use bevy::prelude::*;
use console::Term;

pub struct BoardPlugin;

#[derive(Component)]
pub struct Skin(pub char);

#[derive(Component)]
pub struct Layer(pub usize);

#[derive(Component)]
pub struct Board(Vec<Vec<Vec<char>>>);

#[derive(Component)]
pub struct TerminalHandler(pub Term);

#[derive(Component)]
pub struct BoardSize{
    pub layers: usize,
    pub height: usize,
    pub width: usize,
}

#[derive(Component, Clone, Copy, PartialEq, PartialOrd)]
pub struct Coords{
    pub x: i32,
    pub y: i32,
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

#[derive(Event)]
pub struct MoveEvent{
    pub layer: usize,
    pub from: Coords,
    pub to: Coords,
}

#[derive(Event)]
pub struct AddRemoveEvent{
    pub layer: usize,
    pub coords: Coords,
    pub skin: char,
    pub action: AddRemove,
}
pub enum AddRemove {
    Add,
    Remove,
}

fn init_board (
    mut commands: Commands,
) {
    let bs = BoardSize{
        layers: 3,
        height: 200,
        width: 400,
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
    board_query: Query<(&Board, &TerminalHandler, &BoardSize)>,
    player_query: Query<&Coords, With<super::player::Player>>,
) {
    let (Board(map), TerminalHandler(term), bs) = board_query.single();

    let term_size = term.size();
    let center_coords = player_query.single();
    let padding: usize = 3;
    
    //to jest ohydne ale dziala
    let mut top_edge = center_coords.y + (term_size.0 as i32 / 2) - 2*padding as i32;
    let mut bottom_edge = center_coords.y - (term_size.0 as i32 / 2);
    let mut left_edge = center_coords.x - (term_size.1 as i32 / 2);
    let mut right_edge = center_coords.x + (term_size.1 as i32 / 2) - 2*padding as i32;
    if top_edge > -1 + bs.height as i32 {top_edge = -1 + bs.height as i32}
    if bottom_edge < 0 {bottom_edge = 0}
    if right_edge > -1 + bs.width as i32 {right_edge = -1 + bs.width as i32}
    if left_edge < 0 {left_edge = 0}



    term.clear_screen().unwrap();
    println!("Kordynaty: {}, {}", center_coords.x, center_coords.y);
    for (layer_number, layer) in map.iter().enumerate() {
        term.move_cursor_to(0, term_size.0 as usize - padding - 1).unwrap();
        for y in layer[bottom_edge as usize..=top_edge as usize].iter() {
            term.move_cursor_right(padding).unwrap();
            for cell in y[left_edge as usize..=right_edge as usize].iter() {
                if layer_number > 0 && *cell == '.' {
                    term.move_cursor_right(1).unwrap();
                } else {
                    print!("{}", cell);
                }
            }
            term.move_cursor_up(1).unwrap();
            term.move_cursor_left(term_size.1 as usize).unwrap();
        }
    }
}

pub fn spawn <T: Component>(
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

fn spawn_things(
    mut spawn_event: EventWriter<AddRemoveEvent>,
) {
    spawn_event.send(AddRemoveEvent{
        action: AddRemove::Add,
        coords: Coords { x: 20, y: 20 },
        layer: 0,
        skin: '#',
    });
    spawn_event.send(AddRemoveEvent{
        action: AddRemove::Add,
        coords: Coords { x: 200, y: 150 },
        layer: 0,
        skin: '#',
    });
}

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<MoveEvent>()
            .add_event::<AddRemoveEvent>()
            .add_systems(PreStartup, init_board)
            .add_systems(Startup, prep_term)
            .add_systems(Startup, spawn_things.after(prep_term))
            .add_systems(Update, handle_move)
            .add_systems(Update, handle_addremove)
            .add_systems(Update, render.after(handle_move).after(handle_addremove));
    }
}
