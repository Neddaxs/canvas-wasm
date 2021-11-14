use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

pub struct State {
    pub clicks: i32,
    pub grow: bool,
    aspect: i32,
    pub ctx: Box<CanvasRenderingContext2d>,
}

impl State {
    pub fn new(ctx: Box<CanvasRenderingContext2d>) -> State {
        State {
            clicks: 0,
            aspect: 0,
            grow: true,
            ctx,
        }
    }

    pub fn render(&mut self) {
        let ctx = self.ctx.as_mut();
        ctx.set_fill_style(&JsValue::from_str("red"));
        let offset = self.clicks as f64;
        ctx.fill_rect(10.0 + offset, 10.0 + offset, 100.0, 100.0);
    }

    pub fn click(&mut self) {
        self.clicks += 1;
    }

    pub fn get_aspect(&self) -> i32 {
        self.aspect
    }
}
