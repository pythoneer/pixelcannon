extern crate orbclient;

use orbclient::window::EventIter;

use std::time::Instant;

struct Matrix4f32 {
    pub m: [[f32; 4]; 4]
}

impl Matrix4f32 {
    pub fn new() -> Matrix4f32 {
        Matrix4f32{m: [[0f32; 4]; 4]}
    }

    pub fn init_perspective(mut self, fov: f32, aspect_ratio: f32, z_near: f32, z_far: f32) -> Matrix4f32 {
        let tan_half_fov = (fov / 2f32).tan();
        let z_range = z_near - z_far;

        self.m[0][0] = 1f32 / (tan_half_fov * aspect_ratio);      self.m[0][1] = 0f32;                      self.m[0][2] = 0f32;                            self.m[0][3] = 0f32;
        self.m[1][0] = 0f32;                                      self.m[1][1] = 1f32 / tan_half_fov;       self.m[1][2] = 0f32;                            self.m[1][3] = 0f32;
        self.m[2][0] = 0f32;                                      self.m[2][1] = 0f32;                      self.m[2][2] = (- z_near -z_far) / z_range;     self.m[2][3] = 2f32 * z_far * z_near / z_range;
        self.m[3][0] = 0f32;                                      self.m[3][1] = 0f32;                      self.m[3][2] = 1f32;                            self.m[3][3] = 0f32;

        self
    }

    pub fn init_translation(mut self, x: f32, y: f32, z: f32) -> Matrix4f32 {
        self.m[0][0] = 1f32;    self.m[0][1] = 0f32;    self.m[0][2] = 0f32;    self.m[0][3] = x;
        self.m[1][0] = 0f32;    self.m[1][1] = 1f32;    self.m[1][2] = 0f32;    self.m[1][3] = y;
        self.m[2][0] = 0f32;    self.m[2][1] = 0f32;    self.m[2][2] = 1f32;    self.m[2][3] = z;
        self.m[3][0] = 0f32;    self.m[3][1] = 0f32;    self.m[3][2] = 0f32;    self.m[3][3] = 1f32;

        self
    }

    pub fn init_rotation(mut self, x: f32, y: f32, z: f32) -> Matrix4f32 {

        let mut rx = Matrix4f32::new();
        let mut ry = Matrix4f32::new();
        let mut rz = Matrix4f32::new();

        rz.m[0][0] = z.cos();   rz.m[0][1] = -z.sin();  rz.m[0][2] = 0f32;      rz.m[0][3] = 0f32;
        rz.m[1][0] = z.sin();   rz.m[1][1] = z.cos();   rz.m[1][2] = 0f32;      rz.m[1][3] = 0f32;
        rz.m[2][0] = 0f32;      rz.m[2][1] = 0f32;      rz.m[2][2] = 1f32;      rz.m[2][3] = 0f32;
        rz.m[3][0] = 0f32;      rz.m[3][1] = 0f32;      rz.m[3][2] = 0f32;      rz.m[3][3] = 1f32;

        rx.m[0][0] = 1f32;      rx.m[0][1] = 0f32;      rx.m[0][2] = 0f32;      rx.m[0][3] = 0f32;
        rx.m[1][0] = 0f32;      rx.m[1][1] = x.cos();   rx.m[1][2] = -x.sin();  rx.m[1][3] = 0f32;
        rx.m[2][0] = 0f32;      rx.m[2][1] = x.sin();   rx.m[2][2] = x.cos();   rx.m[2][3] = 0f32;
        rx.m[3][0] = 0f32;      rx.m[3][1] = 0f32;      rx.m[3][2] = 0f32;      rx.m[3][3] = 1f32;

        ry.m[0][0] = y.cos();   ry.m[0][1] = 0f32;      ry.m[0][2] = -y.sin();  ry.m[0][3] = 0f32;
        ry.m[1][0] = 0f32;      ry.m[1][1] = 1f32;      ry.m[1][2] = 0f32;      ry.m[1][3] = 0f32;
        ry.m[2][0] = y.sin();   ry.m[2][1] = 0f32;      ry.m[2][2] = y.cos();   ry.m[2][3] = 0f32;
        ry.m[3][0] = 0f32;      ry.m[3][1] = 0f32;      ry.m[3][2] = 0f32;      ry.m[3][3] = 1f32;

        self.m = rz.mul(&ry.mul(&rx)).m;

        self
    }

