//! Jenn's Aseprite export and a small, movement-independent animation player.
use macroquad::prelude::*;
use serde::Deserialize;

const SHEET_JSON: &str = include_str!("../assets/jenn/jenn-side-256-sprite-sheet.json");
const SHEET_PNG: &[u8] = include_bytes!("../assets/jenn/jenn-side-256-sprite-sheet.png");
const FRONT_JSON: &str = include_str!("../assets/jenn/jenn-front-256-sprite-sheet.json");
const FRONT_PNG: &[u8] = include_bytes!("../assets/jenn/jenn-front-256-sprite-sheet-sheet.png");
const SCALE: f32 = 2.0;
const FRONT_WALK_SPEED: f64 = 2.0;
// Shared canvas origin, not the changing bounds of each pose.
const FEET: Vec2 = vec2(128.0, 250.0);

#[derive(Deserialize)]
struct FrameRect {
    x: u32,
    y: u32,
    w: u32,
    h: u32,
}

#[derive(Deserialize)]
struct Frame {
    frame: FrameRect,
    duration: u32,
    trimmed: bool,
    rotated: bool,
}

#[derive(Deserialize)]
struct Tag {
    name: String,
    from: usize,
    to: usize,
    direction: String,
}

#[derive(Deserialize)]
struct Meta {
    size: SheetSize,
    #[serde(rename = "frameTags")]
    tags: Vec<Tag>,
}

#[derive(Deserialize)]
struct SheetSize {
    w: u32,
    h: u32,
}

#[derive(Deserialize)]
struct Sheet {
    frames: Vec<Frame>,
    meta: Meta,
}

impl Sheet {
    fn parse(json: &str, tags: [&str; 2]) -> Result<Self, String> {
        let sheet: Self = serde_json::from_str(json).map_err(|e| e.to_string())?;
        if sheet.frames.is_empty() {
            return Err("Jenn's sheet has no frames".into());
        }
        for frame in &sheet.frames {
            let r = &frame.frame;
            if frame.trimmed
                || frame.rotated
                || r.w != 256
                || r.h != 256
                || frame.duration == 0
                || u64::from(r.x) + u64::from(r.w) > u64::from(sheet.meta.size.w)
                || u64::from(r.y) + u64::from(r.h) > u64::from(sheet.meta.size.h)
            {
                return Err("Jenn needs untrimmed, unrotated 256px frames with positive durations inside the sheet".into());
            }
        }
        for name in tags {
            let tag = sheet
                .meta
                .tags
                .iter()
                .find(|tag| tag.name == name)
                .ok_or_else(|| format!("Missing animation tag: {name}"))?;
            if tag.from > tag.to || tag.to >= sheet.frames.len() || tag.direction != "forward" {
                return Err(format!("Invalid forward animation tag: {name}"));
            }
        }
        Ok(sheet)
    }

    fn tag(&self, walking: bool, front: bool) -> &Tag {
        let name = match (front, walking) {
            (true, true) => "walk_front",
            (true, false) => "idle_front",
            (false, true) => "walk_right",
            (false, false) => "idle_right",
        };
        self.meta.tags.iter().find(|tag| tag.name == name).unwrap()
    }
}

#[derive(Default)]
struct Playback {
    walking: bool,
    facing_left: bool,
    front: bool,
    elapsed_ms: f64,
}

impl Playback {
    fn update(&mut self, dt: f32, walking: bool, dx: f32, dy: f32) {
        let previous_front = self.front;
        // Downward motion is toward the camera. Upward motion uses the side
        // artwork until a back-facing sheet exists. Ignore tiny resting deltas.
        if dx.abs().max(dy.abs()) > 0.001 {
            self.front = dy > dx.abs();
            if dx.abs() > 0.001 {
                self.facing_left = dx < 0.0;
            }
        }
        if self.walking != walking || previous_front != self.front {
            self.walking = walking;
            self.elapsed_ms = 0.0;
        } else {
            self.elapsed_ms += f64::from(dt.max(0.0)) * 1000.0;
        }
    }

    fn frame(&self, sheet: &Sheet) -> usize {
        let tag = sheet.tag(self.walking, self.front);
        let cycle: f64 = sheet.frames[tag.from..=tag.to]
            .iter()
            .map(|frame| f64::from(frame.duration))
            .sum();
        let speed = if self.front && self.walking {
            FRONT_WALK_SPEED
        } else {
            1.0
        };
        let mut time = (self.elapsed_ms * speed) % cycle;
        for index in tag.from..=tag.to {
            let duration = f64::from(sheet.frames[index].duration);
            if time < duration {
                return index;
            }
            time -= duration;
        }
        tag.from
    }
}

struct Artwork {
    texture: Texture2D,
    sheet: Sheet,
}

