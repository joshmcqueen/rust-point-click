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

pub fn draw_background(state: &GameState) {
    if state.gate_open {
        // Cover the painted gate, keeping its stone arch visible.
        let dark = Color::from_rgba(12, 20, 28, 255);
        draw_circle(978.0, 360.0, 164.0, dark);
        draw_rectangle(814.0, 360.0, 328.0, 262.0, dark);
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
}

fn rgb(r: u8, g: u8, b: u8) -> Color {
    Color::from_rgba(r, g, b, 255)
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
