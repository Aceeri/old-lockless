
use cgmath::Vector3;

pub fn linear_interpolate(from: f32, to: f32, amount: f32) -> f32 {
    (from * (1.0 - amount) + to * amount)
}

pub fn vector_interpolate(from: Vector3<f32>, to: Vector3<f32>, amount: f32) -> Vector3<f32> {
    Vector3::new(
        linear_interpolate(from.x, to.x, amount),
        linear_interpolate(from.y, to.y, amount),
        linear_interpolate(from.z, to.z, amount),
    )
}

