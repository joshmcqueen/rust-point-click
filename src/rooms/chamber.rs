use crate::model::{GameState, ObjectId};
use crate::world::Point;
use macroquad::prelude::*;

pub fn hotspot(p: Point) -> Option<ObjectId> {
    match (p.x, p.y) {
        (650.0..=850.0, 680.0..=855.0) => Some(ObjectId::Chest),
        (780.0..=1140.0, 130.0..=640.0) => Some(ObjectId::Gate),
        (405.0..=475.0, 365.0..=515.0) => Some(ObjectId::Torch),
        (1250.0..=1450.0, 365.0..=490.0) => Some(ObjectId::Inscription),
        _ => None,
    }
}

pub fn approach(object: ObjectId) -> Point {
    match object {
        ObjectId::Chest => Point::new(750.0, 900.0),
        ObjectId::Torch => Point::new(480.0, 710.0),
        ObjectId::Inscription => Point::new(1350.0, 710.0),
        _ => Point::new(960.0, 700.0),
    }
}

pub fn draw_background(state: &GameState, time: f32) {
    draw_room(time);
    if state.gate_open {
        draw_rectangle(
            814.0,
            290.0,
            292.0,
            346.0,
            Color::from_rgba(12, 20, 28, 255),
        );
        draw_rectangle(
            820.0,
            300.0,
            25.0,
            330.0,
            Color::from_rgba(90, 105, 105, 255),
        );
    }
    draw_rectangle(
        1250.0,
        365.0,
        200.0,
        125.0,
        Color::from_rgba(112, 125, 120, 255),
    );
    for row in 0..4 {
        draw_rectangle(
            1270.0,
            385.0 + row as f32 * 24.0,
            150.0 - row as f32 * 12.0,
            5.0,
            Color::from_rgba(48, 64, 68, 255),
        );
    }
    // The opening cuts through the left wall, with daylight on its threshold.
    draw_rectangle(0.0, 370.0, 135.0, 430.0, Color::from_rgba(49, 83, 69, 255));
    draw_rectangle(
        0.0,
        398.0,
        104.0,
        320.0,
        Color::from_rgba(145, 178, 133, 255),
    );
    for row in 0..6 {
        draw_rectangle(
            132.0,
            366.0 + row as f32 * 70.0,
            43.0,
            64.0,
            Color::from_rgba(145, 150, 132, 255),
        );
    }
    draw_rectangle(
        0.0,
        350.0,
        180.0,
        30.0,
        Color::from_rgba(165, 165, 143, 255),
    );
    draw_rectangle(
        0.0,
        800.0,
        180.0,
        18.0,
        Color::from_rgba(175, 173, 136, 255),
    );
    draw_rectangle(0.0, 823.0, 150.0, 9.0, Color::from_rgba(148, 155, 121, 255));
}

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

/// The chest's floor footprint ends at y = 855, which also sets its draw order.
pub fn draw_chest() {
    // A flat lid and a shaded side give this simple obstacle a readable volume.
    draw_ellipse(
        752.0,
        852.0,
        113.0,
        16.0,
        0.0,
        Color::new(0.02, 0.03, 0.04, 0.45),
    );
    draw_rectangle(650.0, 680.0, 200.0, 175.0, rgb(48, 33, 29));
    draw_rectangle(656.0, 686.0, 188.0, 61.0, rgb(145, 99, 57));
    draw_rectangle(656.0, 747.0, 164.0, 102.0, rgb(111, 68, 40));
    draw_rectangle(820.0, 747.0, 24.0, 102.0, rgb(75, 48, 34));

    // Individual boards stay aligned to the same chunky pixel grid as the room.
    for y in [704.0, 724.0] {
        draw_rectangle(656.0, y, 188.0, 4.0, rgb(106, 68, 41));
        draw_rectangle(660.0, y + 4.0, 180.0, 3.0, rgb(165, 116, 67));
    }
    for y in [779.0, 813.0] {
        draw_rectangle(656.0, y, 164.0, 5.0, rgb(76, 45, 32));
        draw_rectangle(660.0, y + 5.0, 154.0, 3.0, rgb(132, 81, 45));
    }
    draw_rectangle(673.0, 765.0, 31.0, 4.0, rgb(145, 91, 49));
    draw_rectangle(763.0, 801.0, 39.0, 4.0, rgb(78, 46, 32));
    draw_rectangle(716.0, 832.0, 35.0, 4.0, rgb(145, 91, 49));

    // Brass straps, lid rim, and a dark keyhole help it read as an archive chest.
    for x in [676.0, 794.0] {
        draw_rectangle(x, 686.0, 17.0, 163.0, rgb(130, 102, 49));
        draw_rectangle(x, 686.0, 5.0, 163.0, rgb(193, 159, 83));
        for y in [694.0, 735.0, 764.0, 831.0] {
            draw_rectangle(x + 7.0, y, 4.0, 4.0, rgb(69, 58, 35));
        }
    }
    draw_rectangle(650.0, 742.0, 200.0, 9.0, rgb(49, 34, 26));
    draw_rectangle(650.0, 739.0, 200.0, 5.0, rgb(184, 139, 70));
    draw_rectangle(732.0, 733.0, 28.0, 42.0, rgb(78, 61, 33));
    draw_rectangle(736.0, 733.0, 20.0, 36.0, rgb(202, 165, 83));
    draw_rectangle(743.0, 746.0, 7.0, 8.0, rgb(45, 38, 31));
    draw_rectangle(745.0, 753.0, 3.0, 8.0, rgb(45, 38, 31));
    draw_rectangle(656.0, 686.0, 188.0, 5.0, rgb(185, 136, 78));
    draw_rectangle(650.0, 849.0, 200.0, 6.0, rgb(43, 32, 28));
}
