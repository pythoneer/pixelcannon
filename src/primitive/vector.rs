#[derive(Debug, Copy, Clone)]
pub struct Vector4f32 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32
}

impl Vector4f32 {

    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Vector4f32 {
        Vector4f32 {
            x: x,
            y: y,
            z: z,
            w: w
        }
    }

    pub fn add_v(&self, other: &Vector4f32) -> Vector4f32 {
        Vector4f32::new(self.x + other.x, self.y + other.y, self.z + other.z, self.w + other.w)
    }

    pub fn sub_v(&self, other: &Vector4f32) -> Vector4f32 {
        Vector4f32::new(self.x - other.x, self.y - other.y, self.z - other.z, self.w - other.w)
    }

    pub fn length(&self) -> f32 {
        ((self.x * self.x) + (self.y * self.y) + (self.z * self.z) + (self.w * self.w)).sqrt()
    }

    pub fn normalized(&self) -> Vector4f32 {
        let length = self.length();
        Vector4f32::new(self.x / length, self.y / length, self.z / length, self.w / length)
    }

    pub fn cross(&self, other: &Vector4f32) -> Vector4f32 {
        let x = self.y * other.z - self.z * other.y;
        let y = self.z * other.x - self.x * other.z;
        let z = self.x * other.y - self.y * other.x;

        Vector4f32::new(x, y, z, 0_f32)
    }
}
