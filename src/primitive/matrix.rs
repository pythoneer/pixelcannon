use primitive::vector::Vector4f32;

pub struct Matrix4f32 {
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
