//! One rectangular obstacle and a tiny visibility graph, all in room coordinates.
use crate::world::Point;

/// Room geometry expressed in the same coordinates as the character's feet.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Bounds {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}

impl Bounds {
    pub fn contains(self, point: Point) -> bool {
        point.x >= self.left
            && point.x <= self.right
            && point.y >= self.top
            && point.y <= self.bottom
    }
}

pub fn blocked(p: Point, obstacle: Bounds) -> bool {
    p.x > obstacle.left && p.x < obstacle.right && p.y > obstacle.top && p.y < obstacle.bottom
}

/// A click inside the footprint stops at its nearest safe edge.
pub fn safe_point(p: Point, obstacle: Bounds) -> Point {
    if !blocked(p, obstacle) {
        return p;
    }
    let candidates = [
        Point::new(obstacle.left, p.y),
        Point::new(obstacle.right, p.y),
        Point::new(p.x, obstacle.top),
        Point::new(p.x, obstacle.bottom),
    ];
    *candidates
        .iter()
        .min_by(|a, b| distance(p, **a).total_cmp(&distance(p, **b)))
        .unwrap()
}

fn distance(a: Point, b: Point) -> f32 {
    (a.x - b.x).hypot(a.y - b.y)
}

/// Intersect a segment with the rectangle's open interior. Edge travel is safe.
fn crosses(a: Point, b: Point, obstacle: Bounds) -> bool {
    let mut enter: f32 = 0.0;
    let mut exit: f32 = 1.0;
    for (start, end, low, high) in [
        (a.x, b.x, obstacle.left, obstacle.right),
        (a.y, b.y, obstacle.top, obstacle.bottom),
    ] {
        let delta = end - start;
        if delta.abs() < f32::EPSILON {
            if start <= low || start >= high {
                return false;
            }
        } else {
            let t1 = (low - start) / delta;
            let t2 = (high - start) / delta;
            enter = enter.max(t1.min(t2));
            exit = exit.min(t1.max(t2));
        }
    }
    enter < exit && exit > 0.0 && enter < 1.0
}

/// Shortest visible route through at most four corners. No grid or physics engine.
pub fn route(start: Point, goal: Point, obstacle: Bounds) -> Vec<Point> {
    let goal = safe_point(goal, obstacle);
    let nodes = [
        start,
        goal,
        Point::new(obstacle.left, obstacle.top),
        Point::new(obstacle.right, obstacle.top),
        Point::new(obstacle.right, obstacle.bottom),
        Point::new(obstacle.left, obstacle.bottom),
    ];
    let mut costs = [f32::INFINITY; 6];
    let mut previous = [0; 6];
    let mut visited = [false; 6];
    costs[0] = 0.0;
    for _ in 0..6 {
        let current = (0..6)
            .filter(|&i| !visited[i])
            .min_by(|&a, &b| costs[a].total_cmp(&costs[b]))
            .unwrap();
        if current == 1 {
            break;
        }
        visited[current] = true;
        for next in 0..6 {
            if !visited[next] && !crosses(nodes[current], nodes[next], obstacle) {
                let cost = costs[current] + distance(nodes[current], nodes[next]);
                if cost < costs[next] {
                    costs[next] = cost;
                    previous[next] = current;
                }
            }
        }
    }
    // Both endpoints are outside this single freestanding rectangle, so a route exists.
    let mut result = vec![goal];
    let mut current = 1;
    while previous[current] != 0 {
        current = previous[current];
        result.push(nodes[current]);
    }
    result.reverse();
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    const CHEST: Bounds = Bounds {
        left: 615.0,
        right: 885.0,
        top: 735.0,
        bottom: 875.0,
    };

    #[test]
    fn routes_both_directions_around_chest_without_cutting_corners() {
        for (start, end) in [
            (Point::new(400.0, 810.0), Point::new(1100.0, 810.0)),
            (Point::new(1100.0, 810.0), Point::new(400.0, 810.0)),
            (Point::new(750.0, 700.0), Point::new(750.0, 920.0)),
        ] {
            let path = route(start, end, CHEST);
            assert!(path.len() > 1);
            assert_eq!(path.last(), Some(&end));
            let mut last = start;
            for point in path {
                assert!(!crosses(last, point, CHEST));
                last = point;
            }
        }
    }

    #[test]
    fn clear_routes_are_direct_and_inside_clicks_stop_outside() {
        let start = Point::new(400.0, 680.0);
        let end = Point::new(1100.0, 680.0);
        assert_eq!(route(start, end, CHEST), vec![end]);
        let path = route(start, Point::new(750.0, 810.0), CHEST);
        assert!(!blocked(*path.last().unwrap(), CHEST));
        assert_ne!(path.last(), Some(&Point::new(750.0, 810.0)));
    }
    #[test]
    fn fountain_routes_use_room_geometry() {
        let fountain = Bounds {
            left: 810.0,
            right: 1110.0,
            top: 700.0,
            bottom: 900.0,
        };
        let start = Point::new(600.0, 800.0);
        let end = Point::new(1300.0, 800.0);
        let mut last = start;
        let path = route(start, end, fountain);
        assert!(path.len() > 1);
        for point in path {
            assert!(!crosses(last, point, fountain));
            last = point;
        }
        assert_eq!(last, end);
        assert!(blocked(Point::new(960.0, 800.0), fountain));
        assert!(!blocked(Point::new(960.0, 800.0), CHEST));
        assert!(!blocked(
            safe_point(Point::new(960.0, 800.0), fountain),
            fountain
        ));
    }
}
