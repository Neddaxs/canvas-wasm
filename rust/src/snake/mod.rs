use std::{cell::RefCell, rc::Rc};

use wasm_bindgen::{prelude::Closure, JsCast};

mod init;
mod state;
mod utils;

extern crate web_sys;

pub fn run(root_id: &str) {
    utils::logger::info("snake running!");

    //  let state
    //
    //  state {
    //      direction,
    //      state: MOVING | IDLE
    //      x,
    //      y
    //  }
    //
    //  eventHandlers
    //  -> state
    //
    //  loop {
    //      update -> state
    //      render()
    //  }

    std::thread::spawn(move || loop {
        utils::logger::error(&format!("Error: {:?}", ""))
    });

    match init::InitData::new(root_id) {
        Ok(init_data) => {
            utils::logger::info("Successfully initialized data!");

            let boxed_ctx = Box::new(init_data.ctx);
            let state_ref = Rc::new(RefCell::new(state::State::new(boxed_ctx)));

            {
                let cloned_state_ref = state_ref.clone();

                let callback = Closure::wrap(Box::new(move |_event: web_sys::Event| {
                    let mut s = cloned_state_ref.borrow_mut();
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
                    // std::thread::sleep(Duration::from_secs(1));
                }
            }
        }
        Err(e) => utils::logger::error(&format!("Error: {:?}", e)),
    }
}
