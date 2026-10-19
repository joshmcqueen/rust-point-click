use crate::model::{GameState, ObjectId};
use crate::world::Point;
use macroquad::prelude::*;

fn rgb(r: u8, g: u8, b: u8) -> Color {
    Color::from_rgba(r, g, b, 255)
}

pub fn hotspot(p: Point, state: &GameState) -> Option<ObjectId> {
    match (p.x, p.y) {
        (425.0..=495.0, 520.0..=575.0) if state.compartment_open && !state.key_collected => {
            Some(ObjectId::Key)
        }
        (390.0..=540.0, 480.0..=610.0) if state.compartment_open => Some(ObjectId::Compartment),
        (550.0..=690.0, 490.0..=610.0) if state.compartment_open => Some(ObjectId::LooseStone),
        (390.0..=550.0, 480.0..=620.0) if !state.compartment_open => Some(ObjectId::LooseStone),
        (810.0..=1110.0, 445.0..=855.0) => Some(ObjectId::Fountain),
        _ => None,
    }
}

pub fn approach(object: ObjectId) -> Point {
    match object {
        ObjectId::Fountain => Point::new(960.0, 900.0),
        _ => Point::new(465.0, 700.0),
    }
}

pub fn draw_background(state: &GameState) {
    clear_background(rgb(133, 191, 200));
    draw_rectangle(0.0, 170.0, 1920.0, 250.0, rgb(168, 210, 208));
    draw_rectangle(1470.0, 90.0, 100.0, 100.0, rgb(250, 231, 155));
    for (x, y) in [(180.0, 120.0), (610.0, 170.0), (1150.0, 95.0)] {
        draw_rectangle(x, y, 190.0, 24.0, rgb(224, 230, 204));
        draw_rectangle(x + 30.0, y - 20.0, 100.0, 24.0, rgb(224, 230, 204));
    }
    // Blocky cypress trees beyond the enclosing garden wall.
    for (i, x) in [90.0, 280.0, 720.0, 1280.0, 1600.0, 1800.0]
        .iter()
        .enumerate()
    {
        let top = 160.0 + (i % 3) as f32 * 40.0;
        draw_rectangle(*x - 12.0, top + 80.0, 24.0, 280.0, rgb(102, 94, 60));
        draw_rectangle(*x - 50.0, top, 100.0, 210.0, rgb(66, 112, 77));
        draw_rectangle(*x - 30.0, top - 32.0, 60.0, 260.0, rgb(83, 132, 82));
        draw_rectangle(*x - 16.0, top - 50.0, 32.0, 270.0, rgb(106, 148, 87));
    }
    draw_rectangle(0.0, 380.0, 1920.0, 280.0, rgb(116, 118, 92));
    for row in 0..4 {
        for col in -1..13 {
            let x = col as f32 * 174.0 + (row % 2) as f32 * 87.0;
            let y = 385.0 + row as f32 * 67.0;
            let shade = ((col + 2 + row * 3) % 5) as u8 * 4;
            draw_rectangle(
                x + 3.0,
                y,
                168.0,
                61.0,
                rgb(172 + shade, 167 + shade, 131 + shade),
            );
            draw_rectangle(x + 7.0, y + 3.0, 158.0, 5.0, rgb(217, 207, 164));
        }
    }
    draw_rectangle(0.0, 363.0, 1920.0, 22.0, rgb(213, 205, 160));
    draw_rectangle(0.0, 650.0, 1920.0, 430.0, rgb(137, 137, 105));
    for row in 0..5 {
        let y = 660.0 + row as f32 * 91.0;
        for col in -1..10 {
            let x = col as f32 * 235.0 + (row % 2) as f32 * 118.0;
            let shade = ((col + 2 + row * 2) % 4) as u8 * 5;
            draw_rectangle(
                x + 4.0,
                y,
                227.0,
                84.0,
                rgb(186 + shade, 177 + shade, 139 + shade),
            );
            draw_rectangle(x + 9.0, y + 4.0, 216.0, 5.0, rgb(224, 213, 171));
        }
    }
    // Ivy and flowers remain outside the open walking strip.
    for i in 0..75 {
        let x = ((i * 139) % 1710) as f32;
        let y = 394.0 + ((i * 41) % 225) as f32;
        if (350.0..710.0).contains(&x) {
            continue;
        }
        draw_rectangle(x, y, 27.0, 18.0, rgb(84, 121 + (i % 4) as u8 * 8, 68));
        draw_rectangle(x + 9.0, y - 10.0, 15.0, 32.0, rgb(107, 148, 80));
        if i % 7 == 0 {
            draw_rectangle(x + 9.0, y, 10.0, 10.0, rgb(221, 159, 113));
        }
    }
    // A dark reveal makes the key clearly readable against weathered stone.
    if state.compartment_open {
        draw_rectangle(390.0, 480.0, 150.0, 130.0, rgb(88, 88, 65));
        draw_rectangle(402.0, 492.0, 126.0, 106.0, rgb(45, 57, 46));
        draw_rectangle(402.0, 583.0, 126.0, 15.0, rgb(126, 123, 87));
        if !state.key_collected {
            draw_rectangle(435.0, 535.0, 48.0, 9.0, rgb(244, 197, 79));
            draw_rectangle(430.0, 527.0, 19.0, 25.0, rgb(244, 197, 79));
            draw_rectangle(436.0, 533.0, 7.0, 12.0, rgb(74, 74, 43));
            draw_rectangle(473.0, 540.0, 8.0, 16.0, rgb(244, 197, 79));
            draw_rectangle(461.0, 540.0, 7.0, 12.0, rgb(244, 197, 79));
        }
    }
    let stone_x = if state.compartment_open { 550.0 } else { 390.0 };
    draw_rectangle(stone_x + 6.0, 493.0, 145.0, 124.0, rgb(96, 96, 72));
    draw_rectangle(stone_x, 480.0, 140.0, 122.0, rgb(193, 183, 143));
    draw_rectangle(stone_x + 6.0, 484.0, 128.0, 8.0, rgb(234, 218, 169));
    draw_rectangle(stone_x + 126.0, 493.0, 14.0, 109.0, rgb(142, 137, 104));
    draw_rectangle(stone_x + 54.0, 526.0, 31.0, 7.0, rgb(157, 150, 112));
    draw_rectangle(stone_x + 65.0, 515.0, 7.0, 30.0, rgb(157, 150, 112));
    // Matching passage back into the cool archive chamber.
    draw_rectangle(1760.0, 355.0, 160.0, 450.0, rgb(40, 56, 57));
    draw_rectangle(1810.0, 388.0, 110.0, 390.0, rgb(29, 42, 48));
    for row in 0..7 {
        draw_rectangle(
            1720.0,
            350.0 + row as f32 * 65.0,
            45.0,
            59.0,
            rgb(194, 184, 144),
        );
    }
    draw_rectangle(1710.0, 330.0, 210.0, 26.0, rgb(226, 213, 169));
    draw_rectangle(1730.0, 805.0, 190.0, 18.0, rgb(214, 203, 159));
}

