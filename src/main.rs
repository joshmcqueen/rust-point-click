mod scene;
mod world;

use macroquad::prelude::*;
use world::{Character, Point};

fn window_conf() -> Conf {
    Conf {
        window_title: "The Ember Archive — prototype".into(),
        window_width: 1280,
        window_height: 720,
        window_resizable: true,
        high_dpi: true,
        ..Default::default()
    }
}

// The room and actor share this pass: light multiplies their actual colors.
const VERTEX: &str = r#"#version 100
attribute vec3 position;
attribute vec2 texcoord;
attribute vec4 color0;
uniform mat4 Model;
uniform mat4 Projection;
varying lowp vec2 uv;
void main() { gl_Position = Projection * Model * vec4(position, 1.0); uv = texcoord; }
"#;
const FRAGMENT: &str = r#"#version 100
precision mediump float;
varying lowp vec2 uv;
uniform sampler2D Texture;
uniform vec2 Torch;
uniform vec2 Lantern;
uniform float Flicker;
uniform float Inspect;
void main() {
    vec2 p = vec2(uv.x, 1.0 - uv.y) * vec2(1920.0, 1080.0);
    float a = max(0.0, 1.0 - distance(p, Torch) / 760.0);
    float b = max(0.0, 1.0 - distance(p, Lantern) / 540.0);
    vec3 light = vec3(0.18, 0.22, 0.31)
        + vec3(1.15, 0.72, 0.32) * a * a * Flicker
        + vec3(0.65, 0.76, 0.86) * b * b;
    gl_FragColor = vec4(texture2D(Texture, uv).rgb * mix(light, vec3(1.0), Inspect), 1.0);
}
"#;

#[macroquad::main(window_conf)]
async fn main() {
    let target = render_target(1920, 1080);
    target.texture.set_filter(FilterMode::Nearest);
    let material = load_material(
        ShaderSource::Glsl {
            vertex: VERTEX,
            fragment: FRAGMENT,
        },
        MaterialParams {
            uniforms: vec![
                UniformDesc::new("Torch", UniformType::Float2),
                UniformDesc::new("Lantern", UniformType::Float2),
                UniformDesc::new("Flicker", UniformType::Float1),
                UniformDesc::new("Inspect", UniformType::Float1),
            ],
            ..Default::default()
        },
    )
    .expect("Could not compile lighting shader");
    let mut actor = Character::new(Point::new(960.0, 810.0));
    let mut lantern = vec2(1260.0, 720.0);
    let mut inspect = false;
    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        if is_key_pressed(KeyCode::L) {
            inspect = !inspect;
        }
        if is_key_pressed(KeyCode::R) {
            lantern = vec2(1260.0, 720.0);
        }
        let view = world::viewport(screen_width(), screen_height());
        let (mx, my) = mouse_position();
        let pointer = view.screen_to_scene(Point::new(mx, my));
        if let Some(p) = pointer {
            if is_mouse_button_pressed(MouseButton::Left) {
                actor.walk_to(p);
            }
            if is_mouse_button_down(MouseButton::Right) {
                lantern = vec2(p.x, p.y);
            }
        }
        actor.update(get_frame_time());
        let time = get_time() as f32;
        set_camera(&Camera2D {
            render_target: Some(target.clone()),
            ..Camera2D::from_display_rect(Rect::new(0.0, 0.0, 1920.0, 1080.0))
        });
        clear_background(BLACK);
        scene::draw_room(time);
        if actor.moving() {
            draw_ellipse_lines(
                actor.target.x,
                actor.target.y,
                22.0,
                8.0,
                0.0,
                2.0,
                Color::new(0.6, 0.65, 0.65, 0.6),
            );
        }
        scene::draw_character(
            vec2(actor.position.x, actor.position.y),
            actor.moving(),
            time,
        );
        scene::draw_torch(time);
        draw_circle(
            lantern.x,
            lantern.y,
            7.0,
            Color::from_rgba(207, 231, 244, 255),
        );
        set_default_camera();
        clear_background(BLACK);
        material.set_uniform("Torch", (440.0f32, 430.0f32));
        material.set_uniform("Lantern", (lantern.x, lantern.y));
        material.set_uniform(
            "Flicker",
            0.96 + (time * 8.0).sin() * 0.025 + (time * 13.3).sin() * 0.015,
        );
        material.set_uniform("Inspect", if inspect { 1.0f32 } else { 0.0f32 });
        gl_use_material(&material);
        draw_texture_ex(
            &target.texture,
            view.x,
            view.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(view.width, view.height)),
                flip_y: true,
                ..Default::default()
            },
        );
        gl_use_default_material();
        // HUD stays crisp and readable at each window size, outside the lighting pass.
        let s = view.scale;
        draw_rectangle(
            view.x,
            view.y,
            view.width,
            94.0 * s,
            Color::new(0.025, 0.035, 0.055, 0.88),
        );
        draw_text(
            "THE EMBER ARCHIVE",
            view.x + 40.0 * s,
            view.y + 42.0 * s,
            32.0 * s,
            Color::from_rgba(232, 207, 157, 255),
        );
        draw_text("Left click: walk   |   Hold right mouse: move cool light   |   L: compare lighting   |   R: reset light   |   Esc: quit",
            view.x+40.0*s, view.y+76.0*s, 23.0*s, Color::from_rgba(190,202,214,255));
        next_frame().await;
    }
}
