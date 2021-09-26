pub struct DisplayState {
    pub window_width: u32,
    pub window_height: u32,
    pub is_enabled_depth_test: bool,
    pub is_enabled_blend: bool,
    pub is_enabled_wireframe: bool,
    pub is_enabled_culling: bool,
}

impl DisplayState {
    pub fn new(window_size_wh: (u32, u32)) -> DisplayState {
        DisplayState {
            window_width: window_size_wh.0,
            window_height: window_size_wh.1,
            is_enabled_depth_test: false,
            is_enabled_blend: false,
            is_enabled_wireframe: false,
            is_enabled_culling: false,
        }
    }
    pub fn set_window_size(&mut self,window_size:&[f32]){
        self.window_width=window_size[0]as u32;
        self.window_height=window_size[1]as u32;
    }
    pub unsafe fn setup(&self) {
        if self.is_enabled_depth_test {
            gl::Enable(gl::DEPTH_TEST);
        } else {
            gl::Disable(gl::DEPTH_TEST);
        }

        if self.is_enabled_blend {
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        } else {
            gl::Disable(gl::BLEND);
        }

        if self.is_enabled_wireframe {
            gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
        } else {
            gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
        }

        if self.is_enabled_culling {
            gl::Enable(gl::CULL_FACE);
        } else {
            gl::Disable(gl::CULL_FACE);
        }
    }
}
