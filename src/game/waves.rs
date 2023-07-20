use std::ops::Mul;

use derive_more::{Add, AddAssign, Sub, SubAssign};
use rand::Rng;

use super::{Dasher, Game, DASHER_MOVE_EVERY, DASHER_MOVE_FOR};

#[derive(Eq, PartialEq, PartialOrd, Ord, Sub, SubAssign, Add, AddAssign)]
pub struct Threat(i32);

impl const Mul for Threat {
    type Output = Threat;

    fn mul(self, rhs: Self) -> Self::Output {
        Threat(self.0 * rhs.0)
    }
}

impl const Mul<i32> for Threat {
    type Output = Threat;

    fn mul(self, rhs: i32) -> Self::Output {
        Threat(self.0 * rhs)
    }
}

// The higher the minimum threat for a wave, the more of a buffer for "clumping" we will have
const MINIMUM_THREAT_FOR_WAVE: Threat = Threat(400);
const WAVE_SPAWN_WIDTH: u32 = 400;
const MAX_DOWNWARDS_DASHER_WAVE_SIZE: u32 = 20;
const DOWNWARD_DASHER_SPEED: f64 = (1.0 / DASHER_MOVE_FOR as f64) * DASHER_MOVE_EVERY as f64;
const DOWNWARD_DASHER_THREAT: Threat = Threat(60);
const THREAT_PER_TICK: Threat = Threat(2);
const MAX_BISHOP_WAVE_SIZE: u32 = 7;

const INCREASE_THREAT_EVERY: u32 = 600;
const INCREASE_SPEED_EVERY: u32 = 600;
const DIFFICULTY_PER_TICK: u32 = 2;
fn get_speed(base_speed: f64, difficulty: u32) -> f64 {
    let increased_speed = difficulty / INCREASE_SPEED_EVERY;
    base_speed + increased_speed as f64
}
fn get_wave_size(max_size: u32) -> u32 {
    let wave_size = rand::thread_rng().gen_range(0..max_size);
    wave_size
}
pub fn spawn_downwards_dashers_with_parameters(
    game: &mut Game,
    max_wave_size: u32,
    speed: f64,
) -> Threat {
    let wave_size = get_wave_size(max_wave_size);
    for _ in 0u32..wave_size {
        let x = rand::thread_rng().gen_range(0..WAVE_SPAWN_WIDTH);
        Dasher::new(game, x as f64, 0.0, 0.0, speed);
    }
    return DOWNWARD_DASHER_THREAT * wave_size as i32;
}
pub fn spawn_downwards_dashers(game: &mut Game) -> Threat {
    let speed = get_speed(DOWNWARD_DASHER_SPEED, game.wave_spawner.current_difficulty);
    return spawn_downwards_dashers_with_parameters(game, MAX_DOWNWARDS_DASHER_WAVE_SIZE, speed);
}
const SPEEDSTER_SPEED_MULTIPLIER: f64 = 2.0;
const MAX_SPEEDSTERS_WAVE_SIZE: u32 = 3;
pub fn spawn_speedsters(game: &mut Game) -> Threat {
    let speed = get_speed(DOWNWARD_DASHER_SPEED, game.wave_spawner.current_difficulty)
        * SPEEDSTER_SPEED_MULTIPLIER;
    return spawn_downwards_dashers_with_parameters(game, MAX_SPEEDSTERS_WAVE_SIZE, speed);
}
const EXTRA_BISHOP_EVERY: u32 = 600;
const MAX_EXTRA_BISHOPS: u32 = 3;
pub fn spawn_bishops(game: &mut Game) -> Threat {
    let extra_wave_size =
        MAX_EXTRA_BISHOPS.min(game.wave_spawner.current_difficulty / EXTRA_BISHOP_EVERY);
    let wave_size = get_wave_size(MAX_BISHOP_WAVE_SIZE) + extra_wave_size;
    let speed = get_speed(DOWNWARD_DASHER_SPEED, game.wave_spawner.current_difficulty);
    for _ in 0u32..wave_size {
        let x = rand::thread_rng().gen_range(0..WAVE_SPAWN_WIDTH);
        let going_right = rand::thread_rng().gen_bool(0.5);
        let horizontal_speed = if going_right { speed } else { -speed } * 0.75;
        Dasher::new(game, x as f64, 0.0, horizontal_speed, speed);
    }
    return DOWNWARD_DASHER_THREAT * wave_size as i32;
}
const BISHOP_WEIGHT: f64 = 0.25;
const DASHER_WEIGHT: f64 = 1.0;
const SPEEDSTER_WEIGHT: f64 = 0.5;
pub fn spawn_wave(game: &mut Game) -> Threat {
    let possibilities: Vec<(f64, Box<dyn Fn(&mut Game) -> Threat>)> = vec![
        (DASHER_WEIGHT, Box::new(spawn_downwards_dashers)),
        (SPEEDSTER_WEIGHT, Box::new(spawn_speedsters)),
        (BISHOP_WEIGHT, Box::new(spawn_bishops)),
    ];
    let mut total_prob: f64 = possibilities.iter().map(|(p, _)| p).sum();
    for (p, f) in possibilities.iter() {
        let chance = p / total_prob;
        if rand::thread_rng().gen_bool(chance) {
            return f(game);
        }
        total_prob -= p;
    }
    return possibilities.last().expect("No wave possibilities").1(game);
}

const STARTING_DIFFICULTY: u32 = DIFFICULTY_PER_TICK * 1200;
pub struct WaveSpawner {
    threat_needed: Threat,
    pub current_difficulty: u32,
}
impl WaveSpawner {
    pub fn new() -> Self {
        WaveSpawner {
            threat_needed: Threat(0),
            current_difficulty: STARTING_DIFFICULTY,
        }
    }
    pub fn step(game: &mut Game) {
        if game.wave_spawner.threat_needed > MINIMUM_THREAT_FOR_WAVE {
            let wave_threat = spawn_wave(game);
            game.wave_spawner.threat_needed -= wave_threat;
        }
        let extra_threat = game.wave_spawner.current_difficulty / INCREASE_THREAT_EVERY;
        game.wave_spawner.threat_needed += THREAT_PER_TICK + Threat(extra_threat as i32);
        if game.player.is_some() {
            game.wave_spawner.current_difficulty += DIFFICULTY_PER_TICK;
        }
    }
}
