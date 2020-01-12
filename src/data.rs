use doem_math::Vector3;

#[derive(Clone)]
pub struct AABB {
    pub middle_point: Vector3,
    pub half_size: Vector3,
}
