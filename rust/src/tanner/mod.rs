use wasm_bindgen::prelude::wasm_bindgen;

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
use web_sys::WebGl2RenderingContext as GL;
use web_sys::*;

mod gl_setup;
mod programs;
mod shaders;
mod utils;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub struct TannerClient {
    gl: WebGl2RenderingContext,
    indigo_code: programs::indigo_code::IndigoCode,
}

#[wasm_bindgen(constructor)]
impl TannerClient {
    pub fn new() -> Self {
        console_error_panic_hook::set_once();
        log("new client initialized");
        let gl = gl_setup::initialize_web_gl_context().unwrap();
        let indigo_code = programs::indigo_code::IndigoCode::new(&gl);
        Self { indigo_code, gl }
    }

    pub fn update(&mut self, _time: f32, _height: f32, _width: f32) -> Result<(), JsValue> {
        Ok(())
    }

    pub fn render(&self) {
        self.gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);
        self.indigo_code.render(&self.gl);
    }
}
