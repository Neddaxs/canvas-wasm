use wasm_bindgen::JsCast;
use web_sys::WebGl2RenderingContext as GL;
use web_sys::*;

mod shaders;
use crate::tanner::utils::log;

pub struct IndigoCode {
    program: WebGlProgram,
}

impl IndigoCode {
    pub fn new(gl: &WebGl2RenderingContext) -> Self {
        gl.clear_color(0.75, 0.85, 0.8, 1.0);
        gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);

        let vertex_shader = gl
            .create_shader(GL::VERTEX_SHADER)
            .ok_or_else(|| {
                log("Error creating vertex_shader");
                return;
            })
            .unwrap();

        let fragment_shader = gl
            .create_shader(GL::FRAGMENT_SHADER)
            .ok_or_else(|| {
                log("Error creating fragment_shader");
                return;
            })
            .unwrap();

        gl.shader_source(&vertex_shader, shaders::VERTEX);
        gl.shader_source(&fragment_shader, shaders::FRAGMENT);

        gl.compile_shader(&vertex_shader);

        if gl
            .get_shader_parameter(&vertex_shader, GL::COMPILE_STATUS)
            .is_falsy()
        {
            let err = &format!(
                "Error compiling vertex shader: {}",
                gl.get_shader_info_log(&vertex_shader).unwrap()
            )[..];
            log(err);
        }

        gl.compile_shader(&fragment_shader);

        if gl
            .get_shader_parameter(&fragment_shader, GL::COMPILE_STATUS)
            .is_falsy()
        {
            let err = &format!(
                "Error compiling fragment shader: {}",
                gl.get_shader_info_log(&fragment_shader).unwrap()
            )[..];
            log(err);
        }

        let program = gl.create_program().unwrap();

        gl.attach_shader(&program, &vertex_shader);
        gl.attach_shader(&program, &fragment_shader);

        gl.link_program(&program);
        gl.validate_program(&program);

        if gl
            .get_program_parameter(&program, GL::LINK_STATUS)
            .is_falsy()
        {
            let err = &format!(
                "Error linking program: {}",
                gl.get_program_info_log(&program).unwrap()
            )[..];
            log(err);
        }

        // X, Y,       R, G, B
        let traingle_vertices: [f32; 15] = [
            0.0, 0.5, 1.0, 1.0, 0.0, -0.5, -0.5, 0.7, 0.0, 1.0, 0.5, -0.5, 0.1, 1.0, 0.6,
        ];

        let memory_buffer = wasm_bindgen::memory()
            .dyn_into::<js_sys::WebAssembly::Memory>()
            .unwrap()
            .buffer();
        let vertices_location = traingle_vertices.as_ptr() as u32 / 4;
        let vert_array = js_sys::Float32Array::new(&memory_buffer)
            .subarray(vertices_location, traingle_vertices.len() as u32);

        let triangle_vertex_buffer = gl.create_buffer().unwrap();
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&triangle_vertex_buffer));
        gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &vert_array, GL::STATIC_DRAW);

        let vert_location = gl.get_attrib_location(&program, "vertPosition");
        let color_location = gl.get_attrib_location(&program, "vertColor");

        gl.vertex_attrib_pointer_with_i32(vert_location as u32, 2, GL::FLOAT, false, 0, 0);
        gl.vertex_attrib_pointer_with_i32(color_location as u32, 3, GL::FLOAT, false, 0, 0);

        gl.enable_vertex_attrib_array(vert_location as u32);
        gl.enable_vertex_attrib_array(color_location as u32);

        gl.use_program(Some(&program));
        gl.draw_arrays(GL::TRIANGLES, 0, 3);
        Self { program }
    }

    pub fn render(&self, gl: &WebGl2RenderingContext) {}
}
