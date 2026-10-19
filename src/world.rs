//! Scene coordinates and walking are independent of drawing and window size.
use crate::navigation::{self, Bounds};
use std::collections::VecDeque;

pub const SCENE_WIDTH: f32 = 1920.0;
pub const SCENE_HEIGHT: f32 = 1080.0;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Viewport {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub scale: f32,
}

/// Fit the scene inside the window, with centered bars when its shape differs.
pub fn viewport(screen_width: f32, screen_height: f32) -> Viewport {
    let scale = (screen_width / SCENE_WIDTH)
        .min(screen_height / SCENE_HEIGHT)
        .max(0.0);
    let width = SCENE_WIDTH * scale;
    let height = SCENE_HEIGHT * scale;
    Viewport {
        x: (screen_width - width) / 2.0,
        y: (screen_height - height) / 2.0,
        width,
        height,
        scale,
    }
}

impl Viewport {
    /// Ignore clicks in the bars instead of moving to an invisible destination.
    pub fn screen_to_scene(self, screen: Point) -> Option<Point> {
        if self.scale <= 0.0
            || screen.x < self.x
            || screen.y < self.y
            || screen.x > self.x + self.width
            || screen.y > self.y + self.height
        {
            return None;
        }
        Some(Point::new(
            (screen.x - self.x) / self.scale,
            (screen.y - self.y) / self.scale,
        ))
    }
}

/// Convert the displayed room (the upper 810 scene pixels) to room coordinates.
/// UI and letterbox pixels have no room position.
pub fn room_pointer(view: Viewport, screen: Point) -> Option<Point> {
    let scene = view.screen_to_scene(screen)?;
    (scene.y < 810.0).then(|| Point::new(scene.x, scene.y / 0.75))
}

/// The character's position represents their feet on the floor.
pub fn clamp_walk_target(point: Point, floor: Bounds) -> Point {
    Point::new(
        point.x.clamp(floor.left, floor.right),
        point.y.clamp(floor.top, floor.bottom),
    )
}

pub struct Character {
    pub position: Point,
    pub target: Point,
    pub speed: f32,
    route: VecDeque<Point>,
    floor: Bounds,
    obstacle: Bounds,
}

impl Character {
    pub fn new(position: Point, floor: Bounds, obstacle: Bounds) -> Self {
        let position = navigation::safe_point(clamp_walk_target(position, floor), obstacle);
        Self {
            position,
            target: position,
            speed: 360.0,
            route: VecDeque::new(),
            floor,
            obstacle,
        }
    }

    /// Enter a room with fresh geometry and no remaining movement from the old room.
    pub fn enter(&mut self, position: Point, floor: Bounds, obstacle: Bounds) {
        self.floor = floor;
        self.obstacle = obstacle;
        self.position = navigation::safe_point(clamp_walk_target(position, floor), obstacle);
        self.target = self.position;
        self.route.clear();
    }

    pub fn walk_to(&mut self, point: Point) {
        self.target = navigation::safe_point(clamp_walk_target(point, self.floor), self.obstacle);
        self.route = navigation::route(self.position, self.target, self.obstacle).into();
    }

    pub fn moving(&self) -> bool {
        self.position != self.target
    }