impl Artwork {
    fn new(json: &str, png: &[u8], tags: [&str; 2]) -> Self {
        let sheet = Sheet::parse(json, tags).expect("Invalid Jenn sprite metadata");
        let texture = Texture2D::from_file_with_format(png, Some(ImageFormat::Png));
        assert_eq!(texture.width() as u32, sheet.meta.size.w);
        assert_eq!(texture.height() as u32, sheet.meta.size.h);
        texture.set_filter(FilterMode::Nearest);
        Self { texture, sheet }
    }
}

pub struct Jenn {
    side: Artwork,
    front: Artwork,
    playback: Playback,
}

impl Jenn {
    pub fn new() -> Self {
        Self {
            side: Artwork::new(SHEET_JSON, SHEET_PNG, ["idle_right", "walk_right"]),
            front: Artwork::new(FRONT_JSON, FRONT_PNG, ["idle_front", "walk_front"]),
            playback: Playback::default(),
        }
    }

    pub fn update(&mut self, dt: f32, walking: bool, dx: f32, dy: f32) {
        self.playback.update(dt, walking, dx, dy);
    }

    pub fn draw(&self, position: Vec2) {
        let position = vec2(position.x.round(), position.y.round());
        draw_ellipse(
            position.x,
            position.y + 2.0,
            25.0,
            8.0,
            0.0,
            Color::new(0.02, 0.04, 0.06, 0.5),
        );
        let artwork = if self.playback.front {
            &self.front
        } else {
            &self.side
        };
        let r = &artwork.sheet.frames[self.playback.frame(&artwork.sheet)].frame;
        draw_texture_ex(
            &artwork.texture,
            position.x - FEET.x * SCALE,
            position.y - FEET.y * SCALE,
            WHITE,
            DrawTextureParams {
                source: Some(Rect::new(r.x as f32, r.y as f32, r.w as f32, r.h as f32)),
                dest_size: Some(vec2(r.w as f32, r.h as f32) * SCALE),
                flip_x: !self.playback.front && self.playback.facing_left,
                ..Default::default()
            },
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exported_tags_and_durations_drive_playback() {
        let mut sheet = Sheet::parse(SHEET_JSON, ["idle_right", "walk_right"]).unwrap();
        let mut player = Playback::default();
        assert_eq!(player.frame(&sheet), 0);
        player.update(0.0, true, 1.0, 0.0);
        assert_eq!(player.frame(&sheet), 1);
        sheet.frames[1].duration = 200;
        player.update(0.15, true, 0.0, 0.0);
        assert_eq!(player.frame(&sheet), 1);
        player.update(0.10, true, 0.0, 0.0);
        assert_eq!(player.frame(&sheet), 2);
        player.elapsed_ms = 700.0;
        assert_eq!(player.frame(&sheet), 1);
        player.update(0.0, false, 0.0, 0.0);
        assert_eq!(player.frame(&sheet), 0);
        assert_eq!(player.elapsed_ms, 0.0);
    }

    #[test]
    fn facing_follows_motion_and_survives_stopping_or_vertical_motion() {
        let mut player = Playback::default();
        player.update(0.0, true, -2.0, 0.0);
        assert!(player.facing_left);
        player.update(0.1, true, 0.0, 0.0);
        player.update(0.1, false, 0.0, 0.0);
        assert!(player.facing_left);
        player.update(0.0, true, 2.0, 0.0);
        assert!(!player.facing_left);
    }

    #[test]
    fn front_walk_uses_its_tags_and_stops_facing_camera() {
        let side = Sheet::parse(SHEET_JSON, ["idle_right", "walk_right"]).unwrap();
        let front = Sheet::parse(FRONT_JSON, ["idle_front", "walk_front"]).unwrap();
        let mut player = Playback::default();
        player.update(0.0, true, -3.0, 1.0);
        assert!(!player.front);
        assert!(player.facing_left);
        player.update(0.35, true, -3.0, 1.0);
        player.update(0.1, true, 1.0, 3.0);
        assert!(player.front);
        assert_eq!(player.elapsed_ms, 0.0);
        assert_eq!(player.frame(&front), 1);
        player.update(0.25, true, 0.0, 3.0);
        assert_eq!(player.frame(&front), 6);
        player.elapsed_ms = 1500.0;
        assert_eq!(player.frame(&front), 1);
        player.update(0.0, false, 0.0, 1.0);
        assert!(player.front);
        assert_eq!(player.frame(&front), 0);
        player.update(0.1, false, 0.0, 0.0);
        assert!(player.front);
        player.update(0.1, true, 0.0, -3.0);
        assert!(!player.front);
        assert_eq!(player.frame(&side), 1);
    }

    #[test]
    fn rejects_unsupported_exports_before_drawing() {
        for json in [
            SHEET_JSON.replace("idle_right", "unused"),
            SHEET_JSON.replace("\"duration\": 100", "\"duration\": 0"),
            SHEET_JSON.replace("\"trimmed\": false", "\"trimmed\": true"),
            SHEET_JSON.replace("\"to\": 6", "\"to\": 99"),
        ] {
            assert!(Sheet::parse(&json, ["idle_right", "walk_right"]).is_err());
        }
    }
}
