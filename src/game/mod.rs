use std::{
    collections::HashMap,
    hash::{BuildHasher, Hasher},
    io::Cursor,
};

use byteorder::{BigEndian, ReadBytesExt};

use self::waves::WaveSpawner;

mod waves;

mod dasher;
pub use dasher::*;

mod player;
pub use player::*;

mod danger;
pub use danger::*;

#[derive(Hash, Eq, Ord, PartialEq, PartialOrd, Debug, Copy, Clone)]
pub struct GameObjectId(u64);

pub struct Position {
    pub x: f64,
    pub y: f64,
}

impl Position {
    pub fn get_distance_squared(&self, other: &Self) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        dx * dx + dy * dy
    }
    pub fn is_closer_than(&self, other: &Self, distance: f64) -> bool {
        let distance_sq = self.get_distance_squared(other);
        distance_sq < distance * distance
    }
}

pub struct GameObjectIdHasher {
    value: u64,
    already_written: bool,
}
impl Hasher for GameObjectIdHasher {
    fn finish(&self) -> u64 {
        self.value
    }

    fn write(&mut self, bytes: &[u8]) {
        if self.already_written {
            panic!("Wrote to hasher twice!");
        }
        self.value = Cursor::new(bytes)
            .read_u64::<BigEndian>()
            .expect("Failed to read u64 when hashing");
        self.already_written = true;
    }
}
#[derive(Default)]
pub struct GameObjectIdBuildHasher {}
impl BuildHasher for GameObjectIdBuildHasher {
    type Hasher = GameObjectIdHasher;

    fn build_hasher(&self) -> Self::Hasher {
        GameObjectIdHasher {
            value: 0,
            already_written: false,
        }
    }
}
type IdHashMap<V> = HashMap<GameObjectId, V, GameObjectIdBuildHasher>;
pub struct Game {
    id_counter: u64,
    time: u64,
    pub deleted: IdHashMap<()>,
    pub player: Option<Player>,
    pub dangers: IdHashMap<Danger>,
    pub wave_spawner: WaveSpawner,
    pub positions: IdHashMap<Position>,
    pub dashers: IdHashMap<Dasher>,
}

impl Game {
    pub fn create_game_object(&mut self, x: f64, y: f64) -> GameObjectId {
        let id = GameObjectId(self.id_counter);
        self.id_counter += 1;
        self.positions.insert(id, Position { x, y });
        id
    }
    pub fn step(&mut self) {
        Player::step(self);
        Danger::step(self);
        WaveSpawner::step(self);
        Dasher::step(self);
        self.time += 1;
    }
    pub fn new() -> Self {
        let game = Game {
            id_counter: 0,
            time: 0,
            player: None,
            deleted: IdHashMap::with_hasher(Default::default()),
            dangers: IdHashMap::with_hasher(Default::default()),
            wave_spawner: WaveSpawner::new(),
            positions: IdHashMap::with_hasher(Default::default()),
            dashers: IdHashMap::with_hasher(Default::default()),
        };
        game
    }
}
