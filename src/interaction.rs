//! Queued actions snapshot intent; session state changes only on arrival.
use crate::{
    model::{GameState, ItemId, ObjectId, RoomId, Verb},
    rooms,
    world::{Character, Point},
};

#[derive(Clone, Copy, Debug)]
pub enum Action {
    Object {
        verb: Verb,
        item: Option<ItemId>,
        object: ObjectId,
    },
    Exit(RoomId),
}
pub struct PendingAction {
    pub room: RoomId,
    pub action: Action,
    pub destination: Point,
}
impl PendingAction {
    pub fn sentence(&self) -> String {
        match self.action {
            Action::Exit(room) => format!("Go to {}", room.name()),
            Action::Object {
                verb,
                item: Some(item),
                object,
            } if matches!(verb, Verb::Use | Verb::Give) => format!(
                "{} {} {} {}",
                verb.name(),
                item.name(),
                if verb == Verb::Give { "to" } else { "with" },
                object.name()
            ),
            Action::Object { verb, object, .. } => format!("{} {}", verb.name(), object.name()),
        }
    }
}
#[derive(Default)]
pub struct Interaction {
    pub pending: Option<PendingAction>,
}
pub enum Outcome {
    Feedback(String),
    Transition,
    Completed,
}
impl Interaction {
    pub fn start(&mut self, actor: &mut Character, room: RoomId, action: Action) {
        let destination = match action {
            Action::Object { object, .. } => rooms::approach(room, object),
            Action::Exit(_) => rooms::exit_point(room),
        };
        actor.walk_to(destination);
        self.pending = Some(PendingAction {
            room,
            action,
            destination,
        });
    }
    pub fn walk(&mut self, actor: &mut Character, point: Point) {
        self.pending = None;
        actor.walk_to(point);
    }
    pub fn finish(&mut self, actor: &mut Character, game: &mut GameState) -> Option<Outcome> {
        let pending = self.pending.as_ref()?;
        if pending.room != game.room {
            self.pending = None;
            return None;
        }
        if actor.moving() || actor.position != pending.destination {
            return None;
        }
        let pending = self.pending.take()?;
        Some(match pending.action {
            Action::Exit(destination) => {
                game.room = destination;
                actor.enter(
                    rooms::entry_point(destination),
                    rooms::floor(destination),
                    rooms::obstacle(destination),
                );
                Outcome::Transition
            }
            Action::Object { verb, item, object } => act(game, verb, item, object),
        })
    }
}

fn act(game: &mut GameState, verb: Verb, item: Option<ItemId>, object: ObjectId) -> Outcome {
    use ObjectId::*;
    use Verb::*;
    // Room membership is explicit, so stale or invalid targets cannot mutate state.
    let belongs = match object {
        Chest | Gate | Torch | Inscription => game.room == RoomId::Chamber,
        Fountain | LooseStone | Compartment | Key => game.room == RoomId::Garden,
    };
    if !belongs {
        return Outcome::Feedback("That isn't here.".into());
    }
    let message = match (verb, item, object) {
        (LookAt, _, Chest) => "A heavy oak chest, banded with brass. Best walk around it.",
        (Open, _, Chest) => "The lid is rusted shut.",
        (Push | Pull | PickUp, _, Chest) => "Far too heavy to move.",
        (LookAt, _, Gate) => "A heavy iron gate. The lock bears a tiny sun.",
        (LookAt, _, Torch) => "Still burning. Someone has been here recently.",
        (LookAt, _, Inscription) => "Only the keeper of the sun may enter the archive.",
        (LookAt, _, Fountain) => {
            "Clear water dances over weathered stone. A welcome patch of sunlight."
        }
        (LookAt, _, LooseStone) => {
            "This stone sits proud of the wall. It might slide inward with a push."
        }
        (Push, _, LooseStone) if !game.compartment_open => {
            game.compartment_open = true;
            "The stone slides inward. A secret compartment holds a small brass key!"
        }
        (Push, _, LooseStone) => "The stone has already opened the compartment.",
        (LookAt, _, Compartment) if game.compartment_open => {
            if game.key_collected {
                "The secret compartment is empty."
            } else {
                "A brass key rests inside. Its bow bears a tiny sun."
            }
        }
        (LookAt, _, Key) if game.compartment_open && !game.key_collected => {
            "A brass key with a tiny sun on its bow."
        }
        (PickUp, _, Key) if game.compartment_open && !game.key_collected => {
            game.key_collected = true;
            game.inventory.push(ItemId::BrassKey);
            "You take the brass key."
        }
        (Use, Some(ItemId::BrassKey), Gate) if game.inventory.contains(&ItemId::BrassKey) => {
            game.unlocked = true;
            "The brass key turns. The gate is unlocked."
        }
        (Open, _, Gate) if game.unlocked => {
            game.gate_open = true;
            if !game.completed {
                game.completed = true;
                return Outcome::Completed;
            }
            "The gate swings open. Beyond it, the archive waits."
        }
        (Open, _, Gate) => "It's locked.",
        (Close, _, Gate) => {
            game.gate_open = false;
            "The gate is closed."
        }
        (PickUp, _, Torch) => "It's fixed firmly to the wall.",
        (TalkTo, _, _) => "No reply. Just the sounds of this quiet place.",
        _ => "That doesn't seem to help.",
    };
    Outcome::Feedback(message.into())
}

