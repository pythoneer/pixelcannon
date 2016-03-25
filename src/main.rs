extern crate orbclient;

use orbclient::window::EventIter;

use std::time::Instant;

struct Vertex {
    x: i32,
    y: i32
}

impl Vertex {
    pub fn calc_double_area(&self, v1: &Vertex, v2: &Vertex) -> i32 {

        let x1 = (v1.x - self.x) as i32;
        let y1 = (v1.y - self.y) as i32;
        let x2 = (v2.x - self.x) as i32;
        let y2 = (v2.y - self.y) as i32;

        (x1 * y2 - x2 * y1)
    }
}

struct RenderContext {
    window: Box<orbclient::Window>,
    scan_buffer: Vec<i32> //TODO(dustin): do i need Vec<32> here? [i32]
}

impl RenderContext {
    pub fn new(width: u32, height: u32, title: &str) -> RenderContext {
        let orb_window = orbclient::Window::new_flags(100, 100, width, height, title, true).unwrap();
        RenderContext{scan_buffer: vec![0; (height * 2) as usize], window: orb_window}
    }

    pub fn events(&mut self) -> EventIter {
        self.window.events()
    }

    pub fn clear(&mut self) {
        self.window.set(orbclient::Color { data: 0xFF000000 });
    }

    pub fn sync(&mut self) {
        self.window.sync();
    }

    pub fn draw_triangle(&mut self, v1: &Vertex, v2: &Vertex, v3: &Vertex) {

        let mut min_vert = &v1;
        let mut mid_vert = &v2;
        let mut max_vert = &v3;

        if max_vert.y < mid_vert.y {
            let tmp = max_vert;
            max_vert = mid_vert;
            mid_vert = tmp;
        }

        if mid_vert.y < min_vert.y {
            let tmp = mid_vert;
            mid_vert = min_vert;
            min_vert = tmp;
        }

        if max_vert.y < mid_vert.y {
            let tmp = max_vert;
            max_vert = mid_vert;
            mid_vert = tmp;
        }

        let area = min_vert.calc_double_area(&max_vert, &mid_vert);
        let side = if area >= 0 { 1 } else { 0 };
        self.convert_triangle(&min_vert, &mid_vert, &max_vert, side);
        self.fill_convex_shape(min_vert.y, max_vert.y);
    }

    fn fill_convex_shape(&mut self, y_min: i32, y_max: i32) {

        for y_idx in y_min..y_max {
            let x_min = self.scan_buffer.get((y_idx * 2) as usize).unwrap().clone();
            let x_max = self.scan_buffer.get((y_idx * 2 + 1) as usize).unwrap().clone();

            for x_idx in x_min..x_max {
                self.window.pixel(x_idx, y_idx, orbclient::Color { data: 0xFFFFFFFF });
            }
        }
    }

    fn convert_triangle(&mut self, min_vert: &Vertex, mid_vert: &Vertex, max_vert: &Vertex, side: i32) {
        self.convert_line(min_vert, max_vert, 0 + side);
        self.convert_line(min_vert, mid_vert, 1 - side);
        self.convert_line(mid_vert, max_vert, 1 - side);
    }

    fn convert_line(&mut self, min_vert: &Vertex, max_vert: &Vertex, side: i32) {
        let start_y = min_vert.y;
        let start_x = min_vert.x;
        let end_y = max_vert.y;
        let end_x = max_vert.x;

        let dist_y = end_y - start_y;
        let dist_x = end_x - start_x;

        if dist_y <= 0 {
            return;
        }

        let step_x = dist_x as f32 / dist_y as f32;
        let mut current_x = start_x as f32;

        for y_coord in start_y..end_y {
            self.scan_buffer[((y_coord * 2 + side) as usize)] = current_x as i32;
            current_x += step_x;
        }
    }

}

fn main() {

    let mut render_context = RenderContext::new(500, 400, "pixelcannon");
    let mut start = Instant::now();

    let min_vert = Vertex{x: 250, y: 0};
    let mid_vert = Vertex{x: 0, y: 200};
    let max_vert = Vertex{x: 500, y: 300};

    'event: loop {

        {
            let end = Instant::now();
            let delta = end.duration_since(start);
            let delta_ms = delta.as_secs() * 1000 + (delta.subsec_nanos() as u64)/1000000;
            start = Instant::now();
            // println!("{} ms", delta_ms);

            render_context.clear();
            render_context.draw_triangle(&min_vert, &mid_vert, &max_vert);
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
