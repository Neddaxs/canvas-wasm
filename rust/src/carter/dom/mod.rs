use wasm_bindgen::{JsCast, JsValue};
use web_sys::{window, Document, HtmlCanvasElement, HtmlDivElement, WebGlRenderingContext, Window};

pub struct Dom {
    pub window: Window,
    pub document: Document,
    pub div: HtmlDivElement,
    pub canvas: HtmlCanvasElement,
}

impl Dom {
    pub fn new(
        window: Window,
        document: Document,
        div: HtmlDivElement,
        canvas: HtmlCanvasElement,
    ) -> Dom {
        Dom {
            window,
            document,
            div,
            canvas,
        }
    }
}

pub fn setup(root_id: &str) -> Result<(Dom, WebGlRenderingContext), JsValue> {
    let window = window().unwrap();
    let document = window.document().unwrap();

    let root: HtmlDivElement = document
        .get_element_by_id(root_id)
        .unwrap()
        .dyn_into()
        .unwrap();

    let canvas: HtmlCanvasElement = document
        .create_element("canvas")
        .unwrap()
        .dyn_into()
        .unwrap();

    root.append_child(&canvas).unwrap();

    let gl: WebGlRenderingContext = canvas.get_context("webgl")?.unwrap().dyn_into().unwrap();

    gl.enable(WebGlRenderingContext::BLEND);
    gl.blend_func(
        WebGlRenderingContext::SRC_ALPHA,
        WebGlRenderingContext::ONE_MINUS_SRC_ALPHA,
    );
    gl.clear_color(0.0, 0.0, 0.1, 1.0);
    gl.clear_depth(1.0);

    Ok((Dom::new(window, document, root, canvas), gl))
}
