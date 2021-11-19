use wasm_bindgen::{JsCast, JsValue};
use web_sys::WebGlRenderingContext as GL;
use web_sys::*;

pub fn initialize_web_gl_context() -> Result<WebGlRenderingContext, JsValue> {
    let window = window().unwrap();
    let document = window.document().unwrap();

    let canvas = document.get_element_by_id("rustCanvas").unwrap();
    let canvas: HtmlCanvasElement = canvas.dyn_into::<HtmlCanvasElement>().unwrap();

    let gl: WebGlRenderingContext = canvas
        .get_context("webgl")
        .unwrap()
        .unwrap()
        .dyn_into::<WebGlRenderingContext>()
        .unwrap();

    gl.enable(GL::BLEND);
    gl.blend_func(GL::SRC_ALPHA, GL::ONE_MINUS_SRC_ALPHA);
    gl.clear_color(0.0, 0.0, 0.0, 1.0);
    gl.clear_depth(1.0);

    Ok(gl)
}
