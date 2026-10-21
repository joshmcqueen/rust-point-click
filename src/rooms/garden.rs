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
        (408.0..=543.0, 439.0..=580.0) if state.compartment_open => Some(ObjectId::Compartment),
        (550.0..=683.0, 439.0..=580.0) if state.compartment_open => Some(ObjectId::LooseStone),
        (408.0..=543.0, 439.0..=580.0) if !state.compartment_open => Some(ObjectId::LooseStone),
        (730.0..=1195.0, 430.0..=860.0) => Some(ObjectId::Fountain),
        _ => None,
    }
}

pub fn approach(object: ObjectId) -> Point {
    match object {
        ObjectId::Fountain => Point::new(960.0, 900.0),
        _ => Point::new(465.0, 700.0),
    }
}

pub fn draw_background(state: &GameState, background: &Texture2D) {
    // A dark reveal makes the key clearly readable against weathered stone.
    if state.compartment_open {
        draw_rectangle(408.0, 439.0, 135.0, 141.0, rgb(88, 88, 65));
        draw_rectangle(416.0, 447.0, 119.0, 125.0, rgb(45, 57, 46));
        draw_rectangle(416.0, 565.0, 119.0, 15.0, rgb(126, 123, 87));
        if !state.key_collected {
            draw_rectangle(435.0, 535.0, 48.0, 9.0, rgb(244, 197, 79));
            draw_rectangle(430.0, 527.0, 19.0, 25.0, rgb(244, 197, 79));
            draw_rectangle(436.0, 533.0, 7.0, 12.0, rgb(74, 74, 43));
            draw_rectangle(473.0, 540.0, 8.0, 16.0, rgb(244, 197, 79));
            draw_rectangle(461.0, 540.0, 7.0, 12.0, rgb(244, 197, 79));
        }
    }
    if state.compartment_open {
        // Reuse the painted stone as a moved piece; the closed stone is already
        // in the background. Source coordinates refer to the original PNG.
        draw_texture_ex(
            background,
            550.0,
            439.0,
            WHITE,
            DrawTextureParams {
                source: Some(Rect::new(409.0, 332.0, 133.0, 107.0)),
                dest_size: Some(vec2(133.0 * 1920.0 / 1923.0, 107.0 * 1080.0 / 817.0)),
                ..Default::default()
            },
        );
    }
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
