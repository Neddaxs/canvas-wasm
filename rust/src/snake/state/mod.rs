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

    pub fn render(&mut self) {
        match self.requires_rerender {
            true => {
                let ctx = self.ctx.as_mut();
                ctx.set_fill_style(&JsValue::from_str("red"));
                // Width is constantly 1, height changes based on amount eaten.
                ctx.fill_rect(self.location.x, self.location.y, 1.0, self.size);
            }
            false => {}
        }
    }

    pub fn toggle_game(&mut self) {
        self.active = !self.active;
    }

    pub fn change_direction(&mut self, key: KeyValue) {
        let direction = match key {
            KeyValue::DownArrow => Some(Directions::Down),
            KeyValue::LeftArrow => Some(Directions::Left),
            KeyValue::RightArrow => Some(Directions::Right),
            KeyValue::UpArrow => Some(Directions::Up),
            _ => None,
        };
        match direction {
            Some(val) => {
                self.direction = val;
                self.made_changes();
            }
            None => {}
        }
    }

    fn made_changes(&mut self) {
        self.requires_rerender = true;
    }
}
