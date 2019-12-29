use doem_math::vector_space::{ Matrix4, Vector4, Scalar };
use luminance::tess::Tess;

pub struct Shape {
    pub tesselations: Vec<Tess>,
    pub position: Matrix4,
    orientation: Matrix4,
    rotation_angle_x: Scalar,
    rotation_angle_y: Scalar,
    rotation_angle_z: Scalar,
    pub scaling: Matrix4,
    pub middle_point: Vector4,
    pub x_half_size: f32,
    pub y_half_size: f32,
    pub z_half_size: f32,
}

impl Shape {
    pub fn new(tesselations: Vec<Tess>, middle_point: Vector4, x_half_size: f32, y_half_size: f32, z_half_size: f32) -> Shape {
        Shape {
            tesselations,
            position: Matrix4::identity(),
            orientation: Matrix4::identity(),
            rotation_angle_x: 0.0,
            rotation_angle_y: 0.0,
            rotation_angle_z: 0.0,
            scaling: Matrix4::identity(),
            middle_point,
            x_half_size,
            y_half_size,
            z_half_size
        }
    }
    // rotatie as = local x y of z
	//	// translatiematrix naar de oorsprong
	//	double[][] to = Matrix.getTranslationMatrix(-centrum.x, -centrum.y, -centrum.z);
	//	
	//	// rotatiematrix om de y-as naar het xy-vlak
	//	double[][] m1 = Matrix.getRotationMatrixM1(rotatieAs);
	//	
	//	// rotatiematrix om de z-as naar de x-as
	//	double[][] m2 = Matrix.getRotationMatrixM2(rotatieAs);
	//	
	//	// rotatie om de x-as
	//	double[][] m3 = Matrix.getRotationMatrixX(degrees);
	//	
	//	// rotatie om de z-as terug
	//	double[][] m4 = Matrix.getRotationMatrixM4(rotatieAs);
	
	//	// rotatie om de y-as terug
	//	double[][] m5 = Matrix.getRotationMatrixM5(rotatieAs);
	//	
	//	// translatiematrix "terug"
	//	double[][] tt = Matrix.getTranslationMatrix(centrum.x, centrum.y, centrum.z);
    pub fn get_x_vector(&self) {
        
    }
    pub fn get_tesselations(&self) -> &Vec<Tess> {
        &self.tesselations
    }
    fn get_rotation_matrix(&self, rotation_vector: &Vector4, angle: Scalar) -> Matrix4 {
        let x = rotation_vector[0][0];
        let y = rotation_vector[1][0];
        let z = rotation_vector[2][0];

        // M1
        let xz = Scalar::sqrt(x.powi(2) + z.powi(2));
        let mut m1 = Matrix4::identity();
        if xz != 0.0 {
            let new_x = x / xz;
            let new_z = z / xz;
            m1 = Matrix4::new_from_array([
                [new_x, 0.0, new_z, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [-new_z, 0.0, new_x, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ]);
        }

        // M2
        let mut m2 = Matrix4::identity();
        let xyz = Scalar::sqrt(x.powi(2) + y.powi(2) + z.powi(2));
        if xyz != 0.0 {
            m2 = Matrix4::new_from_array([
                [xz/xyz,         y/xyz,  0.0, 0.0],
                [-1.0 * (y/xyz), xz/xyz, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0]
            ]);
        }

        // M3
        let m3 = Matrix4::new_2d_rotation_x(angle);

        // M4
        let mut m4 = m2.clone();
        m4[0][1] *= -1.0;
        m4[1][0] *= -1.0;

        // M5
        let mut m5 = m1.clone();
        m5[0][2] *= -1.0;
        m5[2][0] *= -1.0;

        &(&(&(&m5 * &m4) * &m3) * &m2) * &m1
    }
    pub fn get_transformation(&self) -> Matrix4 {
        //let mut x = Vector4::new_from_array([[self.middle_point[0][0] + self.x_half_size], [0.0], [0.0], [0.0]]);
        //let mut y = Vector4::new_from_array([[0.0], [self.middle_point[1][0] + self.y_half_size], [0.0], [0.0]]);
        //let mut z = Vector4::new_from_array([[0.0], [0.0], [self.middle_point[2][0] + self.z_half_size], [0.0]]);

        //let z_rotation = self.get_rotation_matrix(&z, self.rotation_angle_z);
        //x = &z_rotation * &x;
        //y = &z_rotation * &y;
        //z = &z_rotation * &z;

        //let x_rotation = self.get_rotation_matrix(&x, self.rotation_angle_x);
        //x = &x_rotation * &x;
        //y = &x_rotation * &y;
        //z = &x_rotation * &z;

        //let y_rotation = self.get_rotation_matrix(&y, self.rotation_angle_y);
        //x = &y_rotation * &x;
        //y = &y_rotation * &y;
        //z = &y_rotation * &z;

        //let mut orientation = Matrix4::identity();
        //let x_rotation = Matrix4::new_2d_rotation_x(self.rotation_angle_x);
        //orientation = &x_rotation * &orientation;
        //let y_rotation = Matrix4::new_2d_rotation_y(self.rotation_angle_y);
        //orientation = &y_rotation * &orientation;
        //let z_rotation = Matrix4::new_2d_rotation_z(self.rotation_angle_z);
        //orientation = &z_rotation * &orientation;

        //let x_rotation = Matrix4::new_2d_rotation_x(self.rotation_angle_x);
        //let y_rotation = Matrix4::new_2d_rotation_y(self.rotation_angle_y);
        //let z_rotation = Matrix4::new_2d_rotation_z(self.rotation_angle_z);

        //let rotation = &(&y_rotation * &x_rotation) * &z_rotation;

        &self.position * &(&self.orientation * &self.scaling)
    }

    //pub fn set_rotation(&mut self, angle: Scalar) {
    //    self.rotation_matrix = Matrix4::new_2d_rotation_y(angle);
    //    self.rotation_angle = angle;
    //}
    pub fn rotate_x(&mut self, angle: Scalar) {
        println!("Rotating X with: {}rad", angle);
        let x = Vector4::new_from_array([[self.middle_point[0][0] + self.x_half_size], [0.0], [0.0], [0.0]]);
        let x_norm = x.normalize();
        self.orientation = &self.orientation * &self.get_rotation_matrix(&x_norm, angle);
    }
    pub fn rotate_y(&mut self, angle: Scalar) {
        println!("Rotating Y with: {}rad", angle);
        let y = Vector4::new_from_array([[0.0], [self.middle_point[1][0] + self.y_half_size], [0.0], [0.0]]);
        let y_norm = y.normalize();
        self.orientation = &self.orientation * &self.get_rotation_matrix(&y_norm, angle);
    }
    pub fn rotate_z(&mut self, angle: Scalar) {
        println!("Rotating Z with: {}rad", angle);
        let z = Vector4::new_from_array([[0.0], [0.0], [self.middle_point[2][0] + self.z_half_size], [0.0]]);
        let z_norm = z.normalize();
        self.orientation = &self.orientation * &self.get_rotation_matrix(&z_norm, angle);
    }

    //pub fn get_rotation_angle(&self) -> Scalar {
    //    self.rotation_angle
    //}
}
