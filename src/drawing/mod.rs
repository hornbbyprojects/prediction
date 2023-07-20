use crate::game::{Game, DASHER_MOVE_FOR};
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::render::{Canvas, RenderTarget};

const DASHER_VISUAL_RADIUS: i16 = 5;
const PLAYER_VISUAL_RADIUS: i16 = 5;
const SIN_ONE_EIGHTH: f64 = 0.70710678118;
const LINE_COLOR: (u8, u8, u8, u8) = (0, 0, 0, 255);
const ARROW_PRONG_LENGTH: f64 = 2.0;
fn draw_arrow<T: RenderTarget>(canvas: &mut Canvas<T>, x1: i16, y1: i16, x2: i16, y2: i16) {
    let dx = x2 - x1;
    let dy = y2 - y1;
    // Draw the main line of the arrow
    canvas
        .aa_line(x1, y1, x2, y2, (0, 0, 0, 255))
        .expect("Could not draw line");

    /* Draw the two prongs of the arrow
    Coefficients here obtained by considering a rotation of 3/8ths of a turn as a matrix.
    */
    let rx1 = -dx as f64 * SIN_ONE_EIGHTH - dy as f64 * SIN_ONE_EIGHTH;
    let ry1 = dx as f64 * SIN_ONE_EIGHTH - dy as f64 * SIN_ONE_EIGHTH;
    let r1_size = (rx1 * rx1 + ry1 * ry1).sqrt();
    let rx1_normalised = rx1 * ARROW_PRONG_LENGTH / r1_size;
    let ry1_normalised = ry1 * ARROW_PRONG_LENGTH / r1_size;
    canvas
        .aa_line(
            x2,
            y2,
            x2 + rx1_normalised as i16,
            y2 + ry1_normalised as i16,
            LINE_COLOR,
        )
        .unwrap();

    let rx2 = -dx as f64 * SIN_ONE_EIGHTH + dy as f64 * SIN_ONE_EIGHTH;
    let ry2 = -dx as f64 * SIN_ONE_EIGHTH - dy as f64 * SIN_ONE_EIGHTH;
    let r2_size = (rx2 * rx2 + ry2 * ry2).sqrt();
    let rx2_normalised = rx2 * ARROW_PRONG_LENGTH / r2_size;
    let ry2_normalised = ry2 * ARROW_PRONG_LENGTH / r2_size;
    canvas
        .aa_line(
            x2,
            y2,
            x2 + rx2_normalised as i16,
            y2 + ry2_normalised as i16,
            LINE_COLOR,
        )
        .unwrap();
}
pub fn draw<T: RenderTarget>(game: &Game, canvas: &mut Canvas<T>, offset_x: i16, offset_y: i16) {
    canvas.set_draw_color((255, 255, 255, 255));
    canvas.clear();
    if let Some(player) = game.player.as_ref() {
        if let Some(player_pos) = game.positions.get(&player.id) {
            canvas
                .circle(
                    player_pos.x as i16 - offset_x,
                    player_pos.y as i16 - offset_y,
                    PLAYER_VISUAL_RADIUS,
                    (0, 0, 255, 255),
                )
                .expect("Failed to draw player");
        }
    }
    for (id, _dasher) in game.dashers.iter() {
        let position = game.positions.get(id).unwrap();
        let x = position.x as i16 + offset_x;
        let y = position.y as i16 + offset_y;
        canvas
            .circle(x, y, DASHER_VISUAL_RADIUS, (255, 0, 0, 255))
            .expect("Failed to draw dasher");
        let ax = x + _dasher.dx as i16 * DASHER_MOVE_FOR as i16;
        let ay = y + _dasher.dy as i16 * DASHER_MOVE_FOR as i16;
        draw_arrow(canvas, x, y, ax, ay);
    }
    canvas.present();
}
