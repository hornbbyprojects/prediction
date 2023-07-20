use super::*;

pub struct Dasher {
    move_next: u64,
    move_until: u64,
    pub dx: f64,
    pub dy: f64,
    pub delete_next_tick: bool,
}

pub const DASHER_MOVE_EVERY: u64 = 20;
pub const DASHER_MOVE_FOR: u64 = 10;
pub const GAME_SCREEN_WIDTH: f64 = 400.0;
pub const GAME_SCREEN_HEIGHT: f64 = 400.0;
const DASHER_RADIUS: f64 = 5.0;

impl Dasher {
    pub fn new(game: &mut Game, x: f64, y: f64, dx: f64, dy: f64) -> GameObjectId {
        let id = game.create_game_object(x, y);
        game.dashers.insert(
            id,
            Dasher {
                dx,
                dy,
                delete_next_tick: false,
                move_next: game.time + DASHER_MOVE_EVERY,
                move_until: game.time + DASHER_MOVE_EVERY + DASHER_MOVE_FOR,
            },
        );
        Danger::new(game, id, DASHER_RADIUS);
        id
    }
    pub fn step(game: &mut Game) {
        let mut to_delete = Vec::new();
        for (id, dasher) in game.dashers.iter_mut() {
            if game.deleted.contains_key(id) || dasher.delete_next_tick {
                to_delete.push(*id);
                continue;
            }
            if dasher.move_next <= game.time {
                let position = game.positions.get_mut(id).expect("Dasher with no position");
                position.x += dasher.dx;
                position.y += dasher.dy;
                if dasher.move_until <= game.time {
                    dasher.move_next = game.time + DASHER_MOVE_EVERY;
                    dasher.move_until = game.time + DASHER_MOVE_EVERY + DASHER_MOVE_FOR;
                }
                if position.y > GAME_SCREEN_HEIGHT {
                    dasher.delete_next_tick = true;
                }
                if position.x > GAME_SCREEN_WIDTH {
                    position.x -= GAME_SCREEN_WIDTH;
                }
                if position.x < 0.0 {
                    position.x += GAME_SCREEN_WIDTH;
                }
            }
        }
        for id in to_delete {
            game.positions.remove(&id);
            game.dashers.remove(&id);
            game.dangers.remove(&id);
        }
    }
}
