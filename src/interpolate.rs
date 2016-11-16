use primitive::vertex::Vertex;

pub struct Interpolator {
    pub tex_coords_x: [f32; 3],
    pub tex_coords_y: [f32; 3],
    pub one_over_z: [f32; 3],

    pub tex_coords_step_xx: f32,
    pub tex_coords_step_xy: f32,
    pub tex_coords_step_yx: f32,
    pub tex_coords_step_yy: f32,

    pub one_over_step_zx: f32,
    pub one_over_step_zy: f32
}

impl Interpolator {
    pub fn new(min_vert: &Vertex, mid_vert: &Vertex, max_vert: &Vertex) -> Interpolator {
        let one_over_dx = 1_f32 /
            (((mid_vert.pos.x - max_vert.pos.x) *
            (min_vert.pos.y - max_vert.pos.y)) -
            ((min_vert.pos.x - max_vert.pos.x) *
            (mid_vert.pos.y - max_vert.pos.y)));

        let one_over_dy = -one_over_dx;

        let mut _one_over_z = [0f32; 3];
        let mut _tex_coords_x = [0f32; 3];
        let mut _tex_coords_y = [0f32; 3];

        let mut _tex_coords_step_xx = 0f32;
        let mut _tex_coords_step_xy = 0f32;
        let mut _tex_coords_step_yx = 0f32;
        let mut _tex_coords_step_yy = 0f32;

        let mut _one_over_step_zx = 0f32;
        let mut _one_over_step_zy = 0f32;

        _one_over_z[0] = 1.0f32/min_vert.pos.w;
        _one_over_z[1] = 1.0f32/mid_vert.pos.w;
        _one_over_z[2] = 1.0f32/max_vert.pos.w;

        _tex_coords_x[0] = min_vert.tex_coords.x * _one_over_z[0];
        _tex_coords_x[1] = mid_vert.tex_coords.x * _one_over_z[1];
        _tex_coords_x[2] = max_vert.tex_coords.x * _one_over_z[2];

        _tex_coords_y[0] = min_vert.tex_coords.y * _one_over_z[0];
        _tex_coords_y[1] = mid_vert.tex_coords.y * _one_over_z[1];
        _tex_coords_y[2] = max_vert.tex_coords.y * _one_over_z[2];

        _tex_coords_step_xx = Interpolator::calc_step_x(_tex_coords_x, min_vert, mid_vert, max_vert, one_over_dx);
        _tex_coords_step_xy = Interpolator::calc_step_y(_tex_coords_x, min_vert, mid_vert, max_vert, one_over_dy);
        _tex_coords_step_yx = Interpolator::calc_step_x(_tex_coords_y, min_vert, mid_vert, max_vert, one_over_dx);
        _tex_coords_step_yy = Interpolator::calc_step_y(_tex_coords_y, min_vert, mid_vert, max_vert, one_over_dy);
        _one_over_step_zx = Interpolator::calc_step_x(_one_over_z, min_vert, mid_vert, max_vert, one_over_dx);
        _one_over_step_zy = Interpolator::calc_step_y(_one_over_z, min_vert, mid_vert, max_vert, one_over_dy);

        Interpolator{
            tex_coords_x: _tex_coords_x,
            tex_coords_y: _tex_coords_y,
            one_over_z: _one_over_z,

            tex_coords_step_xx: _tex_coords_step_xx,
            tex_coords_step_xy: _tex_coords_step_xy,
            tex_coords_step_yx: _tex_coords_step_yx,
            tex_coords_step_yy: _tex_coords_step_yy,

            one_over_step_zx: _one_over_step_zx,
            one_over_step_zy: _one_over_step_zy
        }
    }

    fn calc_step_x(values: [f32; 3], min_vert: &Vertex, mid_vert: &Vertex, max_vert: &Vertex, one_over_dx: f32) -> f32 {
        let val =   (((values[1] - values[2]) *
                    (min_vert.pos.y - max_vert.pos.y)) -
                    ((values[0] - values[2]) *
                    (mid_vert.pos.y - max_vert.pos.y))) * one_over_dx;

        val
    }

    fn calc_step_y(values: [f32; 3], min_vert: &Vertex, mid_vert: &Vertex, max_vert: &Vertex, one_over_dy: f32) -> f32 {
        let val =   (((values[1] - values[2]) *
                    (min_vert.pos.x - max_vert.pos.x)) -
                    ((values[0] - values[2]) *
                    (mid_vert.pos.x - max_vert.pos.x))) * one_over_dy;

        val
    }
}
