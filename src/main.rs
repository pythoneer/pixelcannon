#![feature(step_by)]

extern crate orbclient;
extern crate orbimage;

use orbclient::EventOption;
use orbimage::Image;

use std::time::Instant;
use std::thread;

use model::mesh::Mesh;
use primitive::matrix::Matrix4f32;
use render::RenderContext;
use texture::bitmap::BitmapTexture;

pub mod interpolate;
pub mod model;
pub mod primitive;
pub mod render;
pub mod texture;

fn main() {
    let mut render_context = RenderContext::new(800, 600, "pixelcannon");
    let mut start = Instant::now();

    let projection = Matrix4f32::new().init_perspective(70.0_f32.to_radians(), render_context.get_width() as f32 / render_context.get_height() as f32, 0.1_f32, 1000_f32);

    let mut basepath = "";
    if cfg!(target_os = "redox") {
        basepath = "/apps/pixelcannon/";
    }

    let mesh = Mesh::from_path(basepath.to_string() + "assets/sphere.obj").unwrap();

    let image = Image::from_path(basepath.to_string() + "assets/img2.png").unwrap();
    let texture = BitmapTexture::from_orbimage(&image);

    let mut trans_x = 0_f32;
    let mut trans_y = 0_f32;
    let mut trans_z = 4.0_f32;

    let mut rot_x = 0_f32;
    let mut rot_y = -0.5_f32;
    let mut rot_z = 0_f32;

    let mut move_forward = false;
    let mut move_back = false;
    let mut move_left = false;
    let mut move_right = false;
    let mut move_up = false;
    let mut move_down = false;

    let mut turn_forward = false;
    let mut turn_back = false;
    let mut turn_left = false;
    let mut turn_right = false;
    let mut turn_up = false;
    let mut turn_down = false;

    let mut frame_cnt = 0_f32;
    let mut counter_duration = 0_f32;

    'event: loop {
        for orbital_event in render_context.events() {
            match orbital_event.to_option() {
                EventOption::Key(key_event) => {
                    match key_event.scancode {
                        //Translation
                        orbclient::K_W => move_forward = key_event.pressed,
                        orbclient::K_S => move_back = key_event.pressed,
                        orbclient::K_A => move_left = key_event.pressed,
                        orbclient::K_D => move_right = key_event.pressed,
                        orbclient::K_Q => move_down = key_event.pressed,
                        orbclient::K_E => move_up = key_event.pressed,

                        //Rotation
                        orbclient::K_I => turn_forward = key_event.pressed,
                        orbclient::K_K => turn_back = key_event.pressed,
                        orbclient::K_J => turn_left = key_event.pressed,
                        orbclient::K_L => turn_right = key_event.pressed,
                        orbclient::K_U => turn_down = key_event.pressed,
                        orbclient::K_O => turn_up = key_event.pressed,
                        _ => ()
                    }
                },
                EventOption::Quit(_quit_event) => break 'event,
                _ => (),
            };
        }

        let end = Instant::now();
        let delta = end.duration_since(start);
        let delta_ms = delta.as_secs() as f32 * 1000_f32 + (delta.subsec_nanos() as f32)/1000000 as f32;
        start = end;

        let speed = delta_ms / 500_f32;

        if move_forward {
            trans_z = 2_f32.max(trans_z - speed);
        }
        if move_back {
            trans_z += speed;
        }
        if move_left {
            trans_x += speed;
        }
        if move_right {
            trans_x -= speed;
        }
        if move_up {
            trans_y -= speed;
        }
        if move_down {
            trans_y += speed;
        }

        if turn_forward {
            rot_x += speed;
        }
        if turn_back {
            rot_x -= speed;
        }
        if turn_left {
            rot_y -= speed;
        }
        if turn_right {
            rot_y += speed;
        }
        if turn_up {
            rot_z -= speed;
        }
        if turn_down {
            rot_z += speed;
        }

        let translation = Matrix4f32::new().init_translation(trans_x, trans_y, trans_z);
        let rotation = Matrix4f32::new().init_rotation(rot_x, rot_y, rot_z);
        let transform = &projection.mul(&translation.mul(&rotation));

        render_context.clear();
        render_context.draw_mesh(&mesh, &transform, &texture);
        render_context.sync();

        frame_cnt += 1_f32;
        counter_duration += delta_ms;
        if counter_duration > 1000_f32 {
            println!("FPS: {}", frame_cnt / counter_duration * 1000_f32);
            frame_cnt = 0_f32;
            counter_duration = 0_f32;
        }
        thread::yield_now();
    }
}
