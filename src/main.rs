extern crate orbclient;
extern crate time;

use orbclient::window::EventIter;
use time::PreciseTime;

struct RenderContext {
    window: Box<orbclient::Window>,
    scan_buffer: Vec<u32> //TODO(dustin): do i need Vec<32> here? [i32]
}

impl RenderContext {
    pub fn new(width: u32, height: u32, title: &str) -> RenderContext {
        let orb_window = orbclient::Window::new_flags(0, 0, width, height, title, true).unwrap();
        RenderContext{scan_buffer: vec![0; (height * 2) as usize], window: orb_window}
    }

    pub fn events(&mut self) -> EventIter {
        self.window.events()
    }

    pub fn draw_scan_buffer(&mut self, y_coord: u32, x_min: u32, x_max: u32 ) {
        self.scan_buffer.insert((y_coord * 2) as usize, x_min);
        self.scan_buffer.insert((y_coord * 2 + 1 ) as usize, x_max);

    }

    pub fn clear(&mut self) {
        self.window.set(orbclient::Color { data: 0xFF000000 });
    }

    pub fn sync(&mut self) {
        self.window.sync();
    }

    pub fn fill_shape(&mut self, y_min: u32, y_max: u32) {

        for y_idx in y_min..y_max {
            let x_min = self.scan_buffer.get((y_idx * 2) as usize).unwrap().clone();
            let x_max = self.scan_buffer.get((y_idx * 2 + 1) as usize).unwrap().clone();

            for x_idx in x_min..x_max {
                self.window.pixel(x_idx as i32, y_idx as i32, orbclient::Color { data: 0xFFFFFFFF });
            }
        }
    }
}

fn main() {

    let mut render_context = RenderContext::new(500, 400, "pixelcannon");
    let mut start = PreciseTime::now();

    'event: loop {

        {
            let end = PreciseTime::now();
            let delta = start.to(end);
            start = PreciseTime::now();

            println!("{} ms", delta.num_milliseconds());

            render_context.clear();
            for scan in 100..200 {
                render_context.draw_scan_buffer(scan, 300 - scan, 300 + scan);
            }
            render_context.fill_shape(100, 200);
            render_context.sync();
        }

        for orbital_event in render_context.events() {
            match orbital_event.to_option() {
                orbclient::EventOption::Quit(_quit_event) => break 'event,
                _ => (),
            };
        }

    }
}
