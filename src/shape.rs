use rusty_linear_algebra::vector_space::{ Matrix4, Scalar };
use luminance::tess::Tess;

pub struct Shape {
    pub tesselations: Vec<Tess>,
    pub position: Matrix4,
    pub rotation_matrix_x: Matrix4,
    pub rotation_matrix_y: Matrix4,
    pub rotation_matrix_z: Matrix4,
    rotation_angle_x: Scalar,
    rotation_angle_y: Scalar,
    rotation_angle_z: Scalar,
    pub scaling: Matrix4,
}

impl Shape {
    pub fn new(tesselations: Vec<Tess>) -> Shape {
        Shape {
            tesselations,
            position: Matrix4::identity(),
            rotation_matrix_x: Matrix4::identity(),
            rotation_matrix_y: Matrix4::identity(),
            rotation_matrix_z: Matrix4::identity(),
            rotation_angle_x: 0.0,
            rotation_angle_y: 0.0,
            rotation_angle_z: 0.0,
            scaling: Matrix4::identity(),
        }
    }
    pub fn get_tesselations(&self) -> &Vec<Tess> {
        &self.tesselations
    }
    pub fn get_transformation(&self) -> Matrix4 {
        let mut m4 = &(&self.position * &self.scaling) * &self.rotation_matrix_x;
        m4 = &m4 * &self.rotation_matrix_y;
        m4 = &m4 * &self.rotation_matrix_z;
        m4
    }
    //pub fn set_rotation(&mut self, angle: Scalar) {
    //    self.rotation_matrix = Matrix4::new_2d_rotation_y(angle);
    //    self.rotation_angle = angle;
    //}
    pub fn rotate_x(&mut self, angle: Scalar) {
        self.rotation_angle_x = self.rotation_angle_x + angle;
        self.rotation_matrix_x = Matrix4::new_2d_rotation_x(self.rotation_angle_x);
    }
    pub fn rotate_y(&mut self, angle: Scalar) {
        self.rotation_angle_y = self.rotation_angle_y + angle;
        self.rotation_matrix_y = Matrix4::new_2d_rotation_y(self.rotation_angle_y);
    }
    pub fn rotate_z(&mut self, angle: Scalar) {
        self.rotation_angle_z = self.rotation_angle_z + angle;
        self.rotation_matrix_z = Matrix4::new_2d_rotation_z(self.rotation_angle_z);
    }

    //pub fn get_rotation_angle(&self) -> Scalar {
    //    self.rotation_angle
    //}
}
