#[allow(dead_code)]
type Point3 = cgmath::Point3<f32>;
#[allow(dead_code)]
type Vector3 = cgmath::Vector3<f32>;
#[allow(dead_code)]
type Matrix4 = cgmath::Matrix4<f32>;

use cgmath::perspective;

pub struct CameraState {
    aspect_ratio: f32,
    pub position: (f32, f32, f32),
    direction: (f32, f32, f32),
}

impl CameraState {
    pub fn new(window_size_wh: (u32, u32)) -> CameraState {
        CameraState {
            aspect_ratio: window_size_wh.0 as f32 / window_size_wh.1 as f32,
            position: (5.0, -5.0, 5.0),
            direction: (0.0, 1.0, 0.0),
        }
    }

    pub fn set_position(&mut self, pos: (f32, f32, f32)) {
        self.position = pos;
    }

    pub fn set_direction(&mut self, dir: (f32, f32, f32)) {
        self.direction = dir;
    }

    pub fn get_perspective(&self) -> Matrix4 {
        let zfar = 1024.0;
        let znear = 0.1;
        let projection_matrix: Matrix4 =
            perspective(cgmath::Deg(45.0f32), self.aspect_ratio, znear, zfar);
        projection_matrix
    }
}
