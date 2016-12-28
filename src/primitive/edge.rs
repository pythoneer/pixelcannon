use interpolate::Interpolator;
use primitive::vertex::Vertex;

pub struct Edge {
    pub pos_x: f32,
    pub step_x: f32,
    pub start_y: i32,
    pub end_y: i32,

    pub tex_coords_x: f32,
    pub tex_coords_step_x: f32,
    pub tex_coords_y: f32,
    pub tex_coords_step_y: f32,
    pub one_over_z: f32,
    pub one_over_step_z: f32
}

impl Edge {
    pub fn new(interpolator: &Interpolator, min_vert: &Vertex, max_vert: &Vertex, min_y_vert_index: i32) -> Edge {
        let dist_y = max_vert.pos.y - min_vert.pos.y;
        let dist_x = max_vert.pos.x - min_vert.pos.x;
        let prestep_y = min_vert.pos.y.ceil() - min_vert.pos.y;
        let _step_x = dist_x as f32 / dist_y as f32;
        let _pos_x = min_vert.pos.x + prestep_y * _step_x;
        let prestep_x = _pos_x - min_vert.pos.x;

        let _tex_coord_x = interpolator.tex_coords_x[min_y_vert_index as usize] +
            interpolator.tex_coords_step_xx * prestep_x +
            interpolator.tex_coords_step_xy * prestep_y;
        let _tex_coord_step_x = interpolator.tex_coords_step_xy + interpolator.tex_coords_step_xx * _step_x;

        let _tex_coord_y = interpolator.tex_coords_y[min_y_vert_index as usize] +
            interpolator.tex_coords_step_yx * prestep_x +
            interpolator.tex_coords_step_yy * prestep_y;
        let _tex_coord_step_y = interpolator.tex_coords_step_yy + interpolator.tex_coords_step_yx * _step_x;

        let _one_over_z = interpolator.one_over_z[min_y_vert_index as usize] +
            interpolator.one_over_step_zx * prestep_x +
            interpolator.one_over_step_zy * prestep_y;
        let _one_over_step_z = interpolator.one_over_step_zy + interpolator.one_over_step_zx * _step_x;


        Edge {
            pos_x: _pos_x,
            step_x: _step_x,
            start_y: min_vert.pos.y.ceil() as i32,
            end_y: max_vert.pos.y.ceil() as i32,

            tex_coords_x: _tex_coord_x,
            tex_coords_step_x: _tex_coord_step_x,
            tex_coords_y: _tex_coord_y,
            tex_coords_step_y: _tex_coord_step_y,
            one_over_z: _one_over_z,
            one_over_step_z: _one_over_step_z
        }
    }

    pub fn step(&mut self) {
        self.pos_x += self.step_x;
        self.tex_coords_x += self.tex_coords_step_x;
        self.tex_coords_y += self.tex_coords_step_y;
        self.one_over_z += self.one_over_step_z;
    }
}
