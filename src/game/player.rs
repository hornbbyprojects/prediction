use super::*;

const PLAYER_SPEED: f64 = 5.0;

pub struct Player {
    pub id: GameObjectId,
    pub left: bool,
    pub right: bool,
    pub up: bool,
    pub down: bool,
    pub dx: f64,
    pub dy: f64,
    pub invincibility_until: u64,
    pub lives: u8,
}

fn sticky_speed(negative: bool, positive: bool, previous: f64) -> f64 {
    if negative {
        if positive {
            return previous;
        } else {
            return -PLAYER_SPEED;
        }
    } else if positive {
        return PLAYER_SPEED;
    }
    return 0.0;
}
const SPAWN_INVINCIBILITY_TIME: u64 = 300;
impl Player {
    pub fn new(game: &mut Game, x: f64, y: f64) {
        let id = game.create_game_object(x, y);
        let player = Player {
            id,
            dx: 0.0,
            dy: 0.0,
            up: false,
            down: false,
            left: false,
            right: false,
            invincibility_until: game.time + SPAWN_INVINCIBILITY_TIME,
            lives: 3,
        };
        game.player = Some(player);
    }
    pub fn move_player(game: &mut Game) {
        if let Some(player) = game.player.as_mut() {
            player.dx = sticky_speed(player.left, player.right, player.dx);
            player.dy = sticky_speed(player.up, player.down, player.dy);
        }
    }
    pub fn step(game: &mut Game) {
        if let Some(player) = game.player.as_ref() {
            if let Some(position) = game.positions.get_mut(&player.id) {
                position.x += player.dx;
                position.y += player.dy;
                if position.x > GAME_SCREEN_WIDTH {
                    position.x -= GAME_SCREEN_WIDTH;
                }
                if position.x < 0.0 {
                    position.x += GAME_SCREEN_WIDTH;
                }
                if position.y > GAME_SCREEN_HEIGHT {
                    position.y = GAME_SCREEN_HEIGHT;
                }
                if position.y < 0.0 {
                    position.y = 0.0;
                }
            }
        }
    }
}
