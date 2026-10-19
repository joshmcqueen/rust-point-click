//! Small room interactions; UI selection stays separate from persistent room state.
use crate::world::Point;

pub fn hotspot(p: Point) -> Option<&'static str> {
    match (p.x, p.y) {
        (650.0..=850.0, 680.0..=855.0) => Some("wooden chest"),
        (780.0..=1140.0, 130.0..=640.0) => Some("archive gate"),
        (405.0..=475.0, 365.0..=515.0) => Some("torch"),
        (1250.0..=1450.0, 365.0..=490.0) => Some("inscription"),
        _ => None,
    }
}

/// Each object has one intentional place for the explorer to stand.
pub fn approach_point(object: &str) -> Point {
    match object {
        "wooden chest" => Point::new(750.0, 900.0),
        "torch" => Point::new(480.0, 710.0),
        "inscription" => Point::new(1350.0, 710.0),
        _ => Point::new(960.0, 700.0),
    }
}

pub struct PendingAction {
    pub verb: String,
    pub item: Option<String>,
    pub object: &'static str,
}

impl PendingAction {
    pub fn sentence(&self) -> String {
        match self.item.as_deref() {
            Some(item) if self.verb == "Give" => format!("Give {item} to {}", self.object),
            Some(item) if self.verb == "Use" => format!("Use {item} with {}", self.object),
            _ => format!("{} {}", self.verb, self.object),
        }
    }
}

#[derive(Default)]
pub struct Interaction {
    pub pending: Option<PendingAction>,
}

impl Interaction {
    pub fn start(
        &mut self,
        actor: &mut crate::world::Character,
        verb: &str,
        item: Option<&str>,
        object: &'static str,
    ) {
        actor.walk_to(approach_point(object));
        // Snapshot selection: later UI changes cannot alter an action in flight.
        self.pending = Some(PendingAction {
            verb: verb.into(),
            item: item.map(str::to_owned),
            object,
        });
    }

    pub fn walk(&mut self, actor: &mut crate::world::Character, point: Point) {
        self.pending = None;
        actor.walk_to(point);
    }

    pub fn finish(&mut self, actor: &crate::world::Character, room: &mut Room) -> Option<String> {
        let action = self.pending.as_ref()?;
        if actor.moving() || actor.position != approach_point(action.object) {
            return None;
        }
        let action = self.pending.take()?;
        Some(room.act(&action.verb, action.item.as_deref(), action.object))
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
            ("Look at", _, "wooden chest") => {
                "A heavy oak chest, banded with brass. Best walk around it.".into()
            }
            ("Open", _, "wooden chest") => "The lid is rusted shut.".into(),
            ("Push" | "Pull" | "Pick up", _, "wooden chest") => "Far too heavy to move.".into(),
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

#[cfg(test)]
mod walking_tests {
    use super::*;
    use crate::world::Character;

    #[test]
    fn action_waits_for_arrival_and_runs_once() {
        let mut actor = Character::new(Point::new(200.0, 900.0));
        let mut room = Room::default();
        let mut interaction = Interaction::default();
        interaction.start(&mut actor, "Use", Some("brass key"), "archive gate");
        assert_eq!(actor.target, approach_point("archive gate"));
        actor.update(0.1);
        assert!(interaction.finish(&actor, &mut room).is_none());
        assert!(!room.unlocked);
        actor.update(10.0);
        assert!(interaction.finish(&actor, &mut room).is_some());
        assert!(room.unlocked);
        assert!(interaction.finish(&actor, &mut room).is_none());
    }

    #[test]
    fn new_action_replaces_old_and_empty_walk_cancels() {
        let mut actor = Character::new(Point::new(200.0, 900.0));
        let mut room = Room::default();
        let mut interaction = Interaction::default();
        interaction.start(&mut actor, "Use", Some("brass key"), "archive gate");
        interaction.start(&mut actor, "Look at", None, "torch");
        assert_eq!(actor.target, approach_point("torch"));
        actor.update(10.0);
        let response = interaction.finish(&actor, &mut room).unwrap();
        assert!(response.contains("burning"));
        assert!(!room.unlocked);
        interaction.start(&mut actor, "Use", Some("brass key"), "archive gate");
        interaction.walk(&mut actor, Point::new(1500.0, 850.0));
        actor.update(10.0);
        assert!(interaction.finish(&actor, &mut room).is_none());
        assert!(!room.unlocked);
    }

    #[test]
    fn already_at_object_acts_without_extra_movement() {
        let mut actor = Character::new(approach_point("archive gate"));
        let mut room = Room::default();
        let mut interaction = Interaction::default();
        interaction.start(&mut actor, "Use", Some("brass key"), "archive gate");
        assert!(!actor.moving());
        assert!(interaction.finish(&actor, &mut room).is_some());
        assert!(room.unlocked);
    }
}

#[cfg(test)]
mod obstacle_tests {
    use super::*;
    use crate::world::Character;

    #[test]
    fn chest_action_waits_until_routed_approach_is_reached() {
        let mut actor = Character::new(Point::new(750.0, 680.0));
        let mut interaction = Interaction::default();
        let mut room = Room::default();
        interaction.start(&mut actor, "Look at", None, "wooden chest");
        for _ in 0..300 {
            actor.update(1.0 / 60.0);
            assert!(!crate::navigation::blocked(actor.position));
            if actor.moving() {
                assert!(interaction.finish(&actor, &mut room).is_none());
            } else {
                break;
            }
        }
        assert_eq!(actor.position, approach_point("wooden chest"));
        assert!(interaction
            .finish(&actor, &mut room)
            .unwrap()
            .contains("oak chest"));
    }
}
