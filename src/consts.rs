use doem_math::vector_space::Vector3;

// BULLET
lazy_static! {
    pub static ref BULLET_COLLIDER_HALF_SIZE: Vector3 = Vector3::new_from_array([[1.0], [1.0], [1.0]]);
    pub static ref BULLET_SCALE: Vector3 = Vector3::new_from_array([[2.0], [2.0], [2.0]]);
}
pub const BULLET_OBJ_PATH: &str = "models/bullet.obj";

// Starship
lazy_static! {
    pub static ref STARSHIP_BULLET_VELOCITY: Vector3 = Vector3::new_from_array([[5.0], [0.0], [0.0]]);
}
pub const STARSHIP_BULLET_DAMAGE: f32 = 20.0;
pub const STARSHIP_OBJ_PATH: &str = "models/starship.obj";

// DONUT
pub const DONUT_OBJ_PATH: &str = "models/donut.obj";

// nondescript circle
lazy_static! {
    pub static ref NONDESCRIPTCIRCLE_COLLIDER_HALF_SIZE: Vector3 = Vector3::new_from_array([[1.0], [1.0], [1.0]]);
    pub static ref NONDESCRIPTCIRCLE_SCALE: Vector3 = Vector3::new_from_array([[10.0], [10.0], [10.0]]);
    pub static ref NONDESCRIPTCIRCLE_SPEED: Vector3 = Vector3::new_from_array([[0.05], [0.05], [0.05]]);
    pub static ref NONDESCRIPTCIRCLE_MIN_SCALE: Vector3 = Vector3::new_from_array([[4.0], [4.0], [4.0]]);
    pub static ref NONDESCRIPTCIRCLE_MAX_SCALE: Vector3 = Vector3::new_from_array([[10.0], [10.0], [10.0]]);
}
pub const NONDESCRIPTCIRCLE_OBJ_PATH: &str = "models/nondescript_circle.obj";

// reference plane
pub const REFERENCEPLANE_OBJ_PATH: &str = "models/reference_plane.obj";