#[cfg(test)]
mod tests {
    use super::*;
    fn actor(room: RoomId, p: Point) -> Character {
        Character::new(p, rooms::floor(room), rooms::obstacle(room))
    }
    #[test]
    fn puzzle_requires_revealing_collecting_unlocking_and_opening() {
        let mut g = GameState::default();
        assert_eq!(g.inventory.len(), 9);
        act(&mut g, Verb::Open, None, ObjectId::Gate);
        act(&mut g, Verb::Use, Some(ItemId::BrassKey), ObjectId::Gate);
        assert!(!g.gate_open && !g.unlocked);
        g.room = RoomId::Garden;
        act(&mut g, Verb::PickUp, None, ObjectId::Key);
        assert!(!g.key_collected);
        act(&mut g, Verb::Push, None, ObjectId::LooseStone);
        act(&mut g, Verb::PickUp, None, ObjectId::Key);
        act(&mut g, Verb::PickUp, None, ObjectId::Key);
        act(&mut g, Verb::Push, None, ObjectId::LooseStone);
        assert_eq!(
            g.inventory
                .iter()
                .filter(|&&i| i == ItemId::BrassKey)
                .count(),
            1
        );
        g.room = RoomId::Chamber;
        act(&mut g, Verb::Use, Some(ItemId::OldCoin), ObjectId::Gate);
        assert!(!g.unlocked);
        act(&mut g, Verb::Use, Some(ItemId::BrassKey), ObjectId::Gate);
        assert!(g.unlocked && !g.gate_open && !g.completed);
        assert!(matches!(
            act(&mut g, Verb::Open, None, ObjectId::Gate),
            Outcome::Completed
        ));
        act(&mut g, Verb::Close, None, ObjectId::Gate);
        assert!(matches!(
            act(&mut g, Verb::Open, None, ObjectId::Gate),
            Outcome::Feedback(_)
        ));
        assert!(g.completed && g.gate_open && g.inventory.contains(&ItemId::BrassKey));
    }
    #[test]
    fn exits_wait_cancel_replace_and_preserve_puzzle_state() {
        let mut g = GameState::default();
        let mut a = actor(g.room, Point::new(960.0, 810.0));
        let mut i = Interaction::default();
        i.start(&mut a, g.room, Action::Exit(RoomId::Garden));
        a.update(0.1);
        assert!(i.finish(&mut a, &mut g).is_none());
        i.walk(&mut a, Point::new(1400.0, 900.0));
        a.update(10.0);
        assert!(i.finish(&mut a, &mut g).is_none());
        assert_eq!(g.room, RoomId::Chamber);
        i.start(
            &mut a,
            g.room,
            Action::Object {
                verb: Verb::Open,
                item: None,
                object: ObjectId::Gate,
            },
        );
        i.start(&mut a, g.room, Action::Exit(RoomId::Garden));
        a.update(10.0);
        assert!(matches!(
            i.finish(&mut a, &mut g),
            Some(Outcome::Transition)
        ));
        assert_eq!(g.room, RoomId::Garden);
        assert!(!a.moving() && i.pending.is_none());
        assert!(rooms::exit(g.room, a.position).is_none());
        act(&mut g, Verb::Push, None, ObjectId::LooseStone);
        act(&mut g, Verb::PickUp, None, ObjectId::Key);
        for dest in [RoomId::Chamber, RoomId::Garden, RoomId::Chamber] {
            i.start(&mut a, g.room, Action::Exit(dest));
            a.update(10.0);
            assert!(matches!(
                i.finish(&mut a, &mut g),
                Some(Outcome::Transition)
            ));
            assert!(g.compartment_open && g.key_collected);
            assert!(!a.moving());
            assert!(i.finish(&mut a, &mut g).is_none());
        }
    }
    #[test]
    fn actions_snapshot_selection_wait_and_execute_once() {
        let mut g = GameState::default();
        let mut a = actor(g.room, Point::new(200.0, 900.0));
        let mut i = Interaction::default();
        i.start(
            &mut a,
            g.room,
            Action::Object {
                verb: Verb::LookAt,
                item: None,
                object: ObjectId::Chest,
            },
        );
        a.update(0.1);
        assert!(i.finish(&mut a, &mut g).is_none());
        a.update(10.0);
        assert!(matches!(
            i.finish(&mut a, &mut g),
            Some(Outcome::Feedback(_))
        ));
        assert!(i.finish(&mut a, &mut g).is_none());
        i.start(
            &mut a,
            g.room,
            Action::Object {
                verb: Verb::LookAt,
                item: None,
                object: ObjectId::Chest,
            },
        );
        assert!(i.finish(&mut a, &mut g).is_some());
        i.start(&mut a, g.room, Action::Exit(RoomId::Garden));
        g.room = RoomId::Garden;
        assert!(i.finish(&mut a, &mut g).is_none());
        assert!(i.pending.is_none());
    }

