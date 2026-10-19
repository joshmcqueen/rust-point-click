//! Small room interactions; UI selection stays separate from persistent room state.
use crate::world::Point;

pub fn hotspot(p: Point) -> Option<&'static str> {
    match (p.x, p.y) {
        (780.0..=1140.0, 130.0..=640.0) => Some("archive gate"),
        (405.0..=475.0, 365.0..=515.0) => Some("torch"),
        (1250.0..=1450.0, 365.0..=490.0) => Some("inscription"),
        _ => None,
    }
}

#[derive(Default)]
pub struct Room {
    pub gate_open: bool,
    pub unlocked: bool,
}

impl Room {
    pub fn act(&mut self, verb: &str, item: Option<&str>, object: &str) -> String {
        match (verb, item, object) {
            ("Look at", _, "archive gate") => {
                "A heavy iron gate. The lock bears a tiny sun.".into()
            }
            ("Look at", _, "torch") => "Still burning. Someone has been here recently.".into(),
            ("Look at", _, "inscription") => {
                "Only the keeper of the sun may enter the archive.".into()
            }
            ("Use", Some("brass key"), "archive gate") => {
                self.unlocked = true;
                "The brass key turns. The gate is unlocked.".into()
            }
            ("Open", _, "archive gate") if self.unlocked => {
                self.gate_open = true;
                "The gate swings open. Beyond it, the archive waits.".into()
            }
            ("Open", _, "archive gate") => "It's locked.".into(),
            ("Close", _, "archive gate") => {
                self.gate_open = false;
                "The gate is closed.".into()
            }
            ("Pick up", _, "torch") => "It's fixed firmly to the wall.".into(),
            ("Talk to", _, _) => "No reply. Just the crackle of the torch.".into(),
            _ => "That doesn't seem to help.".into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn gate_requires_key_and_keeps_state_when_closed() {
        let mut room = Room::default();
        room.act("Open", None, "archive gate");
        assert!(!room.gate_open);
        room.act("Use", Some("scrap of paper"), "archive gate");
        assert!(!room.unlocked);
        room.act("Use", Some("brass key"), "archive gate");
        room.act("Open", None, "archive gate");
        assert!(room.gate_open);
        room.act("Close", None, "archive gate");
        assert!(!room.gate_open && room.unlocked);
    }
}