    pub fn update(&mut self, dt: f32) {
        // Carry unused distance through corners so speed does not depend on frames.
        let mut remaining = self.speed.max(0.0) * dt.max(0.0);
        while let Some(&next) = self.route.front() {
            let dx = next.x - self.position.x;
            let dy = next.y - self.position.y;
            let distance = dx.hypot(dy);
            if distance <= remaining {
                self.position = next;
                self.route.pop_front();
                remaining -= distance;
            } else {
                self.position.x += dx / distance * remaining;
                self.position.y += dy / distance * remaining;
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const FLOOR: Bounds = Bounds {
        left: 130.0,
        right: 1790.0,
        top: 650.0,
        bottom: 940.0,
    };
    const CHEST: Bounds = Bounds {
        left: 615.0,
        right: 885.0,
        top: 735.0,
        bottom: 875.0,
    };

    fn near(actual: f32, expected: f32) {
        assert!((actual - expected).abs() < 0.01, "{actual} != {expected}");
    }

    #[test]
    fn ultrawide_bars_are_not_clickable_and_center_maps_correctly() {
        let view = viewport(2560.0, 1080.0);
        near(view.x, 320.0);
        assert_eq!(view.screen_to_scene(Point::new(100.0, 500.0)), None);
        assert_eq!(
            view.screen_to_scene(Point::new(1280.0, 540.0)),
            Some(Point::new(960.0, 540.0))
        );
    }

    #[test]
    fn smaller_and_tall_windows_map_to_same_design_coordinates() {
        for (width, height) in [(1280.0, 720.0), (1000.0, 1000.0)] {
            let view = viewport(width, height);
            let mapped = view
                .screen_to_scene(Point::new(
                    view.x + 600.0 * view.scale,
                    view.y + 800.0 * view.scale,
                ))
                .unwrap();
            near(mapped.x, 600.0);
            near(mapped.y, 800.0);
        }
        assert!(viewport(0.0, 0.0)
            .screen_to_scene(Point::new(0.0, 0.0))
            .is_none());
    }

    #[test]
    fn walking_is_independent_of_update_frequency() {
        let mut one_step = Character::new(Point::new(200.0, 700.0), FLOOR, CHEST);
        let mut many_steps = Character::new(one_step.position, FLOOR, CHEST);
        one_step.walk_to(Point::new(1500.0, 900.0));
        many_steps.walk_to(one_step.target);
        one_step.update(1.0);
        for _ in 0..60 {
            many_steps.update(1.0 / 60.0);
        }
        near(one_step.position.x, many_steps.position.x);
        near(one_step.position.y, many_steps.position.y);
    }

    #[test]
    fn walking_stops_exactly_without_overshooting_and_clamps_floor() {
        let mut character = Character::new(Point::new(200.0, 700.0), FLOOR, CHEST);
        character.walk_to(Point::new(-10.0, 2000.0));
        assert_eq!(character.target, Point::new(130.0, 940.0));
        character.update(10.0);
        assert_eq!(character.position, character.target);
        assert!(!character.moving());
        character.update(1.0);
        assert_eq!(character.position, Point::new(130.0, 940.0));
    }
}

#[cfg(test)]
mod obstacle_tests {
    use super::*;
    const FLOOR: Bounds = Bounds {
        left: 130.0,
        right: 1790.0,
        top: 650.0,
        bottom: 940.0,
    };
    const CHEST: Bounds = Bounds {
        left: 615.0,
        right: 885.0,
        top: 735.0,
        bottom: 875.0,
    };

    #[test]
    fn walking_and_retargeting_never_enter_chest() {
        let mut actor = Character::new(Point::new(400.0, 810.0), FLOOR, CHEST);
        actor.walk_to(Point::new(1100.0, 810.0));
        for frame in 0..500 {
            if frame == 45 {
                actor.walk_to(Point::new(750.0, 920.0));
            }
            actor.update(1.0 / 60.0);
            assert!(!navigation::blocked(actor.position, CHEST));
        }
        assert_eq!(actor.position, Point::new(750.0, 920.0));
        assert!(!actor.moving());
    }

    #[test]
    fn corner_travel_uses_whole_frame_budget() {
        let mut once = Character::new(Point::new(400.0, 810.0), FLOOR, CHEST);
        let mut many = Character::new(once.position, FLOOR, CHEST);
        once.walk_to(Point::new(1100.0, 810.0));
        many.walk_to(once.target);
        once.update(1.5);
        for _ in 0..90 {
            many.update(1.0 / 60.0);
        }
        assert!((once.position.x - many.position.x).abs() < 0.01);
        assert!((once.position.y - many.position.y).abs() < 0.01);
        once.update(10.0);
        assert_eq!(once.position, once.target);
    }
    #[test]
    fn entering_room_clears_old_route_and_uses_new_geometry() {
        let mut actor = Character::new(Point::new(400.0, 810.0), FLOOR, CHEST);
        actor.walk_to(Point::new(1100.0, 810.0));
        actor.update(0.1);
        let fountain = Bounds {
            left: 810.0,
            right: 1110.0,
            top: 700.0,
            bottom: 900.0,
        };
        actor.enter(Point::new(1700.0, 820.0), FLOOR, fountain);
        assert!(!actor.moving());
        actor.update(10.0);
        assert_eq!(actor.position, Point::new(1700.0, 820.0));
        actor.walk_to(Point::new(600.0, 820.0));
        for _ in 0..600 {
            actor.update(1.0 / 60.0);
            assert!(!navigation::blocked(actor.position, fountain));
        }
        assert_eq!(actor.position, Point::new(600.0, 820.0));
        actor.enter(Point::new(960.0, 800.0), FLOOR, fountain);
        assert!(!navigation::blocked(actor.position, fountain));
        assert_eq!(actor.position, actor.target);
    }

    #[test]
    fn room_pointer_maps_room_and_rejects_ui_and_bars() {
        for (width, height) in [(1280.0, 720.0), (2560.0, 1080.0), (1000.0, 1000.0)] {
            let view = viewport(width, height);
            let screen = |x, y| Point::new(view.x + x * view.scale, view.y + y * view.scale);
            let mapped = room_pointer(view, screen(960.0, 600.0)).unwrap();
            assert!((mapped.x - 960.0).abs() < 0.01);
            assert!((mapped.y - 800.0).abs() < 0.01);
            assert_eq!(room_pointer(view, screen(960.0, 810.0)), None);
            assert_eq!(room_pointer(view, screen(960.0, 1000.0)), None);
            assert_eq!(
                room_pointer(view, Point::new(view.x - 1.0, view.y + 10.0)),
                None
            );
            assert_eq!(
                room_pointer(view, Point::new(view.x + 10.0, view.y - 1.0)),
                None
            );
        }
    }
}