    fn object(verb: Verb, object: ObjectId) -> Action {
        Action::Object {
            verb,
            item: None,
            object,
        }
    }

    fn arrive(i: &mut Interaction, a: &mut Character, g: &mut GameState) -> Outcome {
        // Advance normal walking at 60 Hz, including any obstacle detours.
        for _ in 0..1200 {
            a.update(1.0 / 60.0);
            if let Some(outcome) = i.finish(a, g) {
                assert!(i.pending.is_none());
                assert!(!a.moving());
                return outcome;
            }
        }
        panic!("queued action never reached its approach point");
    }

    #[test]
    fn replacing_an_object_action_discards_its_effect() {
        let mut g = GameState {
            room: RoomId::Garden,
            ..GameState::default()
        };
        let mut a = actor(g.room, rooms::entry_point(g.room));
        let mut i = Interaction::default();
        i.start(&mut a, g.room, object(Verb::Push, ObjectId::LooseStone));
        a.update(0.1);
        assert!(i.finish(&mut a, &mut g).is_none());
        i.start(&mut a, g.room, object(Verb::LookAt, ObjectId::Fountain));
        match arrive(&mut i, &mut a, &mut g) {
            Outcome::Feedback(message) => assert!(message.contains("Clear water")),
            _ => panic!("replacement should examine the fountain"),
        }
        assert_eq!(a.position, rooms::approach(g.room, ObjectId::Fountain));
        assert!(!g.compartment_open);
        assert!(i.finish(&mut a, &mut g).is_none());
    }

    #[test]
    fn replacing_exit_with_object_keeps_player_in_current_room() {
        let mut g = GameState::default();
        let mut a = actor(g.room, Point::new(1500.0, 900.0));
        let mut i = Interaction::default();
        i.start(&mut a, g.room, Action::Exit(RoomId::Garden));
        a.update(0.1);
        assert!(i.finish(&mut a, &mut g).is_none());
        i.start(&mut a, g.room, object(Verb::LookAt, ObjectId::Torch));
        match arrive(&mut i, &mut a, &mut g) {
            Outcome::Feedback(message) => assert!(message.contains("Still burning")),
            _ => panic!("replacement should examine the torch"),
        }
        assert_eq!(g.room, RoomId::Chamber);
        assert_eq!(a.position, rooms::approach(g.room, ObjectId::Torch));
        assert!(i.finish(&mut a, &mut g).is_none());
    }

