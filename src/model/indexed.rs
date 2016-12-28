use primitive::vector::Vector4f32;

pub struct IndexedModel {
    pub positions: Vec<Vector4f32>,
    pub tex_coords: Vec<Vector4f32>,
    pub indices: Vec<i32>,
    pub tangents: Vec<Vector4f32>,
    pub normals: Vec<Vector4f32>
}

impl IndexedModel {
    pub fn new() -> IndexedModel {
        IndexedModel {
            positions: Vec::new(),
            tex_coords: Vec::new(),
            indices: Vec::new(),
            tangents: Vec::new(),
            normals: Vec::new()
        }
    }

    pub fn calc_tangents(&mut self) {
        //TODO(dustin): use idiomatic iterators and combinators
        for idx in (0..self.indices.len()).step_by(3) {
            let i0 = self.indices[idx as usize];
            let i1 = self.indices[(idx + 1) as usize];
            let i2 = self.indices[(idx + 2) as usize];

            let edge1 = self.positions[i1 as usize].sub_v(&self.positions[i0 as usize]);
            let edge2 = self.positions[i2 as usize].sub_v(&self.positions[i0 as usize]);

            let delta_u1 = self.tex_coords[i1 as usize].x - self.tex_coords[i0 as usize].x;
            let delta_v1 = self.tex_coords[i1 as usize].y - self.tex_coords[i0 as usize].y;
            let delta_u2 = self.tex_coords[i2 as usize].x - self.tex_coords[i0 as usize].x;
            let delta_v2 = self.tex_coords[i2 as usize].y - self.tex_coords[i0 as usize].y;

            let divident = delta_u1 * delta_v2 - delta_u2 * delta_v1;
            let f = if divident == 0_f32 { 0_f32 } else { 1_f32 / divident };

            let x = f * (delta_v2 * edge1.x - delta_v1 * edge2.x);
            let y = f * (delta_v2 * edge1.y - delta_v1 * edge2.y);
            let z = f * (delta_v2 * edge1.z - delta_v1 * edge2.z);
        }

        //TODO(dustin): use idiomatic iterators
        for idx in 0..self.normals.len() {
            self.tangents[idx as usize] = self.tangents[idx as usize].normalized();
        }
    }

    pub fn calc_normals(&mut self) {
        //TODO(dustin): use idiomatic iterators and combinators
        for idx in (0..self.indices.len()).step_by(3) {

            let i0 = self.indices[idx as usize];
            let i1 = self.indices[(idx + 1) as usize];
            let i2 = self.indices[(idx + 2) as usize];

            let v1 = self.positions[i1 as usize].sub_v(&self.positions[i0 as usize]);
            let v2 = self.positions[i2 as usize].sub_v(&self.positions[i0 as usize]);

            let normal = v1.cross(&v2).normalized();

            self.normals[i0 as usize] = self.normals[i0 as usize].add_v(&normal);
            self.normals[i1 as usize] = self.normals[i1 as usize].add_v(&normal);
            self.normals[i2 as usize] = self.normals[i2 as usize].add_v(&normal);
        }

        //TODO(dustin): use idiomatic iterators
        for idx in 0..self.normals.len() {
            self.normals[idx as usize] = self.normals[idx as usize].normalized();
        }
    }
}
