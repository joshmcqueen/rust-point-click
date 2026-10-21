mod interaction;
mod interface;
mod model;
mod navigation;
mod rooms;
mod scene;
mod world;

use interaction::{Action, Outcome};
use macroquad::prelude::*;
use model::{GameState, RoomId};
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
uniform vec3 Ambient;
void main() {
    vec2 p = vec2(uv.x, 1.0 - uv.y) * vec2(1920.0, 1080.0);
    // Normalize BEFORE length: squaring pixel distances can overflow reduced
    // precision on some drivers, cutting both lights off at a hard circle.
    float a = max(0.0, 1.0 - length((p - Torch) / 760.0));
    float b = max(0.0, 1.0 - length((p - Lantern) / 540.0));
    vec3 light = Ambient
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
                UniformDesc::new("Ambient", UniformType::Float3),
            ],
            ..Default::default()
        },
    )
    .expect("Could not compile lighting shader");
    let mut game = GameState::default();
    let mut actor = Character::new(
        Point::new(960.0, 810.0),
        rooms::floor(game.room),
        rooms::obstacle(game.room),
    );
    let mut jenn = scene::Jenn::new();
    let backgrounds = rooms::Backgrounds::new();
    let mut completion_visible = false;
    let mut lantern = vec2(1260.0, 720.0);
    let mut inspect = false;
    let mut ui = interface::Interface::new();
    let mut feedback = String::new();
    let mut feedback_until = 0.0;
    let mut interaction = interaction::Interaction::default();
    loop {
        if is_key_pressed(KeyCode::Escape) {
            if completion_visible {
                completion_visible = false;
            } else {
                break;
            }
        }
        if is_key_pressed(KeyCode::L) {
            inspect = !inspect;
        }
        if is_key_pressed(KeyCode::R) {
            lantern = vec2(1260.0, 720.0);
        }
        let view = world::viewport(screen_width(), screen_height());
        if view.scale <= 0.0 {
            show_mouse(true);
            next_frame().await;
            continue;
        }
        let (mx, my) = mouse_position();
        let pointer = view.screen_to_scene(Point::new(mx, my));
        // Reserve the bottom quarter for the interface. Map room input through
        // the same vertical scale used when compositing the room texture.
        let room_pointer = world::room_pointer(view, Point::new(mx, my));
        let hovered_exit = room_pointer.and_then(|p| rooms::exit(game.room, p));
        let modal_was_visible = completion_visible;
        if completion_visible
            && (is_key_pressed(KeyCode::Enter) || is_mouse_button_pressed(MouseButton::Left))
        {
            completion_visible = false;
        }
        if !modal_was_visible {
            if let Some(p) = pointer {
                if p.y >= 810.0 {
                    ui.scroll(mouse_wheel().1, &game.inventory);
                    if is_mouse_button_pressed(MouseButton::Left) {
                        ui.click(p, &game.inventory);
                    }
                } else if is_mouse_button_pressed(MouseButton::Left) {
                    let rp = room_pointer.unwrap();
                    if let Some(destination) = hovered_exit {
                        interaction.start(&mut actor, game.room, Action::Exit(destination));
                        feedback_until = 0.0;
                    } else if let Some(object) = rooms::hotspot(game.room, rp, &game) {
                        interaction.start(
                            &mut actor,
                            game.room,
                            Action::Object {
                                verb: ui.verb,
                                item: ui.selected,
                                object,
                            },
                        );
                        feedback_until = 0.0;
                    } else {
                        interaction.walk(&mut actor, rp);
                    }
                }
            }
        }
        if let Some(p) = room_pointer {
            if is_mouse_button_down(MouseButton::Right) {
                lantern = vec2(p.x, p.y);
            }
        }
        let dt = get_frame_time();
        let previous = actor.position;
        actor.update(dt);
        // Measure actual motion before room transitions can reposition the actor.
        jenn.update(
            dt,
            actor.moving(),
            actor.position.x - previous.x,
            actor.position.y - previous.y,
        );
        if let Some(outcome) = interaction.finish(&mut actor, &mut game) {
            match outcome {
                Outcome::Feedback(response) => {
                    feedback = response;
                    feedback_until = get_time() + 5.0;
                }
                Outcome::Transition => {
                    feedback_until = 0.0;
                    lantern = vec2(1260.0, 720.0);
                }
                Outcome::Completed => {
                    completion_visible = true;
                    feedback_until = 0.0;
                }
            }
        }
        let time = get_time() as f32;
        set_camera(&Camera2D {
            render_target: Some(target.clone()),
            ..Camera2D::from_display_rect(Rect::new(0.0, 0.0, 1920.0, 1080.0))
        });
        clear_background(BLACK);
        backgrounds.draw(game.room, &game);
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
        if actor.position.y < rooms::obstacle_front(game.room) {
            jenn.draw(vec2(actor.position.x, actor.position.y));
            rooms::draw_obstacle(game.room);
        } else {
            rooms::draw_obstacle(game.room);
            jenn.draw(vec2(actor.position.x, actor.position.y));
        }
        rooms::draw_foreground(game.room, time);
        draw_circle(
            lantern.x,
            lantern.y,
            7.0,
            Color::from_rgba(207, 231, 244, 255),
        );
        set_default_camera();
        clear_background(BLACK);
        material.set_uniform("Ambient", rooms::ambient(game.room));
        let (fixed_light, fixed_enabled) = rooms::fixed_light(game.room);
        material.set_uniform("Torch", fixed_light);
        material.set_uniform("Lantern", (lantern.x, lantern.y));
        material.set_uniform(
            "Flicker",
            if fixed_enabled {
                0.96 + (time * 8.0).sin() * 0.025 + (time * 13.3).sin() * 0.015
            } else {
                0.0
            },
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
        ui.draw(pointer, &game.inventory);
        if let Some(action) = &interaction.pending {
            let sentence = format!("Walking to: {}", action.sentence());
            let width = measure_text(&sentence, None, 26, 1.0).width;
            draw_rectangle(
                960.0 - width / 2.0 - 14.0,
                752.0,
                width + 28.0,
                46.0,
                Color::new(0.02, 0.03, 0.05, 0.92),
            );
            draw_text(&sentence, 960.0 - width / 2.0, 784.0, 26.0, WHITE);
        }
        draw_text(
            game.room.name(),
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
        // Re-evaluate after updates: the room may have changed this frame.
        let hovered_exit = room_pointer.and_then(|p| rooms::exit(game.room, p));
        let arrow = hovered_exit.is_some() && !completion_visible;
        show_mouse(!arrow);
        if !completion_visible {
            if let Some(p) = room_pointer {
                let phrase = if let Some(destination) = hovered_exit {
                    Some(format!("Go to {}", destination.name()))
                } else {
                    rooms::hotspot(game.room, p, &game).map(|object| ui.phrase(Some(object.name())))
                };
                if let Some(phrase) = phrase {
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
                }
                if arrow {
                    let direction = if game.room == RoomId::Chamber {
                        -1.0
                    } else {
                        1.0
                    };
                    let tip = vec2(p.x, p.y * 0.75);
                    let tail = tip - vec2(direction * 32.0, 0.0);
                    draw_line(tail.x, tail.y, tip.x, tip.y, 10.0, BLACK);
                    draw_triangle(
                        tip,
                        tip - vec2(direction * 19.0, 17.0),
                        tip - vec2(direction * 19.0, -17.0),
                        BLACK,
                    );
                    draw_line(tail.x, tail.y, tip.x - direction * 5.0, tip.y, 5.0, GOLD);
                    draw_triangle(
                        tip,
                        tip - vec2(direction * 16.0, 12.0),
                        tip - vec2(direction * 16.0, -12.0),
                        GOLD,
                    );
                }
            }
        }
        if completion_visible {
            draw_rectangle(
                440.0,
                300.0,
                1040.0,
                200.0,
                Color::new(0.02, 0.06, 0.08, 0.97),
            );
            draw_rectangle_lines(440.0, 300.0, 1040.0, 200.0, 3.0, GOLD);
            let message = "Demo complete — you opened the Ember Archive!";
            let width = measure_text(message, None, 32, 1.0).width;
            draw_text(message, 960.0 - width / 2.0, 375.0, 32.0, GOLD);
            draw_text(
                "Click or press Enter to keep exploring.",
                650.0,
                440.0,
                28.0,
                WHITE,
            );
        }
        set_default_camera();
        next_frame().await;
    }
    show_mouse(true);
}
