//! One rectangular obstacle and a tiny visibility graph, all in room coordinates.
use crate::world::Point;

// Chest footprint: x=650..850, y=755..855. Extra room for the actor's feet/body.
const LEFT: f32 = 615.0;
const RIGHT: f32 = 885.0;
const TOP: f32 = 735.0;
pub const CHEST_FRONT: f32 = 855.0;
const BOTTOM: f32 = 875.0;

pub fn blocked(p: Point) -> bool {
    p.x > LEFT && p.x < RIGHT && p.y > TOP && p.y < BOTTOM
}

/// A click inside the footprint stops at its nearest safe edge.
pub fn safe_point(p: Point) -> Point {
    if !blocked(p) {
        return p;
    }
    let candidates = [
        Point::new(LEFT, p.y),
        Point::new(RIGHT, p.y),
        Point::new(p.x, TOP),
        Point::new(p.x, BOTTOM),
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
fn crosses(a: Point, b: Point) -> bool {
    let mut enter: f32 = 0.0;
    let mut exit: f32 = 1.0;
    for (start, end, low, high) in [(a.x, b.x, LEFT, RIGHT), (a.y, b.y, TOP, BOTTOM)] {
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
pub fn route(start: Point, goal: Point) -> Vec<Point> {
    let goal = safe_point(goal);
    let nodes = [
        start,
        goal,
        Point::new(LEFT, TOP),
        Point::new(RIGHT, TOP),
        Point::new(RIGHT, BOTTOM),
        Point::new(LEFT, BOTTOM),
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
            if !visited[next] && !crosses(nodes[current], nodes[next]) {
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

    #[test]
    fn routes_both_directions_around_chest_without_cutting_corners() {
        for (start, end) in [
            (Point::new(400.0, 810.0), Point::new(1100.0, 810.0)),
            (Point::new(1100.0, 810.0), Point::new(400.0, 810.0)),
            (Point::new(750.0, 700.0), Point::new(750.0, 920.0)),
        ] {
            let path = route(start, end);
            assert!(path.len() > 1);
            assert_eq!(path.last(), Some(&end));
            let mut last = start;
            for point in path {
                assert!(!crosses(last, point));
                last = point;
            }
        }
    }

    #[test]
    fn clear_routes_are_direct_and_inside_clicks_stop_outside() {
        let start = Point::new(400.0, 680.0);
        let end = Point::new(1100.0, 680.0);
        assert_eq!(route(start, end), vec![end]);
        let path = route(start, Point::new(750.0, 810.0));
        assert!(!blocked(*path.last().unwrap()));
        assert_ne!(path.last(), Some(&Point::new(750.0, 810.0)));
    }
}
