//! Shared temporary character artwork.
use macroquad::prelude::*;

fn rgb(r: u8, g: u8, b: u8) -> Color {
    Color::from_rgba(r, g, b, 255)
}

/// Position is the character's feet, so movement and floor placement agree.
pub fn draw_character(position: Vec2, moving: bool, time: f32) {
    let x = position.x.round();
    let step = if moving {
        (time * 10.0).sin() * 7.0
    } else {
        0.0
    };
    let y = position.y.round();
    draw_ellipse(
        x,
        y + 2.0,
        40.0,
        11.0,
        0.0,
        Color::new(0.02, 0.04, 0.06, 0.5),
    );
    draw_rectangle(x - 23.0, y - 47.0, 18.0, 42.0 + step, rgb(48, 57, 68));
    draw_rectangle(x + 5.0, y - 47.0, 18.0, 42.0 - step, rgb(63, 68, 75));
    draw_rectangle(x - 28.0, y - 12.0 + step, 25.0, 12.0, rgb(39, 40, 45));
    draw_rectangle(x + 4.0, y - 12.0 - step, 29.0, 12.0, rgb(39, 40, 45));
    draw_rectangle(x - 29.0, y - 107.0, 58.0, 67.0, rgb(65, 134, 141));
    draw_rectangle(x - 25.0, y - 103.0, 12.0, 55.0, rgb(96, 169, 166));
    draw_rectangle(x + 20.0, y - 98.0, 12.0, 52.0, rgb(40, 89, 107));
    draw_rectangle(x - 35.0, y - 97.0, 13.0, 51.0, rgb(72, 134, 141));
    draw_rectangle(x - 35.0, y - 52.0, 13.0, 16.0, rgb(208, 163, 116));
    draw_rectangle(x - 27.0, y - 53.0, 55.0, 9.0, rgb(112, 77, 52));
    draw_rectangle(x - 19.0, y - 144.0, 40.0, 40.0, rgb(219, 174, 126));
    draw_rectangle(x - 23.0, y - 152.0, 45.0, 19.0, rgb(68, 48, 40));
    draw_rectangle(x - 23.0, y - 139.0, 10.0, 22.0, rgb(68, 48, 40));
    draw_rectangle(x + 12.0, y - 130.0, 6.0, 6.0, rgb(36, 44, 49));
    draw_rectangle(x - 9.0, y - 108.0, 29.0, 13.0, rgb(190, 111, 62));
    draw_line(
        x - 19.0,
        y - 104.0,
        x + 21.0,
        y - 55.0,
        9.0,
        rgb(130, 87, 54),
    );
    draw_rectangle(x + 11.0, y - 70.0, 27.0, 29.0, rgb(148, 105, 62));
    draw_rectangle(x + 14.0, y - 67.0, 21.0, 7.0, rgb(190, 145, 82));
}
