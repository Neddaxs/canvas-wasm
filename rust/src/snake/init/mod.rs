mod error;

use wasm_bindgen::{JsCast, JsValue};

pub struct Location {
    pub x: i32,
    pub y: i32,
}

pub struct InitData {
    pub window: web_sys::Window,
    pub document: web_sys::Document,
    pub root: web_sys::HtmlDivElement,
    pub canvas: web_sys::HtmlCanvasElement,
    pub ctx: web_sys::CanvasRenderingContext2d,
    pub aspect: f64,
    pub location: Option<Location>,
}

impl InitData {
    pub fn new(root_id: &str) -> Result<InitData, error::InitError> {
        let window = web_sys::window().ok_or_else(|| error::InitError::GetWindowError)?;

        let document = window.document().ok_or_else(|| {
            return error::InitError::GetDocumentError;
        })?;

        let root = match document
            .get_element_by_id(root_id)
            .unwrap()
            .dyn_into::<web_sys::HtmlDivElement>()
        {
            Ok(root) => root,
            Err(_) => return Err(error::InitError::GetRootError),
        };

        let canvas = match document
            .create_element("canvas")
            .unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>()
        {
            Ok(canvas) => canvas,
            Err(_) => return Err(error::InitError::CreateCanvasError),
        };

        let ctx = match canvas
            .get_context("2d")
            .unwrap()
            .ok_or_else(|| return error::InitError::GetContextError)
        {
            Ok(ctx_before_cast) => {
                match ctx_before_cast.dyn_into::<web_sys::CanvasRenderingContext2d>() {
                    Ok(ctx) => ctx,
                    Err(_) => return Err(error::InitError::GetContextError),
                }
            }
            Err(_) => return Err(error::InitError::GetContextError),
        };

        match root.append_child(&canvas) {
            Err(_) => return Err(error::InitError::AppendCanvasError),
            _ => {}
        }

        let mut init_data = InitData {
            window,
            document,
            root,
            canvas,
            ctx,
            aspect: 0.0,
            location: None,
        };

        match init_data.resize_canvas() {
            Ok(_) => return Ok(init_data),
            Err(_) => return Err(error::InitError::ScaleCanvasError),
        };
    }

    pub fn resize_canvas(&mut self) -> Result<(), JsValue> {
        let width: u32 = self.root.offset_width().try_into().unwrap();
        let height: u32 = self.root.offset_height().try_into().unwrap();

        let size = if width > height {
            let offset = (width - height) / 2;
            self.canvas
                .set_attribute(
                    "style",
                    &create_canvas_style(&format!("{}px", offset), "0px"),
                )
                .unwrap();
            height
        } else {
            let offset = (height - width) / 2;
            self.canvas
                .set_attribute(
                    "style",
                    &create_canvas_style("0px", &format!("{}px", offset)),
                )
                .unwrap();
            width
        };

        self.canvas.set_width(size);
        self.canvas.set_height(size);
        self.update_aspect();
        Ok(())
    }

    fn update_aspect(&mut self) {
        self.aspect = if self.canvas.offset_width() < 1 {
            1.0
        } else {
            (self.canvas.offset_width() as f64) / 100.0
        }
    }

    pub fn mouse_down(&mut self, x: i32, y: i32) {
        self.location = Some(Location { x, y });
    }

    pub fn mouse_up(&mut self) {
        self.location = None;
    }
}

fn create_canvas_style(left: &str, top: &str) -> String {
    format!("position: absolute; left: {}; top: {};", left, top)
}
