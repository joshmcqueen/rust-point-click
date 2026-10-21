//! Room content: artwork, walkable space, and intentional interaction positions.
mod chamber;
mod garden;

use crate::model::{GameState, ObjectId, RoomId};
use crate::navigation::Bounds;
use crate::world::Point;
use macroquad::prelude::*;

pub fn floor(_room: RoomId) -> Bounds {
    Bounds {
        left: 130.0,
        right: 1790.0,
        top: 650.0,
        bottom: 940.0,
    }
}

pub fn obstacle(room: RoomId) -> Bounds {
    match room {
        RoomId::Chamber => Bounds {
            left: 615.0,
            right: 885.0,
            top: 735.0,
            bottom: 875.0,
        },
        RoomId::Garden => Bounds {
            // An empty footprint leaves the painted fountain walkable for now.
            left: 0.0,
            right: 0.0,
            top: 0.0,
            bottom: 0.0,
        },
    }
}

pub fn obstacle_front(_room: RoomId) -> f32 {
    855.0
}

pub fn hotspot(room: RoomId, p: Point, state: &GameState) -> Option<ObjectId> {
    match room {
        RoomId::Chamber => chamber::hotspot(p),
        RoomId::Garden => garden::hotspot(p, state),
    }
}

pub fn approach(room: RoomId, object: ObjectId) -> Point {
    match room {
        RoomId::Chamber => chamber::approach(object),
        RoomId::Garden => garden::approach(object),
    }
}

/// Passage hitboxes are in room coordinates, above the interface after mapping.
pub fn exit(room: RoomId, p: Point) -> Option<RoomId> {
    let (area, destination) = match room {
        RoomId::Chamber => (
            Bounds {
                left: 0.0,
                right: 185.0,
                top: 650.0,
                bottom: 940.0,
            },
            RoomId::Garden,
        ),
        RoomId::Garden => (
            Bounds {
                left: 1735.0,
                right: 1920.0,
                top: 650.0,
                bottom: 940.0,
            },
            RoomId::Chamber,
        ),
    };
    area.contains(p).then_some(destination)
}

/// Position and enabled state of the room's fixed point light.
pub fn fixed_light(room: RoomId) -> ((f32, f32), bool) {
    ((440.0, 418.0), room == RoomId::Chamber)
}

pub fn exit_point(room: RoomId) -> Point {
    match room {
        RoomId::Chamber => Point::new(130.0, 820.0),
        RoomId::Garden => Point::new(1790.0, 820.0),
    }
}

pub fn entry_point(room: RoomId) -> Point {
    match room {
        RoomId::Chamber => Point::new(260.0, 820.0),
        RoomId::Garden => Point::new(1660.0, 820.0),
    }
}

pub fn ambient(room: RoomId) -> (f32, f32, f32) {
    match room {
        RoomId::Chamber => (0.18, 0.22, 0.31),
        RoomId::Garden => (0.94, 0.90, 0.76),
    }
}

/// Embedded artwork keeps native builds independent of their launch directory.
pub struct Backgrounds {
    castle: Texture2D,
    garden: Texture2D,
}

impl Backgrounds {
    pub fn new() -> Self {
        let load = |bytes: &[u8]| {
            let texture = Texture2D::from_file_with_format(bytes, Some(ImageFormat::Png));
            texture.set_filter(FilterMode::Nearest);
            texture
        };
        Self {
            castle: load(include_bytes!("../assets/backgrounds/castle.png")),
            garden: load(include_bytes!("../assets/backgrounds/garden.png")),
        }
    }

    pub fn draw(&self, room: RoomId, state: &GameState) {
        let texture = match room {
            RoomId::Chamber => &self.castle,
            RoomId::Garden => &self.garden,
        };
        draw_texture_ex(
            texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(1920.0, 1080.0)),
                ..Default::default()
            },
        );
        match room {
            RoomId::Chamber => chamber::draw_background(state),
            RoomId::Garden => garden::draw_background(state, texture),
        }
    }
}

pub fn draw_obstacle(room: RoomId) {
    if room == RoomId::Chamber {
        chamber::draw_chest();
    }
}

