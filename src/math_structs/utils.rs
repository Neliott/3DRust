use crate::math_structs::matrix4x4::Matrix4x4;
use crate::math_structs::vector3::Vector3;
use crate::rendering::mesh_renderer::MeshRenderer;

pub fn multiply_matrix(a: Matrix4x4, b: Matrix4x4) -> Matrix4x4{
    let mut result : Matrix4x4 = MeshRenderer::identity_matrix();
    for i in 0..4{
        for j in 0..4{
            result[i][j] = a[i][0] * b[0][j] + a[i][1] * b[1][j] + a[i][2] * b[2][j] + a[i][3] * b[3][j];
        }
    }
    result
}

pub fn multiply_vector(v: &Vector3, m: &Matrix4x4) -> Vector3{
    //This is the correct matrix multiplication
    let mut result = Vector3::zero();
    result.x = v.x * m[0][0] + v.y * m[0][1] + v.z * m[0][2] + m[0][3];
    result.y = v.x * m[1][0] + v.y * m[1][1] + v.z * m[1][2] + m[1][3];
    result.z = v.x * m[2][0] + v.y * m[2][1] + v.z * m[2][2] + m[2][3];
    let w = v.x * m[3][0] + v.y * m[3][1] + v.z * m[3][2] + m[3][3];
    if w != 0f32{
        result.x /= w;
        result.y /= w;
        result.z /= w;
    }
    result
}