use doem_math::vector_space::Vector3;

// BULLET
lazy_static! {
    pub static ref BULLET_COLLIDER_HALF_SIZE: Vector3 = Vector3::new_from_array([[1.0], [1.0], [1.0]]);
    pub static ref BULLET_SCALE: Vector3 = Vector3::new_from_array([[1.0], [1.0], [1.0]]);
}
pub const BULLET_OBJ_PATH: &str = "models/bullet.obj";

// Starship
lazy_static! {
    pub static ref STARSHIP_BULLET_VELOCITY: Vector3 = Vector3::new_from_array([[5.0], [0.0], [0.0]]);
}
pub const STARSHIP_BULLET_DAMAGE: f32 = 20.0;

// DONUT
pub const DONUT_OBJ_PATH: &str = "models/donut.obj";

// reference plane
pub const REFERENCEPLANE_OBJ_PATH: &str = "models/reference_plane.obj";