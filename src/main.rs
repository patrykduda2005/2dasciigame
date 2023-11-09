#![feature(thread_spawn_unchecked)]
use std::time::Duration;
use bevy::app::RunMode::*;
use bevy::app::App;
use bevy::app::ScheduleRunnerPlugin;
use bevy::input::InputPlugin;
mod board;
mod player;

fn main() {
    App::new()
        .add_plugins(ScheduleRunnerPlugin{
            run_mode: Loop {
                wait: Some(
                    Duration::from_millis(17),
                ),
            }  
        })
        .add_plugins(InputPlugin)
        .add_plugins(board::BoardPlugin)
        .add_plugins(player::PlayerPlugin)
        .run();
}
