#![feature(const_trait_impl)]

use std::{
    thread::sleep,
    time::{Duration, Instant},
};

use game::{Player, GAME_SCREEN_HEIGHT, GAME_SCREEN_WIDTH};
use sdl2::{event::WindowEvent, keyboard::Keycode};

mod drawing;
mod game;

const WINDOW_WIDTH: u32 = 400;
const WINDOW_HEIGHT: u32 = 400;
const TICK_TIME: Duration = Duration::from_millis(1000 / 60);
fn main() {
    let mut game = game::Game::new();
    let sdl2_system = sdl2::init().expect("Couldn't initialise SDL");
    let video_subsystem = sdl2_system.video().expect("No video");
    let mut window_builder =
        video_subsystem.window("PREDICT THEM OR DIE", WINDOW_WIDTH, WINDOW_HEIGHT);
    let window = window_builder
        .opengl()
        .build()
        .expect("Could not create window!");
    let mut canvas = window
        .into_canvas()
        .accelerated()
        .build()
        .expect("Could not create canvas!");
    let mut event_pump = sdl2_system
        .event_pump()
        .expect("Could not obtain event pump!");
    Player::new(
        &mut game,
        GAME_SCREEN_WIDTH as f64 / 2.0,
        GAME_SCREEN_HEIGHT - 20.0,
    );
    'main: loop {
        let tick_start = Instant::now();
        game.step();
        drawing::draw(&game, &mut canvas, 0, 0);
        event_pump.pump_events();
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                sdl2::event::Event::KeyUp {
                    timestamp: _,
                    window_id: _,
                    keycode,
                    scancode: _,
                    keymod: _,
                    repeat: _,
                } => match keycode {
                    Some(Keycode::W) => {
                        if let Some(player) = game.player.as_mut() {
                            player.up = false;
                            Player::move_player(&mut game);
                        }
                    }
                    Some(Keycode::A) => {
                        if let Some(player) = game.player.as_mut() {
                            player.left = false;
                            Player::move_player(&mut game);
                        }
                    }
                    Some(Keycode::S) => {
                        if let Some(player) = game.player.as_mut() {
                            player.down = false;
                            Player::move_player(&mut game);
                        }
                    }
                    Some(Keycode::D) => {
                        if let Some(player) = game.player.as_mut() {
                            player.right = false;
                            Player::move_player(&mut game);
                        }
                    }
                    Some(Keycode::P) => {
                        Player::new(
                            &mut game,
                            GAME_SCREEN_WIDTH as f64 / 2.0,
                            GAME_SCREEN_HEIGHT - 20.0,
                        );
                    }
                    Some(Keycode::R) => {
                        game = game::Game::new();
                        Player::new(
                            &mut game,
                            GAME_SCREEN_WIDTH as f64 / 2.0,
                            GAME_SCREEN_HEIGHT - 20.0,
                        );
                    }

                    _ => {}
                },
                sdl2::event::Event::KeyDown {
                    timestamp: _,
                    window_id: _,
                    keycode,
                    scancode: _,
                    keymod: _,
                    repeat: _,
                } => match keycode {
                    Some(Keycode::W) => {
                        if let Some(player) = game.player.as_mut() {
                            player.up = true;
                            Player::move_player(&mut game);
                        }
                    }
                    Some(Keycode::A) => {
                        if let Some(player) = game.player.as_mut() {
                            player.left = true;
                            Player::move_player(&mut game);
                        }
                    }
                    Some(Keycode::S) => {
                        if let Some(player) = game.player.as_mut() {
                            player.down = true;
                            Player::move_player(&mut game);
                        }
                    }
                    Some(Keycode::D) => {
                        if let Some(player) = game.player.as_mut() {
                            player.right = true;
                            Player::move_player(&mut game);
                        }
                    }
                    _ => {}
                },
                sdl2::event::Event::Window {
                    timestamp: _,
                    window_id: _,
                    win_event,
                } => match win_event {
                    WindowEvent::Close { .. } => {
                        break 'main;
                    }
                    _ => {}
                },
                _ => {}
            }
        }
        if game.player.is_some() {
            canvas
                .window_mut()
                .set_title(&format!(
                    "PREDICT THEM AND LIVE (score {})",
                    game.wave_spawner.current_difficulty
                ))
                .expect("Could not set title");
        } else {
            canvas
                .window_mut()
                .set_title(&format!(
                    "DEATH (score {})",
                    game.wave_spawner.current_difficulty
                ))
                .expect("Could not set title");
        }
        let time_passed = tick_start.elapsed();
        if time_passed < TICK_TIME {
            let remaining = TICK_TIME - time_passed;
            sleep(remaining);
        }
    }
}
