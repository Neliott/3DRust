use sdl2::pixels::Color;
use crate::color::ColorUtils;
use crate::math_structs::matrix4x4::Matrix4x4;
use crate::math_structs::utils::multiply_vector;
use crate::math_structs::vector3::Vector3;

pub struct Tris{
    pub a: Vector3,
    pub b: Vector3,
    pub c: Vector3,
    pub color: Color
}

impl Tris {
    pub fn new(a: Vector3, b: Vector3, c: Vector3) -> Tris {
        Tris { a, b, c, color: ColorUtils::random() }
    }
    pub fn to_screen_space(&self, mvp_matrix: &Matrix4x4) -> Tris {
        let a = multiply_vector(&self.a, &mvp_matrix);
        let b = multiply_vector(&self.b, &mvp_matrix);
        let c = multiply_vector(&self.c, &mvp_matrix);
        Tris { a, b, c, color: self.color }
    }
}