pub fn draw_foreground(room: RoomId, time: f32) {
    if room == RoomId::Chamber {
        chamber::draw_torch(time);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn garden_walks_directly_across_the_painted_fountain() {
        let start = Point::new(600.0, 800.0);
        let destination = Point::new(1300.0, 800.0);
        assert_eq!(
            crate::navigation::route(start, destination, obstacle(RoomId::Garden)),
            vec![destination]
        );
        let fountain = Point::new(960.0, 800.0);
        assert!(!crate::navigation::blocked(
            fountain,
            obstacle(RoomId::Garden)
        ));
        assert_eq!(
            crate::navigation::safe_point(fountain, obstacle(RoomId::Garden)),
            fountain
        );
    }

    #[test]
    fn all_room_approaches_and_entries_are_reachable() {
        for (room, objects) in [
            (
                RoomId::Chamber,
                vec![
                    ObjectId::Chest,
                    ObjectId::Gate,
                    ObjectId::Torch,
                    ObjectId::Inscription,
                ],
            ),
            (
                RoomId::Garden,
                vec![
                    ObjectId::Fountain,
                    ObjectId::LooseStone,
                    ObjectId::Compartment,
                    ObjectId::Key,
                ],
            ),
        ] {
            let mut points = vec![entry_point(room), exit_point(room)];
            points.extend(objects.into_iter().map(|object| approach(room, object)));
            for point in points {
                assert!(floor(room).contains(point));
                assert!(!crate::navigation::blocked(point, obstacle(room)));
            }
        }
    }

    #[test]
    fn exit_mapping_survives_resizing_and_rejects_bars_and_interface() {
        for (width, height) in [(1280.0, 720.0), (2560.0, 1080.0), (1000.0, 1000.0)] {
            let viewport = crate::world::viewport(width, height);
            for room in [RoomId::Chamber, RoomId::Garden] {
                let scene = exit_point(room);
                let screen = Point::new(
                    viewport.x + scene.x * viewport.scale,
                    viewport.y + scene.y * 0.75 * viewport.scale,
                );
                assert!(
                    exit(room, crate::world::room_pointer(viewport, screen).unwrap()).is_some()
                );
                // The actual pointer conversion rejects the verb panel before exit hit testing.
                let interface = Point::new(
                    viewport.x + scene.x * viewport.scale,
                    viewport.y + 1050.0 * viewport.scale,
                );
                assert_eq!(
                    crate::world::room_pointer(viewport, interface).and_then(|p| exit(room, p)),
                    None
                );
            }
            if viewport.x > 0.0 {
                assert_eq!(
                    viewport.screen_to_scene(Point::new(viewport.x - 1.0, height * 0.5)),
                    None
                );
            }
            if viewport.y > 0.0 {
                assert_eq!(
                    viewport.screen_to_scene(Point::new(width * 0.5, viewport.y - 1.0)),
                    None
                );
            }
        }
    }

    #[test]
    fn passages_have_explicit_boundaries_and_safe_separate_entries() {
        for room in [RoomId::Chamber, RoomId::Garden] {
            assert!(exit(room, exit_point(room)).is_some());
            assert!(exit(room, entry_point(room)).is_none());
            for y in [649.0, 941.0, 1080.0] {
                assert_eq!(exit(room, Point::new(exit_point(room).x, y)), None);
            }
            let p = entry_point(room);
            let b = obstacle(room);
            assert!(!(p.x > b.left && p.x < b.right && p.y > b.top && p.y < b.bottom));
        }
        assert_eq!(
            exit(RoomId::Chamber, Point::new(185.0, 650.0)),
            Some(RoomId::Garden)
        );
        assert_eq!(exit(RoomId::Chamber, Point::new(186.0, 650.0)), None);
        assert_eq!(
            exit(RoomId::Garden, Point::new(1735.0, 940.0)),
            Some(RoomId::Chamber)
        );
        assert_eq!(exit(RoomId::Garden, Point::new(1734.0, 940.0)), None);
    }
}
