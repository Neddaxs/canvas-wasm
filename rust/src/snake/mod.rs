use std::{cell::RefCell, rc::Rc, thread::sleep, time::Duration};

use wasm_bindgen::{prelude::Closure, JsCast, JsValue};

mod init;
mod state;
mod utils;

extern crate web_sys;

pub fn run(root_id: &str) {
    utils::logger::info("snake running!");

    match init::InitData::new(root_id) {
        Ok(init_data) => {
            utils::logger::info("Successfully initialized data!");

            init_data.ctx.set_fill_style(&JsValue::from_str("orange"));
            init_data.ctx.fill_rect(19.0, 20.0, 300.0, 100.0);

            let boxed_ctx = Box::new(init_data.ctx);

            let state_ref = Rc::new(RefCell::new(state::State::new(boxed_ctx)));

            {
                let cloned_state = state_ref.clone();

                let callback = Closure::wrap(Box::new(move |_event: web_sys::Event| {
                    let mut s = cloned_state.borrow_mut();
                    s.grow = !s.grow;
                    s.click();
                    s.render();
                }) as Box<dyn FnMut(_)>);

                match init_data
                    .canvas
                    .add_event_listener_with_callback("click", callback.as_ref().unchecked_ref())
                {
                    Err(e) => utils::logger::error(&format!("Error: {:?}", e)),
                    Ok(_) => callback.forget(),
                }
            }

            {
                utils::logger::info(&format!("A: {}", state_ref.borrow().clicks));

                for _ in 0..50 {
                    let mut s = state_ref.borrow_mut();
                    utils::logger::info(&format!("Loop: {}", s.clicks));
                    s.click();
                    s.render();
                    // sleep(Duration::from_secs(1));
                }
            }
        }
        Err(e) => utils::logger::error(&format!("Error: {:?}", e)),
    }
}