    pub fn init_sreenspace_transform(mut self, half_width: f32, half_height: f32) -> Matrix4f32 {
        self.m[0][0] = half_width;  self.m[0][1] = 0f32;            self.m[0][2] = 0f32;    self.m[0][3] = half_width;
        self.m[1][0] = 0f32;        self.m[1][1] = -half_height;    self.m[1][2] = 0f32;    self.m[1][3] = half_height;
        self.m[2][0] = 0f32;        self.m[2][1] = 0f32;            self.m[2][2] = 1f32;    self.m[2][3] = 0f32;
        self.m[3][0] = 0f32;        self.m[3][1] = 0f32;            self.m[3][2] = 0f32;    self.m[3][3] = 1f32;

        self
    }

    pub fn transform(&self, other: &Vector4f32) -> Vector4f32 {
        Vector4f32 {
            x: self.m[0][0] * other.x + self.m[0][1] * other.y + self.m[0][2] * other.z + self.m[0][3] * other.w,
            y: self.m[1][0] * other.x + self.m[1][1] * other.y + self.m[1][2] * other.z + self.m[1][3] * other.w,
            z: self.m[2][0] * other.x + self.m[2][1] * other.y + self.m[2][2] * other.z + self.m[2][3] * other.w,
            w: self.m[3][0] * other.x + self.m[3][1] * other.y + self.m[3][2] * other.z + self.m[3][3] * other.w }
    }

    pub fn mul(&self, other: &Matrix4f32) -> Matrix4f32 {

        let mut ret = Matrix4f32::new();

        for c_idx in 0..4 {
            for r_idx in 0..4 {
                ret.m[c_idx][r_idx] =
                self.m[c_idx][0] * other.m[0][r_idx] +
                self.m[c_idx][1] * other.m[1][r_idx] +
                self.m[c_idx][2] * other.m[2][r_idx] +
                self.m[c_idx][3] * other.m[3][r_idx];
            }
        }

        ret
    }
}

struct Vector4f32 {
    x: f32,
    y: f32,
    z: f32,
    w: f32
}

struct Vertex {
    pos: Vector4f32
}

impl Vertex {

    pub fn new(_x: f32, _y: f32, _z: f32) -> Vertex {
        Vertex{pos: Vector4f32{x: _x, y: _y, z: _z, w: 1f32}}
    }

    pub fn new_with_vector4f32(_pos: Vector4f32) -> Vertex {
        Vertex{pos: _pos}
    }

    pub fn calc_double_area(&self, v1: &Vertex, v2: &Vertex) -> i32 {

        let x1 = (v1.pos.x as i32 - self.pos.x as i32) as i32;
        let y1 = (v1.pos.y as i32 - self.pos.y as i32) as i32;
        let x2 = (v2.pos.x as i32 - self.pos.x as i32) as i32;
        let y2 = (v2.pos.y as i32 - self.pos.y as i32) as i32;

        (x1 * y2 - x2 * y1)
    }

    pub fn transform(&self, transform: &Matrix4f32) -> Vertex {
        Vertex::new_with_vector4f32(transform.transform(&self.pos))
    }

    pub fn perspective_divide(&self) -> Vertex {
        Vertex::new_with_vector4f32(Vector4f32{ x: self.pos.x / self.pos.w, y: self.pos.y / self.pos.w, z: self.pos.z / self.pos.w, w: self.pos.w})
    }

}

struct Edge {
    pos_x: f32,
    step_x: f32,
    start_y: i32,
    end_y: i32
}

impl Edge {
    pub fn new(min_vert: &Vertex, max_vert: &Vertex) -> Edge {

        let dist_y = max_vert.pos.y - min_vert.pos.y;
        let dist_x = max_vert.pos.x - min_vert.pos.x;
        let prestep_y = min_vert.pos.y.ceil() - min_vert.pos.y;
        let _step_x = dist_x as f32 / dist_y as f32;

        Edge {
            pos_x: min_vert.pos.x + prestep_y * _step_x,
            step_x: _step_x,
            start_y: min_vert.pos.y.ceil() as i32,
            end_y: max_vert.pos.y.ceil() as i32,
        }
    }

    pub fn step(&mut self) {
        self.pos_x += self.step_x;
    }
}

struct RenderContext {
    window: Box<orbclient::Window>,
}

impl RenderContext {
    pub fn new(width: u32, height: u32, title: &str) -> RenderContext {
        let orb_window = orbclient::Window::new_flags(100, 100, width, height, title, true).unwrap();
        RenderContext{window: orb_window}
    }

    pub fn get_height(&self) -> u32 {
        self.window.height()
    }

    pub fn get_width(&self) -> u32 {
        self.window.width()
    }

    pub fn events(&mut self) -> EventIter {
        self.window.events()
    }

