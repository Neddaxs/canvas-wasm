mod error;

use wasm_bindgen::JsCast;

pub struct InitData {
    pub window: web_sys::Window,
    pub document: web_sys::Document,
    pub root: web_sys::HtmlDivElement,
    pub canvas: web_sys::HtmlCanvasElement,
    pub ctx: web_sys::CanvasRenderingContext2d,
    pub aspect: i32,
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

        canvas.set_width(root.offset_width().try_into().unwrap());
        canvas.set_height(root.offset_height().try_into().unwrap());
        canvas.set_tab_index(1);

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
            aspect: 0,
        };

        init_data.update_aspect();

        Ok(init_data)
    }

    fn update_aspect(&mut self) {
        self.aspect = if self.canvas.offset_width() == 0 {
            1
        } else {
            self.canvas.offset_width() / 100
        }
    }
}
