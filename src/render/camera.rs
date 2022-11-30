struct Camera {
    pub fov: f32,
    pub aspect: f32,
    pub near_plane: f32,
    pub far_plane: f32,
    pub projection_matrix: mat4
}

impl Camera {
    pub fn set_fov(&mut self, fov: f32) {
        self.fov = fov;
        self.update_projection_matrix();
    }

    pub fn update_projection_matrix(&mut self) {
        let _fov: f32 = 2.0 * ((self.fov.to_radians() / 2.0).tan() * self.aspect).atan();
        self.projection_matrix = gl::Perspective(_fov, self.aspect, self.near_plane, self.far_plane);
    }
}