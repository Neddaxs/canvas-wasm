use std::{
    cell::{RefCell, RefMut},
    rc::Rc,
};

use wasm_bindgen::{prelude::Closure, JsCast, JsValue};

use super::{game_state, init};

extern crate libm;
extern crate web_sys;

const GRASS_COLOR: &str = "#348C31";
const SNAKE_COLOR: &str = "#b6b428";
const APPLE_COLOR: &str = "#C7372F";

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(f: &Closure<dyn FnMut(f64)>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

pub fn render(init_data: &mut RefMut<init::InitData>, state: &mut RefMut<game_state::State>) {
    let ctx = &init_data.ctx;
    ctx.set_font("30px Arial");

    match state.running_state {
        game_state::RunningState::RUNNING => {
            state.move_snake();
            let tile_width = game_state::tile_size() as f64;
            // fix this to use proper scaling
            let tile_size = tile_width * 1 as f64;

            for tile in state.board() {
                match tile.state {
                    game_state::Tile::SNAKE => {
                        ctx.set_fill_style(&JsValue::from_str(SNAKE_COLOR));
                    }
                    game_state::Tile::EMPTY => {
                        ctx.set_fill_style(&JsValue::from_str(GRASS_COLOR));
                    }
                    game_state::Tile::APPLE => {
                        ctx.set_fill_style(&JsValue::from_str(APPLE_COLOR));
                    }
                }

                ctx.fill_rect(
                    (tile.col as f64) * tile_size,
                    (tile.row as f64) * tile_size,
                    tile_size,
                    tile_size,
                );
            }

            ctx.stroke_text(&format!("{:?}", state.apples_collected)[..], 100.0, 100.0)
                .ok();
        }
        game_state::RunningState::DIED => {
            ctx.stroke_text("DIED", 100.0, 100.0).ok();
        }
        game_state::RunningState::IDLE => {
            ctx.stroke_text("IDLE", 100.0, 100.0).ok();
        }
        game_state::RunningState::PAUSED => {
            ctx.stroke_text("PAUSED", 100.0, 100.0).ok();
        }
    }
}

pub fn handle_renders(
    init_data_ref: &Rc<RefCell<init::InitData>>,
    game_state_ref: &Rc<RefCell<game_state::State>>,
) {
    let init_data_ref_clone = init_data_ref.clone();
    let game_state_ref_clone = game_state_ref.clone();

    render(
        &mut init_data_ref.borrow_mut(),
        &mut game_state_ref.borrow_mut(),
    );

    let mut previous_timestamp: Option<f64> = None;
    let mut frame: f64 = -1.0;
    let raf_callback = Rc::new(RefCell::new(None));

    let raf_callback_clone = raf_callback.clone();

    *raf_callback_clone.borrow_mut() = Some(Closure::wrap(Box::new(move |timestamp: f64| {
        let mut game_data = game_state_ref_clone.borrow_mut();
        let mut init_data = init_data_ref_clone.borrow_mut();

        let delay = 1000.0 / game_data.fps;

        match previous_timestamp {
            Some(value) => {
                let calculated_frame = libm::floor((timestamp - value) / delay);

                if calculated_frame > frame {
                    frame = calculated_frame;
                    render(&mut init_data, &mut game_data);
                }
            }
            _ => {
                previous_timestamp = Some(timestamp);
            }
        }

        request_animation_frame(raf_callback.borrow().as_ref().unwrap())
    }) as Box<dyn FnMut(f64)>));

    request_animation_frame(raf_callback_clone.borrow().as_ref().unwrap());
}