pub fn draw_fountain(time: f32) {
    draw_ellipse(
        960.0,
        852.0,
        170.0,
        18.0,
        0.0,
        Color::new(0.20, 0.25, 0.15, 0.35),
    );
    draw_rectangle(810.0, 730.0, 300.0, 125.0, rgb(117, 128, 113));
    draw_rectangle(820.0, 744.0, 280.0, 101.0, rgb(165, 171, 141));
    draw_rectangle(806.0, 720.0, 308.0, 26.0, rgb(220, 214, 174));
    draw_rectangle(830.0, 693.0, 260.0, 40.0, rgb(121, 149, 131));
    draw_rectangle(841.0, 702.0, 238.0, 29.0, rgb(81, 157, 169));
    for i in 0..7 {
        let x = 850.0 + i as f32 * 31.0;
        let y = 710.0 + ((time * 2.0 + i as f32).sin() * 6.0).round();
        draw_rectangle(x, y, 21.0, 4.0, rgb(171, 217, 207));
    }
    draw_rectangle(937.0, 535.0, 46.0, 182.0, rgb(160, 171, 145));
    draw_rectangle(942.0, 540.0, 10.0, 171.0, rgb(217, 214, 174));
    draw_rectangle(894.0, 525.0, 132.0, 28.0, rgb(202, 205, 168));
    draw_rectangle(907.0, 511.0, 106.0, 16.0, rgb(91, 161, 167));
    draw_rectangle(947.0, 461.0, 26.0, 57.0, rgb(179, 192, 158));
    draw_rectangle(951.0, 445.0, 18.0, 20.0, rgb(220, 218, 178));
    // Quantized falling droplets animate without smoothing away pixel edges.
    for i in 0..12 {
        let y = 550.0 + ((time * 85.0 + i as f32 * 16.0) % 150.0 / 4.0).floor() * 4.0;
        for x in [909.0, 1003.0] {
            draw_rectangle(x, y, 6.0, 12.0, rgb(179, 224, 218));
        }
    }
    for x in [838.0, 905.0, 972.0, 1039.0] {
        draw_rectangle(x, 769.0, 40.0, 47.0, rgb(145, 156, 127));
        draw_rectangle(x + 5.0, 774.0, 30.0, 5.0, rgb(191, 195, 155));
    }
    draw_rectangle(806.0, 841.0, 308.0, 14.0, rgb(128, 141, 115));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn key_hotspot_requires_reveal_and_disappears_after_pickup() {
        let mut state = GameState::default();
        let key = Point::new(460.0, 545.0);
        assert_eq!(hotspot(key, &state), Some(ObjectId::LooseStone));
        state.compartment_open = true;
        assert_eq!(hotspot(key, &state), Some(ObjectId::Key));
        assert_eq!(
            hotspot(Point::new(600.0, 550.0), &state),
            Some(ObjectId::LooseStone)
        );
        state.key_collected = true;
        assert_eq!(hotspot(key, &state), Some(ObjectId::Compartment));
    }
}
