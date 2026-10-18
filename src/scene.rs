//! Temporary, hand-drawn geometry keeps the prototype free of external assets.
use macroquad::prelude::*;

fn rgb(r: u8, g: u8, b: u8) -> Color {
    Color::from_rgba(r, g, b, 255)
}

/// Draw the room in the fixed 1920 × 1080 scene coordinate system.
pub fn draw_room(_time: f32) {
    clear_background(rgb(36, 48, 59));

    // Large, staggered masonry blocks. Deterministic variation avoids a flat wall.
    for row in 0..8 {
        for column in 0..15 {
            let x = column as f32 * 146.0 - if row % 2 == 0 { 0.0 } else { 73.0 };
            let y = row as f32 * 80.0;
            let variation = ((row * 17 + column * 11) % 15) as u8;
            draw_rectangle(
                x + 3.0,
                y + 3.0,
                140.0,
                74.0,
                rgb(91 + variation, 109 + variation, 118 + variation),
            );
            draw_rectangle(x + 6.0, y + 5.0, 134.0, 5.0, rgb(124, 141, 145));
            draw_rectangle(x + 6.0, y + 68.0, 134.0, 7.0, rgb(67, 84, 94));
        }
    }

    // A sealed arch gives the single room a destination and a clear silhouette.
    draw_rectangle(776.0, 255.0, 368.0, 389.0, rgb(28, 39, 48));
    draw_circle(960.0, 265.0, 184.0, rgb(28, 39, 48));
    draw_rectangle(814.0, 286.0, 292.0, 350.0, rgb(49, 65, 72));
    draw_circle(960.0, 286.0, 146.0, rgb(49, 65, 72));
    for index in 0..9 {
        let angle = std::f32::consts::PI + index as f32 * std::f32::consts::PI / 8.0;
        let center = vec2(960.0, 286.0) + vec2(angle.cos(), angle.sin()) * 181.0;
        draw_rectangle(
            center.x - 27.0,
            center.y - 28.0,
            54.0,
            56.0,
            rgb(149, 153, 143),
        );
        draw_rectangle(
            center.x - 23.0,
            center.y - 24.0,
            46.0,
            5.0,
            rgb(179, 178, 158),
        );
    }
    for y in [310.0, 390.0, 470.0, 550.0] {
        for x in [750.0, 1116.0] {
            draw_rectangle(x, y, 57.0, 74.0, rgb(130, 140, 137));
            draw_rectangle(x + 5.0, y + 4.0, 47.0, 6.0, rgb(164, 168, 153));
        }
    }
    for index in 0..6 {
        draw_rectangle(
            830.0 + index as f32 * 46.0,
            300.0,
            7.0,
            330.0,
            rgb(28, 43, 50),
        );
    }
    draw_rectangle(814.0, 455.0, 292.0, 14.0, rgb(29, 44, 52));
    draw_rectangle(920.0, 441.0, 80.0, 43.0, rgb(98, 106, 98));
    draw_circle(960.0, 459.0, 7.0, rgb(29, 43, 48));

    // Framing columns and their layered bases.
    for x in [157.0, 1560.0] {
        draw_rectangle(x - 26.0, 50.0, 202.0, 48.0, rgb(138, 150, 149));
        draw_rectangle(x - 10.0, 98.0, 170.0, 36.0, rgb(91, 111, 120));
        draw_rectangle(x, 134.0, 150.0, 483.0, rgb(118, 137, 141));
        draw_rectangle(x + 18.0, 134.0, 22.0, 483.0, rgb(152, 163, 158));
        draw_rectangle(x + 116.0, 134.0, 34.0, 483.0, rgb(72, 94, 106));
        for y in [245.0, 370.0, 495.0] {
            draw_rectangle(x, y, 150.0, 7.0, rgb(71, 92, 103));
        }
        draw_rectangle(x - 12.0, 617.0, 174.0, 28.0, rgb(141, 153, 147));
        draw_rectangle(x - 31.0, 645.0, 212.0, 35.0, rgb(96, 115, 120));
    }

    draw_rectangle(0.0, 630.0, 1920.0, 30.0, rgb(67, 85, 91));
    draw_rectangle(0.0, 660.0, 1920.0, 420.0, rgb(79, 96, 99));
    // Increasing tile size suggests depth without a perspective camera.
    for (row, (top, bottom)) in [
        (665.0, 732.0),
        (738.0, 831.0),
        (837.0, 963.0),
        (969.0, 1080.0),
    ]
    .iter()
    .enumerate()
    {
        let width = 228.0 + row as f32 * 43.0;
        for column in -1..10 {
            let x = column as f32 * width + if row % 2 == 0 { 0.0 } else { width * 0.5 };
            let shade = ((column + 2 + row as i32 * 3) % 5) as u8 * 3;
            draw_rectangle(
                x + 4.0,
                *top,
                width - 8.0,
                bottom - top,
                rgb(104 + shade, 117 + shade, 114 + shade),
            );
            draw_rectangle(x + 9.0, top + 4.0, width - 18.0, 4.0, rgb(143, 149, 134));
        }
    }

    // Cool architectural shade is deliberately baked in; it is not an occlusion shadow.
    draw_triangle(
        vec2(1470.0, 660.0),
        vec2(1920.0, 660.0),
        vec2(1920.0, 1040.0),
        Color::new(0.06, 0.12, 0.19, 0.43),
    );
    draw_triangle(
        vec2(1470.0, 660.0),
        vec2(1920.0, 1040.0),
        vec2(1690.0, 1040.0),
        Color::new(0.06, 0.12, 0.19, 0.43),
    );

    // Broken stones, moss, and cracks keep the placeholder scene lived-in.
    for (x, y, w) in [
        (98.0, 706.0, 57.0),
        (336.0, 686.0, 39.0),
        (1504.0, 692.0, 68.0),
        (1784.0, 739.0, 89.0),
    ] {
        draw_rectangle(x + 7.0, y + 13.0, w, 17.0, rgb(51, 70, 76));
        draw_rectangle(x, y, w, 22.0, rgb(143, 149, 136));
        draw_rectangle(x + 4.0, y + 3.0, w - 10.0, 5.0, rgb(169, 170, 148));
    }
    for index in 0..30 {
        let x = 345.0 + ((index * 53) % 329) as f32;
        let y = 574.0 + ((index * 17) % 73) as f32;
        draw_rectangle(x, y, 12.0 + (index % 3) as f32 * 7.0, 7.0, rgb(67, 104, 79));
    }
    for (x, y) in [(610.0, 186.0), (1310.0, 337.0), (1190.0, 836.0)] {
        draw_line(x, y, x + 17.0, y + 22.0, 4.0, rgb(49, 70, 79));
        draw_line(x + 17.0, y + 22.0, x + 10.0, y + 41.0, 4.0, rgb(49, 70, 79));
    }
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

pub fn draw_torch(time: f32) {
    draw_rectangle(425.0, 446.0, 30.0, 65.0, rgb(44, 49, 51));
    draw_rectangle(434.0, 422.0, 12.0, 72.0, rgb(110, 71, 42));
    draw_rectangle(418.0, 442.0, 44.0, 13.0, rgb(67, 62, 54));
    let flicker = (time * 11.0).sin() * 4.0;
    draw_triangle(
        vec2(420.0, 435.0),
        vec2(461.0, 435.0),
        vec2(437.0 + flicker, 373.0),
        rgb(255, 140, 39),
    );
    draw_triangle(
        vec2(428.0, 438.0),
        vec2(452.0, 438.0),
        vec2(445.0 - flicker, 397.0),
        rgb(255, 220, 104),
    );
    draw_rectangle(434.0, 421.0, 13.0, 17.0, rgb(255, 246, 178));
}
