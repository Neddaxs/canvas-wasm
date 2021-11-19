use super::super::shaders::{fragment, vertex};
use super::super::utils;
use js_sys::WebAssembly;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::WebGlRenderingContext as GL;
use web_sys::*;

pub struct Color2D {
    program: WebGlProgram,
    rect_vertices_buffer: WebGlBuffer,
    rect_vertices_length: usize,
    u_color: WebGlUniformLocation,
    u_opacity: WebGlUniformLocation,
    u_transform: WebGlUniformLocation,
}

impl Color2D {
    pub fn new(gl: &WebGlRenderingContext) -> Self {
        let program =
            utils::link_program(&gl, vertex::color_2d::SHADER, fragment::color_2d::SHADER).unwrap();

        // x, y
        let vertices: [f32; 12] = [0., 1., 0., 0., 1., 1., 1., 1., 0., 0., 1., 0.];

        let memory_buffer = wasm_bindgen::memory()
            .dyn_into::<WebAssembly::Memory>()
            .unwrap()
            .buffer();

        let vertices_location = vertices.as_ptr() as u32 / 4;
        let vert_array = js_sys::Float32Array::new(&memory_buffer)
            .subarray(vertices_location, vertices.len() as u32);

        let rect_vertices_buffer = gl.create_buffer().ok_or("failed to create buffer").unwrap();

        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&rect_vertices_buffer));
        gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &vert_array, GL::STATIC_DRAW);

        Self {
            u_color: gl.get_uniform_location(&program, "uColor").unwrap(),
            u_opacity: gl.get_uniform_location(&program, "uOpacity").unwrap(),
            u_transform: gl.get_uniform_location(&program, "uTransform").unwrap(),
            rect_vertices_length: vertices.len(),
            rect_vertices_buffer,
            program,
        }
    }

    pub fn render(
        &self,
        gl: &WebGlRenderingContext,
        bottom: f32,
        top: f32,
        left: f32,
        right: f32,
        canvas_height: f32,
        canvas_width: f32,
    ) {
        gl.use_program(Some(&self.program));

        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.rect_vertices_buffer));
        gl.vertex_attrib_pointer_with_i32(0, 2, GL::FLOAT, false, 0, 0);
        gl.enable_vertex_attrib_array(0);

        gl.uniform4f(Some(&self.u_color), 0., 0.5, 0.5, 1.0);

        gl.uniform1f(Some(&self.u_opacity), 1.);

        let translation_matrix = utils::translation_matrix(
            2. * left / canvas_width - 1.,
            2. * bottom / canvas_height - 1.,
            0.,
        );

        let scale_mat = utils::scaling_matrix(
            2. * (right - left) / canvas_width,
            2. * (top - bottom) / canvas_height,
            0.,
        );

        let transform_mat = utils::mult_matrix_4(scale_mat, translation_matrix);

        gl.uniform_matrix4fv_with_f32_array(Some(&self.u_transform), false, &transform_mat);

        gl.draw_arrays(GL::TRIANGLES, 0, (self.rect_vertices_length / 2) as i32);
    }
}
