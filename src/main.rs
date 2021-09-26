use std::mem;
use std::os::raw::c_void;
use std::process;

use c_str_macro::c_str;
use cgmath::prelude::SquareMatrix;
use chrono::Local;

use gl::types::{GLfloat, GLsizei, GLsizeiptr};
// use gl::UNSIGNED_INT_IMAGE_1D;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
// use sdl2::video;

#[allow(dead_code)]
type Point3 = cgmath::Point3<f32>;
#[allow(dead_code)]
type Vector3 = cgmath::Vector3<f32>;
#[allow(dead_code)]
type Matrix4 = cgmath::Matrix4<f32>;

use imgui::im_str;
pub mod camera;
pub mod display;
pub mod model;
pub mod shader;
pub mod vertex;
use camera::CameraState;
use display::DisplayState;
use model::Model;
use shader::Shader;
use vertex::Vertex;
const INIT_WINDOW_WIDTH: u32 = 1200;
const INIT_WINDOW_HEIGHT: u32 = 1080;
const FLOAT_NUM: usize = 3;
// const VERTEX_NUM: usize = 4;
// const BUF_LEN: usize = FLOAT_NUM * VERTEX_NUM;

fn get_current_time() -> String {
    let dt = Local::now();
    let str_ = dt.format("%Y-%m-%d %H:%M:%S").to_string();
    str_
}
fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    println!("{} OK: init sdl2 video sub system", get_current_time());

    // set up video subsystem...
    {
        let gl_attr = video_subsystem.gl_attr();
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(3, 1);
        let (major, minor) = gl_attr.context_version();
        println!(
            "{} OK: init OpenGL: version={}.{}",
            get_current_time(),
            major,
            minor
        );
    }

    // create window
    let window = video_subsystem
        .window("title", INIT_WINDOW_WIDTH, INIT_WINDOW_HEIGHT)
        .opengl()
        .position_centered()
        .resizable()
        .build()
        .unwrap();
    println!("{} OK: build window", get_current_time());

    // set up opengl context
    let _gl_context = window.gl_create_context().unwrap();
    gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as _);
    println!("{} OK: create opengl context", get_current_time());

    // set up shaders
    let mut shader = Shader::new();
    shader.setup("rsc/shader/mono_shader.vs", "rsc/shader/mono_shader.fs");

    #[rustfmt::skip]
    let axis_array:[f32;12]=[
        0.0,0.0,0.0,
        1.0,0.0,0.0,
        0.0,1.0,0.0,
        0.0,0.0,1.0,
    ];

    #[rustfmt::skip]
    let axis_indices: [i32; 6] = [
        0, 1,
        0, 2,
        0, 3,
    ];
    let mut model = Model::new();
    // /home/twmoca/Downloads/IronMan/IronMan.obj
    // /home/twmoca/Documents/3d_obj/BaseSpiderMan.obj
    // /home/twmoca/Downloads/uploads_files_2663635_Audi_RS_6_Avant.obj
    // /home/twmoca/Downloads/RangeRoverSport/2016_Custom_Range_Rover_Sport.obj
    // /home/twmoca/Documents/3d_obj/Enzo/Enzo.obj
    if let Err(e) =
        model.load("/home/twmoca/Documents/3d_obj/teapot.obj")
    {
        println!("Model error: {}", e);
        process::exit(1);
    }
    model.create_vertex_normal();
    let vertex_buf = model.create_concat_vertex();

    let buf_len = vertex_buf.len();
    let vertex_num = model.vertex.len() / (FLOAT_NUM * 2);
    println!("buf_len: {}", buf_len);
    println!("vertex_num: {}", vertex_num);
    
    let mut vertex = Vertex::new(
        (vertex_buf.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
        vertex_buf.as_ptr() as *const c_void,
        gl::STATIC_DRAW,
        vec![gl::FLOAT, gl::FLOAT],
        vec![FLOAT_NUM as i32, FLOAT_NUM as i32],
        (FLOAT_NUM + FLOAT_NUM) as i32 * mem::size_of::<GLfloat>() as GLsizei,
        vertex_num as i32,
    );
    vertex.setup_ibo(
        model.indices.vertex_indices.len() as GLsizeiptr,
        model.indices.vertex_indices.as_ptr() as *const c_void,
    );
    let mut axis_vertex = Vertex::new(
        (12 * mem::size_of::<GLfloat>()) as GLsizeiptr,
        axis_array.as_ptr() as *const c_void,
        gl::STATIC_DRAW,
        vec![gl::FLOAT],
        vec![FLOAT_NUM as i32],
        FLOAT_NUM as i32 * mem::size_of::<GLfloat>() as GLsizei,
        4 as i32,
    );
    axis_vertex.setup_ibo(6 as GLsizeiptr, axis_indices.as_ptr() as *const c_void);

    // init imgui
    let mut imgui_context = imgui::Context::create();
    imgui_context.set_ini_filename(None);

    // init imgui sdl2
    let mut imgui_sdl2_context = imgui_sdl2::ImguiSdl2::new(&mut imgui_context, &window);
    let renderer = imgui_opengl_renderer::Renderer::new(&mut imgui_context, |s| {
        video_subsystem.gl_get_proc_address(s) as _
    });

    let mut cam_state = CameraState::new((INIT_WINDOW_WIDTH, INIT_WINDOW_HEIGHT));
    let mut display_state=DisplayState::new((INIT_WINDOW_WIDTH,INIT_WINDOW_HEIGHT));
    // let mut alpha: f32 = 1.0f32;
    // let mut material_specular: Vector3 = Vector3 {
    //     x: 0.2,
    //     y: 0.2,
    //     z: 0.2,
    // };
    // let mut material_shininess: f32 = 0.1f32;
    // let mut light_direction: Vector3 = Vector3 {
    //     x: 1.0,
    //     y: 1.0,
    //     z: 0.0,
    // };
    // let mut ambient: Vector3 = Vector3 {
    //     x: 0.3,
    //     y: 0.3,
    //     z: 0.3,
    // };
    // let mut diffuse: Vector3 = Vector3 {
    //     x: 0.5,
    //     y: 0.5,
    //     z: 0.5,
    // };
    // let mut specular: Vector3 = Vector3 {
    //     x: 0.2,
    //     y: 0.2,
    //     z: 0.2,
    // };

    let mut event_pump = sdl_context.event_pump().unwrap();

    // let mut button_down_pos=0;
    'main: loop {
        for event in event_pump.poll_iter() {
            imgui_sdl2_context.handle_event(&mut imgui_context, &event);
            if imgui_sdl2_context.ignore_event(&event) {
                continue;
            }

            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'main,
                _ => {}
            }
        }

        unsafe {
            display_state.setup();

            gl::Viewport(0, 0, display_state.window_width as i32, display_state.window_height as i32);

            // clear screen
            gl::ClearColor(1.0, 1.0, 1.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            // init matrice for model, view and projection
            let model_matrix = Matrix4::identity();
            let view_matrix = Matrix4::look_at(
                Point3 {
                    x: cam_state.position.0,
                    y: cam_state.position.1,
                    z: cam_state.position.2,
                },
                Point3 {
                    x: 0.0,
                    y: 1.0,
                    z: 0.0,
                },
                Vector3 {
                    x: 0.0,
                    y: 1.0,
                    z: 0.0,
                },
            );

            let projection_matrix: Matrix4 = cam_state.get_perspective();

            // shader use matrices
            shader.use_program();
            shader.set_mat4(c_str!("uModel"), &model_matrix);
            shader.set_mat4(c_str!("uView"), &view_matrix);
            shader.set_mat4(c_str!("uProjection"), &projection_matrix);
            shader.set_vec3(c_str!("uViewPosition"), cam_state.position.0, cam_state.position.1, cam_state.position.2);
            // shader.set_float(c_str!("uAlpha"), alpha);
            // shader.set_vector3(c_str!("uMaterial.specular"), &material_specular);
            // shader.set_float(c_str!("uMaterial.shininess"), material_shininess);
            // shader.set_vector3(c_str!("uLight.direction"), &light_direction);
            // shader.set_vector3(c_str!("uLight.ambient"), &ambient);
            // shader.set_vector3(c_str!("uLight.diffuse"), &diffuse);
            // shader.set_vector3(c_str!("uLight.specular"), &specular);

            // vertex.draw();
            vertex.draw_elements(
                gl::TRIANGLES,
                model.indices.vertex_indices.len() as GLsizei,
                model.indices.vertex_indices.as_ptr() as *const c_void,
            );
            // vertex.draw_elements2(&model.indices);
            // axis_vertex.draw_elements(
            //     gl::LINES,
            //     6 as GLsizei,
            //     axis_indices.as_ptr() as *const c_void,
            // );

            imgui_sdl2_context.prepare_frame(
                imgui_context.io_mut(),
                &window,
                &event_pump.mouse_state(),
            );
            let ui = imgui_context.frame();
            imgui::Window::new(im_str!("Information"))
                .size([300.0, 450.0], imgui::Condition::FirstUseEver)
                .position([10.0, 10.0], imgui::Condition::FirstUseEver)
                .build(&ui, || {
                    ui.text(im_str!("OpenGL Test App ver 1.0"));
                    ui.separator();
                    ui.text(im_str!("FPS: {:.1}", ui.io().framerate));
                    let display_size = ui.io().display_size;
                    display_state.set_window_size(&display_size);
                    ui.text(format!(
                        "Display Size: ({:.1}, {:.1})",
                        display_size[0], display_size[1]
                    ));
                    let mouse_pos = ui.io().mouse_pos;
                    ui.text(format!(
                        "Mouse Position: ({:.1}, {:.1})",
                        mouse_pos[0], mouse_pos[1]
                    ));
                    ui.separator();

                    ui.checkbox(im_str!("Depth Test"),&mut display_state.is_enabled_depth_test);
                    ui.checkbox(im_str!("Blend"), &mut display_state.is_enabled_blend);
                    ui.checkbox(im_str!("Wireframe"), &mut display_state.is_enabled_wireframe);
                    ui.checkbox(im_str!("Culling"), &mut display_state.is_enabled_culling);

                    ui.separator();
                    imgui::Slider::new(im_str!("Camera X"))
                        .range(-5.0..=5.0)
                        .build(&ui, &mut cam_state.position.0);

                    imgui::Slider::new(im_str!("Camera Y"))
                        .range(-5.0..=5.0)
                        .build(&ui, &mut cam_state.position.1);

                    imgui::Slider::new(im_str!("Camera Z"))
                        .range(-5.0..=5.0)
                        .build(&ui, &mut cam_state.position.2);
                });
            imgui_sdl2_context.prepare_render(&ui, &window);
            renderer.render(ui);

            window.gl_swap_window();
        }
        // ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60))
    }
}