    #[test]
    fn queued_puzzle_completes_after_walking_and_only_announces_once() {
        let mut g = GameState::default();
        let mut a = actor(g.room, Point::new(1500.0, 900.0));
        let mut i = Interaction::default();
        i.start(&mut a, g.room, Action::Exit(RoomId::Garden));
        assert!(matches!(
            arrive(&mut i, &mut a, &mut g),
            Outcome::Transition
        ));
        assert_eq!(g.room, RoomId::Garden);
        assert_eq!(a.position, rooms::entry_point(g.room));
        i.start(&mut a, g.room, object(Verb::Push, ObjectId::LooseStone));
        assert!(i.finish(&mut a, &mut g).is_none());
        assert!(!g.compartment_open);
        assert!(matches!(
            arrive(&mut i, &mut a, &mut g),
            Outcome::Feedback(_)
        ));
        assert!(g.compartment_open);

        // Leave the compartment before collecting, proving pickup waits for arrival.
        i.walk(&mut a, rooms::entry_point(g.room));
        a.update(10.0);
        i.start(&mut a, g.room, object(Verb::PickUp, ObjectId::Key));
        a.update(0.1);
        assert!(i.finish(&mut a, &mut g).is_none());
        assert!(!g.key_collected);
        assert_eq!(g.inventory.len(), 9);
        assert!(matches!(
            arrive(&mut i, &mut a, &mut g),
            Outcome::Feedback(_)
        ));
        assert_eq!(a.position, rooms::approach(g.room, ObjectId::Key));
        assert!(g.key_collected);
        assert!(i.finish(&mut a, &mut g).is_none());
        i.start(&mut a, g.room, object(Verb::PickUp, ObjectId::Key));
        assert!(matches!(
            arrive(&mut i, &mut a, &mut g),
            Outcome::Feedback(_)
        ));
        assert_eq!(
            g.inventory
                .iter()
                .filter(|&&item| item == ItemId::BrassKey)
                .count(),
            1
        );

        i.start(&mut a, g.room, Action::Exit(RoomId::Chamber));
        assert!(matches!(
            arrive(&mut i, &mut a, &mut g),
            Outcome::Transition
        ));
        assert_eq!(g.room, RoomId::Chamber);
        assert_eq!(a.position, rooms::entry_point(g.room));
        assert!(g.compartment_open && g.key_collected);
        i.start(
            &mut a,
            g.room,
            Action::Object {
                verb: Verb::Use,
                item: Some(ItemId::BrassKey),
                object: ObjectId::Gate,
            },
        );
        assert!(i.finish(&mut a, &mut g).is_none());
        assert!(!g.unlocked);
        assert!(matches!(
            arrive(&mut i, &mut a, &mut g),
            Outcome::Feedback(_)
        ));
        assert!(g.unlocked && !g.completed && !g.gate_open);
        i.start(&mut a, g.room, object(Verb::Open, ObjectId::Gate));
        assert!(matches!(arrive(&mut i, &mut a, &mut g), Outcome::Completed));
        assert!(g.completed && g.gate_open);
        i.start(&mut a, g.room, object(Verb::Close, ObjectId::Gate));
        assert!(matches!(
            arrive(&mut i, &mut a, &mut g),
            Outcome::Feedback(_)
        ));
        assert!(g.completed && !g.gate_open);
        i.start(&mut a, g.room, object(Verb::Open, ObjectId::Gate));
        assert!(matches!(
            arrive(&mut i, &mut a, &mut g),
            Outcome::Feedback(_)
        ));
        assert!(g.completed && g.gate_open);
        assert!(g.inventory.contains(&ItemId::BrassKey));
        assert!(i.finish(&mut a, &mut g).is_none());
    }
}
