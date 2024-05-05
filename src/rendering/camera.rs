use crate::game::transform::Transform;
use crate::math_structs::matrix4x4::Matrix4x4;
use crate::math_structs::utils::multiply_matrix;

pub struct Camera{
    pub transform: Transform,
    pub fov: f32,
    pub near: f32,
    pub far: f32,
}

impl Camera{
    pub fn new(transform: Transform) -> Camera{
        Camera{
            transform,
            fov: 90f32,
            near: 0.1f32,
            far: 1000f32,
        }
    }
    fn get_rotation_matrix(&self) -> Matrix4x4{
        let x = self.transform.rotation.x.to_radians();
        let y = self.transform.rotation.y.to_radians();
        let z = self.transform.rotation.z.to_radians();
        let a = x.cos();
        let b = x.sin();
        let c = y.cos();
        let d = y.sin();
        let e = z.cos();
        let f = z.sin();
        let ad = a * d;
        let bd = b * d;
        return [
            [c * e, -c * f, d, 0f32],
            [bd * e + a * f, -bd * f + a * e, -b * c, 0f32],
            [-ad * e + b * f, ad * f + b * e, a * c, 0f32],
            [0f32, 0f32, 0f32, 1f32]
        ]
    }
    fn get_view_matrix(&self) -> Matrix4x4{
        let mut matrix : Matrix4x4 = self.get_rotation_matrix();
        matrix[0][3] = self.transform.position.x;
        matrix[1][3] = self.transform.position.y;
        matrix[2][3] = self.transform.position.z;
        //matrix = multiply_matrix(matrix, self.get_rotation_matrix());
        matrix
    }
    fn get_projection_matrix(&self, aspect_ratio:f32) -> Matrix4x4{
        let near = self.near;
        let far = self.far;
        let f = 1f32 / (self.fov / 2f32).tan();
        let range = 1f32 / (near - far);
        return [
            [f / aspect_ratio, 0f32, 0f32, 0f32],
            [0f32, f, 0f32, 0f32],
            [0f32, 0f32, (near + far) * range, -1f32],
            [0f32, 0f32, near * far * range * 2f32, 0f32]
        ]
    }

    pub fn get_view_projection_matrix(&self, aspect_ratio:f32) -> Matrix4x4{
        multiply_matrix(self.get_projection_matrix(aspect_ratio), self.get_view_matrix())
    }
}