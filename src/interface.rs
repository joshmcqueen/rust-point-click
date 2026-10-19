//! A small verb-and-inventory interface, expressed in the room's design coordinates.
use crate::world::Point;
use macroquad::prelude::*;

const VERBS: [&str; 9] = [
    "Open", "Close", "Give", "Pick up", "Look at", "Talk to", "Push", "Pull", "Use",
];
const TEAL: Color = Color::new(0.20, 0.77, 0.75, 1.0);
const GOLD: Color = Color::new(1.0, 0.79, 0.38, 1.0);

pub struct Interface {
    pub verb: &'static str,
    pub selected: Option<usize>,
    pub row: usize,
    pub items: Vec<&'static str>,
}

impl Interface {
    pub fn new() -> Self {
        Self {
            verb: "Look at",
            selected: None,
            row: 0,
            items: vec![
                "brass key",
                "folded note",
                "pocket watch",
                "old coin",
                "red ribbon",
                "glass vial",
                "matchbox",
                "small compass",
                "wax seal",
                "copper wire",
            ],
        }
    }

    fn max_row(&self) -> usize {
        self.items.len().div_ceil(4).saturating_sub(2)
    }

    pub fn scroll(&mut self, delta: f32) {
        if delta > 0.0 {
            self.row = self.row.saturating_sub(1);
        }
        if delta < 0.0 {
            self.row = (self.row + 1).min(self.max_row());
        }
    }

    pub fn click(&mut self, point: Point) {
        for (index, verb) in VERBS.iter().enumerate() {
            if contains(verb_rect(index), point) {
                self.verb = verb;
                if self.verb != "Use" && self.verb != "Give" && self.verb != "Look at" {
                    self.selected = None;
                }
                return;
            }
        }
        if contains(Rect::new(1830.0, 855.0, 65.0, 80.0), point) {
            self.scroll(1.0);
            return;
        }
        if contains(Rect::new(1830.0, 955.0, 65.0, 80.0), point) {
            self.scroll(-1.0);
            return;
        }
        for slot in 0..8 {
            let index = self.row * 4 + slot;
            if index < self.items.len() && contains(item_rect(slot), point) {
                self.selected = if self.selected == Some(index) {
                    None
                } else {
                    Some(index)
                };
                if self.selected.is_some() && self.verb != "Look at" && self.verb != "Give" {
                    self.verb = "Use";
                }
                return;
            }
        }
    }

    pub fn phrase(&self, hotspot: Option<&str>) -> String {
        let mut phrase = self.verb.to_string();
        if let Some(index) = self.selected {
            if let Some(item) = self.items.get(index) {
                phrase.push(' ');
                phrase.push_str(item);
                if let Some(target) = hotspot.filter(|_| self.verb != "Look at") {
                    phrase.push_str(if self.verb == "Give" {
                        " to "
                    } else {
                        " with "
                    });
                    phrase.push_str(target);
                }
                return phrase;
            }
        }
        if let Some(target) = hotspot {
            phrase.push(' ');
            phrase.push_str(target);
        }
        phrase
    }

    pub fn draw(&self, mouse: Option<Point>) {
        draw_rectangle(0.0, 810.0, 1920.0, 270.0, Color::from_rgba(8, 20, 28, 255));
        draw_rectangle(0.0, 810.0, 1920.0, 3.0, Color::from_rgba(47, 91, 95, 255));
        for (index, verb) in VERBS.iter().enumerate() {
            let rect = verb_rect(index);
            let hover = mouse.is_some_and(|point| contains(rect, point));
            let color = if self.verb == *verb {
                GOLD
            } else if hover {
                WHITE
            } else {
                TEAL
            };
            draw_text(verb, rect.x + 6.0, rect.y + 47.0, 48.0, color);
            if self.verb == *verb {
                draw_rectangle(rect.x + 7.0, rect.y + 56.0, 34.0, 3.0, GOLD);
            }
        }
        for slot in 0..8 {
            let rect = item_rect(slot);
            let index = self.row * 4 + slot;
            let selected = self.selected == Some(index);
            let hover = mouse.is_some_and(|point| contains(rect, point));
            draw_rectangle(
                rect.x,
                rect.y,
                rect.w,
                rect.h,
                Color::from_rgba(13, 33, 42, 255),
            );
            if let Some(name) = self.items.get(index) {
                icon(index, rect.x + 82.0, rect.y + 31.0);
                let font_size = 20;
                let width = measure_text(name, None, font_size, 1.0).width;
                draw_text(
                    name,
                    rect.x + (rect.w - width) / 2.0,
                    rect.y + 87.0,
                    font_size as f32,
                    if selected {
                        GOLD
                    } else {
                        Color::from_rgba(178, 207, 203, 255)
                    },
                );
            }
            if selected || hover {
                draw_rectangle_lines(
                    rect.x,
                    rect.y,
                    rect.w,
                    rect.h,
                    2.0,
                    if selected { GOLD } else { TEAL },
                );
            }
        }
        for (up, y, enabled) in [
            (true, 855.0, self.row > 0),
            (false, 955.0, self.row < self.max_row()),
        ] {
            let rect = Rect::new(1830.0, y, 65.0, 80.0);
            let hover = mouse.is_some_and(|point| contains(rect, point));
            let color = if !enabled {
                Color::from_rgba(45, 67, 72, 255)
            } else if hover {
                WHITE
            } else {
                TEAL
            };
            let center = vec2(1862.0, y + 40.0);
            let direction = if up { -1.0 } else { 1.0 };
            draw_triangle(
                center + vec2(0.0, direction * 16.0),
                center + vec2(-17.0, -direction * 12.0),
                center + vec2(17.0, -direction * 12.0),
                color,
            );
        }
        draw_text("INVENTORY", 965.0, 839.0, 19.0, TEAL);
    }
}

