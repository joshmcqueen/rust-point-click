//! A small verb-and-inventory interface, expressed in the room's design coordinates.
use crate::model::{ItemId, Verb};
use crate::world::Point;
use macroquad::prelude::*;

const VERBS: [Verb; 9] = [
    Verb::Open,
    Verb::Close,
    Verb::Give,
    Verb::PickUp,
    Verb::LookAt,
    Verb::TalkTo,
    Verb::Push,
    Verb::Pull,
    Verb::Use,
];
const TEAL: Color = Color::new(0.20, 0.77, 0.75, 1.0);
const GOLD: Color = Color::new(1.0, 0.79, 0.38, 1.0);

pub struct Interface {
    pub verb: Verb,
    pub selected: Option<ItemId>,
    pub row: usize,
}

impl Interface {
    pub fn new() -> Self {
        Self {
            verb: Verb::LookAt,
            selected: None,
            row: 0,
        }
    }

    fn max_row(items: &[ItemId]) -> usize {
        items.len().div_ceil(4).saturating_sub(2)
    }

    pub fn scroll(&mut self, delta: f32, items: &[ItemId]) {
        if delta > 0.0 {
            self.row = self.row.saturating_sub(1);
        }
        if delta < 0.0 {
            self.row = (self.row + 1).min(Self::max_row(items));
        }
    }

    pub fn click(&mut self, point: Point, items: &[ItemId]) {
        for (index, verb) in VERBS.iter().enumerate() {
            if contains(verb_rect(index), point) {
                self.verb = *verb;
                if self.verb != Verb::Use && self.verb != Verb::Give && self.verb != Verb::LookAt {
                    self.selected = None;
                }
                return;
            }
        }
        if contains(Rect::new(1830.0, 855.0, 65.0, 80.0), point) {
            self.scroll(1.0, items);
            return;
        }
        if contains(Rect::new(1830.0, 955.0, 65.0, 80.0), point) {
            self.scroll(-1.0, items);
            return;
        }
        for slot in 0..8 {
            let index = self.row * 4 + slot;
            if index < items.len() && contains(item_rect(slot), point) {
                self.selected = if self.selected == Some(items[index]) {
                    None
                } else {
                    Some(items[index])
                };
                if self.selected.is_some() && self.verb != Verb::LookAt && self.verb != Verb::Give {
                    self.verb = Verb::Use;
                }
                return;
            }
        }
    }

    pub fn phrase(&self, hotspot: Option<&str>) -> String {
        if self.verb == Verb::LookAt {
            if let Some(object) = hotspot {
                return format!("Look at {object}");
            }
        }
        let mut phrase = self.verb.name().to_string();
        if let Some(item) = self.selected {
            phrase.push(' ');
            phrase.push_str(item.name());
            if let Some(target) = hotspot.filter(|_| self.verb != Verb::LookAt) {
                phrase.push_str(if self.verb == Verb::Give {
                    " to "
                } else {
                    " with "
                });
                phrase.push_str(target);
            }
            return phrase;
        }
        if let Some(target) = hotspot {
            phrase.push(' ');
            phrase.push_str(target);
        }
        phrase
    }

