use super::*;

const PLAYER_DANGER_RADIUS: f64 = 5.0;
pub struct Danger {
    radius: f64,
}

impl Danger {
    pub fn new(game: &mut Game, id: GameObjectId, radius: f64) {
        let danger = Danger { radius };
        game.dangers.insert(id, danger);
    }
    pub fn step(game: &mut Game) {
        let mut hit_player = Vec::new();
        let mut damaged = false;
        if let Some(player) = game.player.as_ref() {
            if let Some(position) = game.positions.get(&player.id) {
                for (id, danger) in game.dangers.iter() {
                    let danger_pos = game.positions.get(&id).expect("Danger had no position");
                    if position.is_closer_than(danger_pos, PLAYER_DANGER_RADIUS + danger.radius) {
                        damaged = true;
                        hit_player.push(*id);
                    }
                }
            }
        }
        if damaged {
            for id in hit_player {
                game.deleted.insert(id, ());
            }
            game.player = None;
        }
    }
}