fn contains(rect: Rect, point: Point) -> bool {
    rect.contains(vec2(point.x, point.y))
}
fn verb_rect(index: usize) -> Rect {
    Rect::new(
        35.0 + (index / 3) as f32 * 300.0,
        842.0 + (index % 3) as f32 * 74.0,
        288.0,
        69.0,
    )
}
fn item_rect(slot: usize) -> Rect {
    Rect::new(
        960.0 + (slot % 4) as f32 * 214.0,
        850.0 + (slot / 4) as f32 * 102.0,
        207.0,
        96.0,
    )
}

// Deliberately simple placeholder silhouettes; no external artwork is needed.
fn icon(index: usize, x: f32, y: f32) {
    let brass = Color::from_rgba(206, 155, 68, 255);
    let paper = Color::from_rgba(203, 187, 145, 255);
    match index {
        0 => {
            draw_rectangle_lines(x, y - 15.0, 24.0, 24.0, 6.0, brass);
            draw_rectangle(x + 21.0, y - 8.0, 45.0, 8.0, brass);
            draw_rectangle(x + 51.0, y, 8.0, 12.0, brass);
        }
        1 => {
            draw_rectangle(x, y - 18.0, 56.0, 40.0, paper);
            for row in 0..3 {
                draw_rectangle(
                    x + 9.0,
                    y - 9.0 + row as f32 * 9.0,
                    36.0,
                    3.0,
                    Color::from_rgba(111, 89, 69, 255),
                );
            }
        }
        2 | 3 | 7 => {
            draw_circle(x + 26.0, y + 2.0, 23.0, brass);
            draw_circle(
                x + 26.0,
                y + 2.0,
                17.0,
                if index == 3 {
                    paper
                } else {
                    Color::from_rgba(57, 73, 79, 255)
                },
            );
            draw_line(x + 26.0, y + 2.0, x + 36.0, y - 9.0, 4.0, WHITE);
        }
        4 => {
            draw_rectangle(x + 15.0, y - 19.0, 16.0, 43.0, MAROON);
            draw_triangle(
                vec2(x + 2.0, y - 10.0),
                vec2(x + 50.0, y - 10.0),
                vec2(x + 26.0, y + 10.0),
                RED,
            );
        }
        5 => {
            draw_rectangle(x + 18.0, y - 24.0, 18.0, 12.0, brass);
            draw_rectangle(
                x + 9.0,
                y - 12.0,
                36.0,
                37.0,
                Color::from_rgba(100, 180, 184, 255),
            );
            draw_rectangle(
                x + 14.0,
                y + 5.0,
                26.0,
                15.0,
                Color::from_rgba(56, 120, 137, 255),
            );
        }
        6 => {
            draw_rectangle(x, y - 18.0, 57.0, 40.0, Color::from_rgba(164, 77, 49, 255));
            draw_rectangle(x + 7.0, y - 10.0, 43.0, 24.0, paper);
        }
        8 => {
            draw_rectangle(x + 3.0, y - 17.0, 50.0, 38.0, paper);
            draw_circle(x + 28.0, y + 1.0, 13.0, MAROON);
        }
        _ => {
            for ring in 0..3 {
                draw_rectangle_lines(
                    x + ring as f32 * 7.0,
                    y - 18.0 + ring as f32 * 6.0,
                    44.0,
                    30.0,
                    4.0,
                    Color::from_rgba(192, 107, 65, 255),
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn selected_item_survives_scrolling_and_unlocks_gate() {
        let mut ui = Interface::new();
        ui.verb = "Use";
        ui.click(Point::new(980.0, 870.0));
        ui.scroll(-1.0);
        ui.scroll(-1.0);
        assert_eq!(ui.row, 1);
        assert_eq!(
            ui.phrase(Some("archive gate")),
            "Use brass key with archive gate"
        );
        let mut room = crate::interaction::Room::default();
        room.act(ui.verb, ui.selected.map(|i| ui.items[i]), "archive gate");
        assert!(room.unlocked);
        ui.scroll(1.0);
        ui.click(Point::new(980.0, 870.0));
        assert_eq!(ui.selected, None);
    }
}
