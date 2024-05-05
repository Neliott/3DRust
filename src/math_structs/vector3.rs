pub struct Vector3{
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Vector3{
    pub fn new(x: f32, y: f32, z: f32) -> Vector3{
        Vector3{x,y,z}
    }
    pub fn zero() -> Vector3{
        Vector3{x: 0f32, y: 0f32, z: 0f32}
    }
    pub fn one() -> Vector3{
        Vector3{x: 1f32, y: 1f32, z: 1f32}
    }
}

impl Clone for Vector3 {
    fn clone(&self) -> Self {
        Vector3 {
            x: self.x,
            y: self.y,
            z: self.z
        }
    }
}

impl Copy for Vector3{}