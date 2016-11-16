use primitive::matrix::Matrix4f32;
use primitive::vector::Vector4f32;

pub struct Vertex {
    pub pos: Vector4f32,
    pub tex_coords: Vector4f32 //TODO(dustin): don't waste space here we only need 2 values
}

impl Vertex {
    // pub fn new(_x: f32, _y: f32, _z: f32) -> Vertex {
    //     Vertex{pos: Vector4f32{x: _x, y: _y, z: _z, w: 1f32}}
    // }

    pub fn new_with_pos_and_texcoords(_pos: Vector4f32, _coords: Vector4f32) -> Vertex {
        Vertex{pos: _pos, tex_coords: _coords}
    }

    pub fn calc_double_area(&self, v1: &Vertex, v2: &Vertex) -> i32 {
        let x1 = (v1.pos.x as i32 - self.pos.x as i32) as i32;
        let y1 = (v1.pos.y as i32 - self.pos.y as i32) as i32;
        let x2 = (v2.pos.x as i32 - self.pos.x as i32) as i32;
        let y2 = (v2.pos.y as i32 - self.pos.y as i32) as i32;

        (x1 * y2 - x2 * y1)
    }

    //TODO(dustin): fix this!
    pub fn transform(&self, transform: &Matrix4f32) -> Vertex {
        Vertex::new_with_pos_and_texcoords(transform.transform(&self.pos), Vector4f32{x: self.tex_coords.x, y: self.tex_coords.y, z: self.tex_coords.z, w: self.tex_coords.w})
    }

    pub fn perspective_divide(&self) -> Vertex {
        Vertex::new_with_pos_and_texcoords(Vector4f32{ x: self.pos.x / self.pos.w, y: self.pos.y / self.pos.w, z: self.pos.z / self.pos.w, w: self.pos.w}, Vector4f32{x: self.tex_coords.x, y: self.tex_coords.y, z: self.tex_coords.z, w: self.tex_coords.w})
    }
}
