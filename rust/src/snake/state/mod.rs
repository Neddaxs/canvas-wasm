use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

use crate::snake::utils::keys::KeyValue;

pub enum Directions {
    Up,
    Down,
    Left,
    Right,
}

pub struct Location {
    x: f64,
    y: f64,
}

pub struct State {
    pub ctx: Box<CanvasRenderingContext2d>,
    pub active: bool,
    requires_rerender: bool,
    size: f64,
    location: Location,
    direction: Directions,
}

impl State {
    pub fn new(ctx: Box<CanvasRenderingContext2d>) -> State {
        State {
            ctx,
            size: 1.0,
            location: Location { x: 0.0, y: 0.0 },
            direction: Directions::Up,
            active: false,
            requires_rerender: false,
        }
    }

    fn set_location(&mut self, x: f64, y: f64) {
        self.location = Location { x, y };
    }
}
