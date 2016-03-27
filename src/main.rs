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

    pub fn init_perspective(&mut self, fov: f32, aspect_ratio: f32, z_near: f32, z_far: f32) -> &mut Matrix4f32 {
        let tan_half_fov = (fov / 2f32).tan();
        let z_range = z_near - z_far;

        self.m[0][0] = 1f32 / (tan_half_fov * aspect_ratio);      self.m[0][1] = 0f32;                      self.m[0][2] = 0f32;                            self.m[0][3] = 0f32;
        self.m[1][0] = 0f32;                                      self.m[1][1] = 1f32 / tan_half_fov;       self.m[1][2] = 0f32;                            self.m[1][3] = 0f32;
        self.m[2][0] = 0f32;                                      self.m[2][1] = 0f32;                      self.m[2][2] = (- z_near -z_far) / z_range;     self.m[2][3] = 2f32 * z_far * z_near / z_range;
        self.m[3][0] = 0f32;                                      self.m[3][1] = 0f32;                      self.m[3][2] = 1f32;                            self.m[3][3] = 0f32;

        self
    }

    pub fn init_translation(&mut self, x: f32, y: f32, z: f32) -> &mut Matrix4f32 {
        self.m[0][0] = 1f32;    self.m[0][1] = 0f32;    self.m[0][2] = 0f32;    self.m[0][3] = x;
        self.m[1][0] = 0f32;    self.m[1][1] = 1f32;    self.m[1][2] = 0f32;    self.m[1][3] = y;
        self.m[2][0] = 0f32;    self.m[2][1] = 0f32;    self.m[2][2] = 1f32;    self.m[2][3] = z;
        self.m[3][0] = 0f32;    self.m[3][1] = 0f32;    self.m[3][2] = 0f32;    self.m[3][3] = 1f32;

        self
    }

    pub fn init_rotation(&mut self, x: f32, y: f32, z: f32) -> &mut Matrix4f32 {

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

    pub fn init_sreenspace_transform(&mut self, half_width: f32, half_height: f32) -> &mut Matrix4f32 {
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

struct RenderContext {
    window: Box<orbclient::Window>,
    scan_buffer: Vec<i32> //TODO(dustin): do i need Vec<32> here? [i32]
}

impl RenderContext {
    pub fn new(width: u32, height: u32, title: &str) -> RenderContext {
        let orb_window = orbclient::Window::new_flags(100, 100, width, height, title, true).unwrap();
        RenderContext{scan_buffer: vec![0; (height * 2) as usize], window: orb_window}
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

        let mut inter = Matrix4f32::new(); //TODO(dustin): fix this
        let screen_space_transform = inter.init_sreenspace_transform(self.get_width() as f32 / 2f32, self.get_height() as f32 / 2f32);

        let mut min_vert = v1.transform(screen_space_transform).perspective_divide();
        let mut mid_vert = v2.transform(screen_space_transform).perspective_divide();
        let mut max_vert = v3.transform(screen_space_transform).perspective_divide();

        if max_vert.pos.y < mid_vert.pos.y {
            let tmp = max_vert;
            max_vert = mid_vert;
            mid_vert = tmp;
        }

        if mid_vert.pos.y < min_vert.pos.y {
            let tmp = mid_vert;
            mid_vert = min_vert;
            min_vert = tmp;
        }

        if max_vert.pos.y < mid_vert.pos.y {
            let tmp = max_vert;
            max_vert = mid_vert;
            mid_vert = tmp;
        }

        let area = min_vert.calc_double_area(&max_vert, &mid_vert);
        let side = if area >= 0 { 1 } else { 0 };
        self.convert_triangle(&min_vert, &mid_vert, &max_vert, side);
        self.fill_convex_shape(min_vert.pos.y as i32, max_vert.pos.y as i32);
    }

    fn fill_convex_shape(&mut self, y_min: i32, y_max: i32) {

        for y_idx in y_min..y_max {
            let x_min = self.scan_buffer.get((y_idx * 2) as usize).unwrap().clone();
            let x_max = self.scan_buffer.get((y_idx * 2 + 1) as usize).unwrap().clone();

            for x_idx in x_min..x_max {
                self.window.pixel(x_idx, y_idx, orbclient::Color { data: 0xFFE8A90C });
            }
        }
    }

    fn convert_triangle(&mut self, min_vert: &Vertex, mid_vert: &Vertex, max_vert: &Vertex, side: i32) {
        self.convert_line(min_vert, max_vert, 0 + side);
        self.convert_line(min_vert, mid_vert, 1 - side);
        self.convert_line(mid_vert, max_vert, 1 - side);
    }

    fn convert_line(&mut self, min_vert: &Vertex, max_vert: &Vertex, side: i32) {
        let start_y = min_vert.pos.y;
        let start_x = min_vert.pos.x;
        let end_y = max_vert.pos.y;
        let end_x = max_vert.pos.x;

        let dist_y = end_y - start_y;
        let dist_x = end_x - start_x;

        if dist_y <= 0f32 {
            return;
        }

        let step_x = dist_x as f32 / dist_y as f32;
        let mut current_x = start_x as f32;

        for y_coord in start_y as i32..end_y as i32 {
            self.scan_buffer[((y_coord * 2 + side) as usize)] = current_x as i32;
            current_x += step_x;
        }
    }

}

fn main() {

    let mut render_context = RenderContext::new(500, 400, "pixelcannon");
    let mut start = Instant::now();

    let min_vert = Vertex::new(-1f32, -1f32, 0f32);
    let mid_vert = Vertex::new( 0f32,  1f32, 0f32);
    let max_vert = Vertex::new( 1f32, -1f32, 0f32);

    let mut inter = Matrix4f32::new();//TODO(dustin): fix this
    let projection = inter.init_perspective(70.0f32.to_radians(), render_context.get_width() as f32 / render_context.get_height() as f32, 0.1f32, 1000f32);

    let mut rot_cnt = 0.0f32;

    'event: loop {

        {
            let end = Instant::now();
            let delta = end.duration_since(start);
            let delta_ms = delta.as_secs() as f32 * 1000f32 + (delta.subsec_nanos() as f32)/1000000000 as f32;
            start = Instant::now();
            // println!("{:?} ms", delta_ms);

            rot_cnt += delta_ms as f32;
            let mut inter2 = Matrix4f32::new();//TODO(dustin): fix this
            let translation = inter2.init_translation(0.0f32, 0.0f32, 3.0f32 + rot_cnt.sin());
            let mut inter3 = Matrix4f32::new();//TODO(dustin): fix this
            let rotation = inter3.init_rotation(rot_cnt, rot_cnt, 0.0f32);
            let transform = &projection.mul(&translation.mul(&rotation));

            render_context.clear();
            render_context.draw_triangle(&min_vert.transform(&transform), &mid_vert.transform(&transform), &max_vert.transform(&transform));
            render_context.sync();

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
