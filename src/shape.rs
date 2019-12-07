use rusty_linear_algebra::vector_space::{ Matrix4, Scalar };
use luminance::tess::Tess;

pub struct Shape {
    pub tess: Tess,
    pub position: Matrix4,
    pub rotation_matrix: Matrix4,
    rotation_angle: Scalar,
    pub scaling: Matrix4,
}

impl Shape {
    pub fn new(tess: Tess) -> Shape {
        Shape {
            tess,
            position: Matrix4::identity(),
            rotation_matrix: Matrix4::identity(),
            rotation_angle: 0.0,
            scaling: Matrix4::identity(),
        }
    }
    pub fn get_tess(&self) -> &Tess {
        &self.tess
    }
    pub fn get_transformation(&self) -> Matrix4 {
        &(&self.position * &self.scaling) * &self.rotation_matrix
    }
    pub fn set_rotation(&mut self, angle: Scalar) {
        self.rotation_matrix = Matrix4::new_2d_rotation(angle);
        self.rotation_angle = angle;
    }
    pub fn rotate(&mut self, angle: Scalar) {
        self.rotation_angle = self.rotation_angle + angle;
        self.rotation_matrix = Matrix4::new_2d_rotation(self.rotation_angle);
    }
    pub fn get_rotation_angle(&self) -> Scalar {
        self.rotation_angle
    }
}
