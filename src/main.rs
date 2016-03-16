extern crate sdl;
extern crate rand;
extern crate time;

use rand::Rng;

use time::PreciseTime;

use sdl::video::{SurfaceFlag, VideoFlag, Surface};
use sdl::event::{Event, Key};

struct Color {
    r: u8,
    g: u8,
    b: u8
}

struct Stars_3D {
    speed: i32,
    spread: i32,
    stars_x: Vec<f32>,
    stars_y: Vec<f32>,
    stars_z: Vec<f32>
}

impl Stars_3D {
    pub fn new(num_stars: usize, _spread: i32, _speed: i32) -> Stars_3D {
        let mut obj = Stars_3D {
            spread: _spread,
            speed: _speed,
            stars_x: vec![0.0; num_stars],
            stars_y: vec![0.0; num_stars],
            stars_z: vec![0.0; num_stars] };

        for idx in 0..num_stars {
            obj.init_star(idx);
        }

        obj
    }

    fn init_star(&mut self, idx: usize) {
        let mut rng = rand::thread_rng();
        self.stars_x[idx] = (2 as f32) * (rng.next_f32() - 0.5) * (self.spread as f32);
        self.stars_y[idx] = (2 as f32) * (rng.next_f32() - 0.5) * (self.spread as f32);
        self.stars_z[idx] = (rng.next_f32() + 0.00001) * (self.spread as f32);
    }

    pub fn update_and_render(&mut self, screen: &Surface, delta: f32)
	{
        clear(screen);

        let color = Color{r:233, g:233, b:233};

		let halfWidth  = screen.get_width() as f32 / 2.0f32;
		let halfHeight = screen.get_height()as f32 / 2.0f32;
        for i in 0..self.stars_x.len() {
            self.stars_z[i] -= delta * self.speed as f32;

            if(self.stars_z[i] <= 0f32) {
                self.init_star(i);
            }

            let x = (self.stars_x[i]/self.stars_z[i] * halfWidth + halfWidth);
            let y = (self.stars_y[i]/self.stars_z[i] * halfHeight + halfHeight);

            if x < 0f32 || x >= screen.get_width() as f32 || y < 0f32 || y >= screen.get_height() as f32 {
                self.init_star(i);
            } else {
                set_pixel(x as i32, y as i32, &color, &screen);
            }
        }
	}
}


fn clear(screen: &Surface) {
    screen.fill_rect(Some(sdl::Rect {
                x: 0,
                y: 0,
                w: screen.get_width(),
                h: screen.get_height()
            }), sdl::video::Color::RGB(0,0,0));
}

fn set_pixel(_x: i32, _y:i32, color: &Color, screen: &Surface) {

    // println!("{}",screen.get_width());
    let x: usize = _x as usize * 4;
    let y: usize = _y as usize * 4;
    let idx_blue: usize = x + y * screen.get_width() as usize;
    let idx_green: usize = (x + y * screen.get_width() as usize) + 1;
    let idx_red: usize = (x + y * screen.get_width() as usize) + 2;
    let idx_alpha: usize = (x + y * screen.get_width() as usize) + 3;

    screen.with_lock(|pixels| {
        pixels[idx_blue] = color.b;
        pixels[idx_green] = color.g;
        pixels[idx_red] = color.r;
        pixels[idx_alpha] = 0;

        true
    });
}

fn draw_line(x0: i32, y0: i32, x1: i32, y1: i32, color: &Color, screen: &Surface) {
    for x in x0..x1 {
        // println!("{}", x);
        let t: f32 = (x-x0) as f32 /(x1-x0) as f32;
        let y: i32 = (y0 as f32 *(1.0-t) + y1 as f32 *t) as i32;
        // image.set(x, y, color);
        set_pixel(x, y, color, screen);
    }
}

fn main() {
    sdl::init(&[sdl::InitFlag::Video]);
    sdl::wm::set_caption("rust-sdl demo - video", "rust-sdl");

    let mut rng = rand::thread_rng();
    let screen = match sdl::video::set_video_mode(1024, 768, 32,
                                                  &[SurfaceFlag::HWSurface],
                                                  &[VideoFlag::DoubleBuf]) {
        Ok(screen) => screen,
        Err(err) => panic!("failed to set video mode: {}", err)
    };

    // let color = Color{r:233, g:233, b:233};

    // set_pixel(400, 300, &color, &screen);

    // draw_line(10, 10, 100, 100, &color, &screen);

    // draw_line(13, 20, 80, 40,  &color, &screen);
    // draw_line(20, 13, 40, 80,  &color, &screen);
    // draw_line(80, 40, 13, 20,  &color, &screen);

    let mut stars = Stars_3D::new(4000, 64, 20);

    let mut start = PreciseTime::now();

    'main : loop {
        'event : loop {
            match sdl::event::poll_event() {
                Event::Quit => break 'main,
                Event::None => break 'event,
                Event::Key(k, _, _, _)
                    if k == Key::Escape
                        => break 'main,
                _ => { }
            }
        }

        let end = PreciseTime::now();
        let delta = start.to(end);
        start = PreciseTime::now();

        println!("{} ms", delta.num_milliseconds());

        stars.update_and_render(&screen, delta.num_milliseconds() as f32 / 1000f32);
        screen.flip();

    }

    sdl::quit();
}
