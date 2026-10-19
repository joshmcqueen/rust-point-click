//! Stable content identities and session state; no drawing or input code.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RoomId {
    Chamber,
    Garden,
}
impl RoomId {
    pub fn name(self) -> &'static str {
        match self {
            Self::Chamber => "Ember Archive",
            Self::Garden => "Sunlit Garden",
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ObjectId {
    Chest,
    Gate,
    Torch,
    Inscription,
    Fountain,
    LooseStone,
    Compartment,
    Key,
}
impl ObjectId {
    pub fn name(self) -> &'static str {
        match self {
            Self::Chest => "wooden chest",
            Self::Gate => "archive gate",
            Self::Torch => "torch",
            Self::Inscription => "inscription",
            Self::Fountain => "fountain",
            Self::LooseStone => "loose stone",
            Self::Compartment => "secret compartment",
            Self::Key => "brass key",
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Verb {
    Open,
    Close,
    Give,
    PickUp,
    LookAt,
    TalkTo,
    Push,
    Pull,
    Use,
}
impl Verb {
    pub fn name(self) -> &'static str {
        match self {
            Self::Open => "Open",
            Self::Close => "Close",
            Self::Give => "Give",
            Self::PickUp => "Pick up",
            Self::LookAt => "Look at",
            Self::TalkTo => "Talk to",
            Self::Push => "Push",
            Self::Pull => "Pull",
            Self::Use => "Use",
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ItemId {
    BrassKey,
    FoldedNote,
    PocketWatch,
    OldCoin,
    RedRibbon,
    GlassVial,
    Matchbox,
    SmallCompass,
    WaxSeal,
    CopperWire,
}
impl ItemId {
    pub fn name(self) -> &'static str {
        match self {
            Self::BrassKey => "brass key",
            Self::FoldedNote => "folded note",
            Self::PocketWatch => "pocket watch",
            Self::OldCoin => "old coin",
            Self::RedRibbon => "red ribbon",
            Self::GlassVial => "glass vial",
            Self::Matchbox => "matchbox",
            Self::SmallCompass => "small compass",
            Self::WaxSeal => "wax seal",
            Self::CopperWire => "copper wire",
        }
    }
}
pub struct GameState {
    pub room: RoomId,
    pub inventory: Vec<ItemId>,
    pub gate_open: bool,
    pub unlocked: bool,
    pub compartment_open: bool,
    pub key_collected: bool,
    pub completed: bool,
}
impl Default for GameState {
    fn default() -> Self {
        use ItemId::*;
        Self {
            room: RoomId::Chamber,
            inventory: vec![
                FoldedNote,
                PocketWatch,
                OldCoin,
                RedRibbon,
                GlassVial,
                Matchbox,
                SmallCompass,
                WaxSeal,
                CopperWire,
            ],
            gate_open: false,
            unlocked: false,
            compartment_open: false,
            key_collected: false,
            completed: false,
        }
    }
}