    pub fn clear(&mut self) {
        self.window.set(orbclient::Color { data: 0xFF220CE8 });
    }

    pub fn sync(&mut self) {
        self.window.sync();
    }

    pub fn draw_triangle(&mut self, v1: &Vertex, v2: &Vertex, v3: &Vertex) {

        let screen_space_transform = Matrix4f32::new().init_sreenspace_transform(self.get_width() as f32 / 2f32, self.get_height() as f32 / 2f32);

        let mut min_vert = v1.transform(&screen_space_transform).perspective_divide();
        let mut mid_vert = v2.transform(&screen_space_transform).perspective_divide();
        let mut max_vert = v3.transform(&screen_space_transform).perspective_divide();

        if max_vert.pos.y < mid_vert.pos.y {
            std::mem::swap(&mut mid_vert, &mut max_vert);
        }

        if mid_vert.pos.y < min_vert.pos.y {
            std::mem::swap(&mut mid_vert, &mut min_vert);
        }

        if max_vert.pos.y < mid_vert.pos.y {
            std::mem::swap(&mut max_vert, &mut mid_vert);
        }

        self.scan_triangle(&min_vert, &mid_vert, &max_vert, min_vert.calc_double_area(&max_vert, &mid_vert) >= 0);
    }

    fn scan_triangle(&mut self,  min_vert: &Vertex, mid_vert: &Vertex, max_vert: &Vertex, side: bool) {
        let mut top_to_bottom = Edge::new(min_vert, max_vert);
        let mut top_to_middle = Edge::new(min_vert, mid_vert);
        let mut middle_to_bottom = Edge::new(mid_vert, max_vert);

        self.scan_edges(&mut top_to_bottom, &mut top_to_middle, side);
        self.scan_edges(&mut top_to_bottom, &mut middle_to_bottom, side);
    }

    fn scan_edges(&mut self, first: &mut Edge, second: &mut Edge, side: bool) {
        let start_y = second.start_y;
        let end_y = second.end_y;

        let mut left = first;
        let mut right = second;

        if side {
            std::mem::swap(&mut left, &mut right);
        }

        for idx_y in start_y..end_y {
            self.draw_scan_line(&left, &right, idx_y);
            left.step();
            right.step();
        }
    }

    fn draw_scan_line(&mut self, left: &Edge, right: &Edge, idx_y: i32) {
        let min_x = left.pos_x.ceil() as i32;
        let max_x = right.pos_x.ceil()as i32;

        for idx_x in min_x..max_x {
            self.window.pixel(idx_x, idx_y, orbclient::Color { data: 0xFFE8A90C });
        }
    }
}

fn main() {

    let mut render_context = RenderContext::new(500, 400, "pixelcannon");
    let mut start = Instant::now();

    let min_vert = Vertex::new(-1f32, -1f32, 0f32);
    let mid_vert = Vertex::new( 0f32,  1f32, 0f32);
    let max_vert = Vertex::new( 1f32, -1f32, 0f32);

    let projection = Matrix4f32::new().init_perspective(70.0f32.to_radians(), render_context.get_width() as f32 / render_context.get_height() as f32, 0.1f32, 1000f32);

    let mut rot_cnt = 0f32;

    let mut frame_cnt = 0f32;
    let mut counter_duration = 0f32;

    'event: loop {

        {
            let end = Instant::now();
            let delta = end.duration_since(start);
            let delta_ms = delta.as_secs() as f32 * 1000f32 + (delta.subsec_nanos() as f32)/1000000 as f32;
            start = Instant::now();
            // println!("{:?} ms", delta_ms);

            rot_cnt += delta_ms / 1000f32;
            let translation = Matrix4f32::new().init_translation(0.0f32, 0.0f32, 4.3f32 + rot_cnt.sin() * 2f32);
            let rotation = Matrix4f32::new().init_rotation(rot_cnt, rot_cnt, 0.0f32);
            let transform = &projection.mul(&translation.mul(&rotation));

            render_context.clear();
            render_context.draw_triangle(&min_vert.transform(&transform), &mid_vert.transform(&transform), &max_vert.transform(&transform));
            render_context.sync();

            frame_cnt += 1f32;
            counter_duration += delta_ms;
            if counter_duration > 1000f32 {
                println!("FPSxx: {}", frame_cnt / counter_duration * 1000f32);
                frame_cnt = 0f32;
                counter_duration = 0f32;
            }

            // break 'event;
        }

        for orbital_event in render_context.events() {
            match orbital_event.to_option() {
                orbclient::EventOption::Quit(_quit_event) => break 'event,
                _ => (),
            };
        }

    }
}
