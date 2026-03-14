#[derive(Debug, Clone, PartialEq)]
pub struct Camera {
    pub fov_degrees: f32,
    pub near: f32,
    pub far: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            fov_degrees: 60.0,
            near: 0.1,
            far: 1000.0,
        }
    }
}