    pub fn draw(&self, mouse: Option<Point>, items: &[ItemId]) {
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
            draw_text(verb.name(), rect.x + 6.0, rect.y + 47.0, 48.0, color);
            if self.verb == *verb {
                draw_rectangle(rect.x + 7.0, rect.y + 56.0, 34.0, 3.0, GOLD);
            }
        }
        for slot in 0..8 {
            let rect = item_rect(slot);
            let index = self.row * 4 + slot;
            let selected = items
                .get(index)
                .is_some_and(|item| self.selected == Some(*item));
            let hover = mouse.is_some_and(|point| contains(rect, point));
            draw_rectangle(
                rect.x,
                rect.y,
                rect.w,
                rect.h,
                Color::from_rgba(13, 33, 42, 255),
            );
            if let Some(item) = items.get(index) {
                let name = item.name();
                icon(*item, rect.x + 82.0, rect.y + 31.0);
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
            (false, 955.0, self.row < Self::max_row(items)),
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
fn icon(item: ItemId, x: f32, y: f32) {
    let brass = Color::from_rgba(206, 155, 68, 255);
    let paper = Color::from_rgba(203, 187, 145, 255);
    match item {
        ItemId::BrassKey => {
            draw_rectangle_lines(x, y - 15.0, 24.0, 24.0, 6.0, brass);
            draw_rectangle(x + 21.0, y - 8.0, 45.0, 8.0, brass);
            draw_rectangle(x + 51.0, y, 8.0, 12.0, brass);
        }
        ItemId::FoldedNote => {
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
        ItemId::PocketWatch | ItemId::OldCoin | ItemId::SmallCompass => {
            draw_circle(x + 26.0, y + 2.0, 23.0, brass);
            draw_circle(
                x + 26.0,
                y + 2.0,
                17.0,
                if item == ItemId::OldCoin {
                    paper
                } else {
                    Color::from_rgba(57, 73, 79, 255)
                },
            );
            draw_line(x + 26.0, y + 2.0, x + 36.0, y - 9.0, 4.0, WHITE);
        }
        ItemId::RedRibbon => {
            draw_rectangle(x + 15.0, y - 19.0, 16.0, 43.0, MAROON);
            draw_triangle(
                vec2(x + 2.0, y - 10.0),
                vec2(x + 50.0, y - 10.0),
                vec2(x + 26.0, y + 10.0),
                RED,
            );
        }
        ItemId::GlassVial => {
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
        ItemId::Matchbox => {
            draw_rectangle(x, y - 18.0, 57.0, 40.0, Color::from_rgba(164, 77, 49, 255));
            draw_rectangle(x + 7.0, y - 10.0, 43.0, 24.0, paper);
        }
        ItemId::WaxSeal => {
            draw_rectangle(x + 3.0, y - 17.0, 50.0, 38.0, paper);
            draw_circle(x + 28.0, y + 1.0, 13.0, MAROON);
        }
        ItemId::CopperWire => {
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
    fn selected_item_survives_scrolling() {
        let items = crate::model::GameState::default().inventory;
        assert_eq!(items.len(), 9);
        assert!(!items.contains(&ItemId::BrassKey));
        let mut ui = Interface::new();
        ui.verb = Verb::Use;
        ui.click(Point::new(980.0, 870.0), &items);
        let selected = ui.selected;
        ui.scroll(-1.0, &items);
        ui.scroll(-1.0, &items);
        assert_eq!(ui.row, 1);
        assert_eq!(ui.selected, selected);
        ui.scroll(1.0, &items);
        ui.click(Point::new(980.0, 870.0), &items);
        assert_eq!(ui.selected, None);
    }

    #[test]
    fn appended_key_can_be_selected_and_used() {
        let mut items = crate::model::GameState::default().inventory;
        items.push(ItemId::BrassKey);
        let mut ui = Interface::new();
        ui.verb = Verb::Push;
        ui.scroll(-1.0, &items);
        ui.click(Point::new(1190.0, 975.0), &items);
        assert_eq!(ui.selected, Some(ItemId::BrassKey));
        assert_eq!(ui.verb, Verb::Use);
        assert_eq!(
            ui.phrase(Some("archive gate")),
            "Use brass key with archive gate"
        );
        ui.scroll(1.0, &items);
        assert_eq!(ui.selected, Some(ItemId::BrassKey));
    }

    #[test]
    fn inventory_selection_preserves_look_and_give() {
        let items = crate::model::GameState::default().inventory;
        for verb in [Verb::LookAt, Verb::Give] {
            let mut ui = Interface::new();
            ui.verb = verb;
            ui.click(Point::new(980.0, 870.0), &items);
            assert_eq!(ui.verb, verb);
        }
    }
}
