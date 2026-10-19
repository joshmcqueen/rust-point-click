mod interaction;
mod interface;
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
varying highp vec2 uv;
void main() { gl_Position = Projection * Model * vec4(position, 1.0); uv = texcoord; }
"#;
const FRAGMENT: &str = r#"#version 100
precision highp float;
varying highp vec2 uv;
uniform sampler2D Texture;
uniform vec2 Torch;
uniform vec2 Lantern;
uniform float Flicker;
uniform float Inspect;
void main() {
    vec2 p = vec2(uv.x, 1.0 - uv.y) * vec2(1920.0, 1080.0);
    // Normalize BEFORE length: squaring pixel distances can overflow reduced
    // precision on some drivers, cutting both lights off at a hard circle.
    float a = max(0.0, 1.0 - length((p - Torch) / 760.0));
    float b = max(0.0, 1.0 - length((p - Lantern) / 540.0));
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
    let mut ui = interface::Interface::new();
    let mut room = interaction::Room::default();
    let mut feedback = String::new();
    let mut feedback_until = 0.0;
    let mut pending: Option<(String, Option<String>, String)> = None;
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
        // Reserve the bottom quarter for the interface. Map room input through
        // the same vertical scale used when compositing the room texture.
        let room_pointer = pointer
            .filter(|p| p.y < 810.0)
            .map(|p| Point::new(p.x, p.y / 0.75));
        if let Some(p) = pointer {
            if p.y >= 810.0 {
                ui.scroll(mouse_wheel().1);
                if is_mouse_button_pressed(MouseButton::Left) {
                    ui.click(p);
                }
            } else if is_mouse_button_pressed(MouseButton::Left) {
                let rp = room_pointer.unwrap();
                if let Some(name) = interaction::hotspot(rp) {
                    let item = ui.selected.map(|i| ui.items[i].to_string());
                    pending = Some((ui.verb.to_string(), item, name.to_string()));
                    actor.walk_to(Point::new(rp.x, 700.0));
                } else {
                    pending = None;
                    actor.walk_to(rp);
                }
            }
        }
        if let Some(p) = room_pointer {
            if is_mouse_button_down(MouseButton::Right) {
                lantern = vec2(p.x, p.y);
            }
        }
        actor.update(get_frame_time());
        if !actor.moving() {
            if let Some((verb, item, object)) = pending.take() {
                feedback = room.act(&verb, item.as_deref(), &object);
                feedback_until = get_time() + 5.0;
            }
        }
        let time = get_time() as f32;
        set_camera(&Camera2D {
            render_target: Some(target.clone()),
            ..Camera2D::from_display_rect(Rect::new(0.0, 0.0, 1920.0, 1080.0))
        });
        clear_background(BLACK);
        scene::draw_room(time);
        if room.gate_open {
            draw_rectangle(
                814.0,
                290.0,
                292.0,
                346.0,
                Color::from_rgba(12, 20, 28, 255),
            );
            draw_rectangle(
                820.0,
                300.0,
                25.0,
                330.0,
                Color::from_rgba(90, 105, 105, 255),
            );
        }
        draw_rectangle(
            1250.0,
            365.0,
            200.0,
            125.0,
            Color::from_rgba(112, 125, 120, 255),
        );
        for row in 0..4 {
            draw_rectangle(
                1270.0,
                385.0 + row as f32 * 24.0,
                150.0 - row as f32 * 12.0,
                5.0,
                Color::from_rgba(48, 64, 68, 255),
            );
        }
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
                dest_size: Some(vec2(view.width, view.height * 0.75)),
                flip_y: true,
                ..Default::default()
            },
        );
        gl_use_default_material();
        // Overlay uses the full design canvas and is never darkened by lighting.
        let mut overlay_camera = Camera2D::from_display_rect(Rect::new(
            -view.x / view.scale,
            -view.y / view.scale,
            screen_width() / view.scale,
            screen_height() / view.scale,
        ));
        // Screen cameras invert Y internally; use positive zoom for top-down UI.
        // The room camera targets a texture and follows a different Y convention.
        overlay_camera.zoom.y = -overlay_camera.zoom.y;
        set_camera(&overlay_camera);
        ui.draw(pointer);
        draw_text(
            "THE EMBER ARCHIVE",
            28.0,
            36.0,
            28.0,
            Color::from_rgba(232, 207, 157, 255),
        );
        draw_text(
            "Right hold: light   L: lighting   R: reset   Esc: quit",
            1160.0,
            34.0,
            22.0,
            LIGHTGRAY,
        );
        if get_time() < feedback_until {
            let width = measure_text(&feedback, None, 28, 1.0).width;
            draw_rectangle(
                960.0 - width / 2.0 - 18.0,
                65.0,
                width + 36.0,
                48.0,
                Color::new(0.02, 0.03, 0.05, 0.92),
            );
            draw_text(&feedback, 960.0 - width / 2.0, 98.0, 28.0, WHITE);
        }
        if let Some(p) = room_pointer {
            if let Some(name) = interaction::hotspot(p) {
                let phrase = ui.phrase(Some(name));
                let width = measure_text(&phrase, None, 30, 1.0).width;
                let x = p.x.clamp(width / 2.0 + 12.0, 1908.0 - width / 2.0);
                let y = (p.y * 0.75 - 40.0).clamp(150.0, 775.0);
                draw_rectangle(
                    x - width / 2.0 - 10.0,
                    y - 30.0,
                    width + 20.0,
                    42.0,
                    Color::new(0.01, 0.02, 0.03, 0.85),
                );
                draw_text(&phrase, x - width / 2.0, y, 30.0, WHITE);
                draw_line(p.x - 10.0, p.y * 0.75, p.x + 10.0, p.y * 0.75, 2.0, WHITE);
                draw_line(p.x, p.y * 0.75 - 10.0, p.x, p.y * 0.75 + 10.0, 2.0, WHITE);
            }
        }
        set_default_camera();
        next_frame().await;
    }
}
