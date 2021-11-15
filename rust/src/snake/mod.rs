use std::{cell::RefCell, rc::Rc, thread::sleep, time::Duration};

use wasm_bindgen::{prelude::Closure, JsCast};

mod game_state;
mod init;
mod renderer;
mod state;
mod utils;

extern crate web_sys;

const SLEEPER_TIMEOUT: u64 = 1;

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

    /*
    std::thread::spawn(move || loop {
        utils::logger::error(&format!("Error: {:?}", ""))
    });
    */

    match init::InitData::new(root_id) {
        Ok(init_data) => {
            utils::logger::info("Successfully initialized data!");

            let game_state = game_state::State::new(None);

            let boxed_ctx = Box::new(init_data.ctx);

            let state_ref = Rc::new(RefCell::new(state::State::new(boxed_ctx)));

            renderer::render(&init_data, &game_state);

            // KeyDown
            {
                let cloned_state_ref = state_ref.clone();

                let key_press_callback =
                    Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
                        utils::logger::info("keydown");
                        event.prevent_default();

                        let mut s = cloned_state_ref.borrow_mut();
                        let key = utils::keys::get_key(event.key().as_str());
                        utils::logger::info(&format!("{:?}", key));
                        s.change_direction(key);
                    }) as Box<dyn FnMut(_)>);

                match init_data.window.add_event_listener_with_callback(
                    "keydown",
                    key_press_callback.as_ref().unchecked_ref(),
                ) {
                    Err(e) => utils::logger::error(&format!("Error: {:?}", e)),
                    Ok(_) => key_press_callback.forget(),
                }
            }

            // OnClick
            {
                let cloned_state_ref = state_ref.clone();

                let onclick_callback = Closure::wrap(Box::new(move |_event: web_sys::Event| {
                    utils::logger::info("click");
                    let mut s = cloned_state_ref.borrow_mut();
                    s.toggle_game();
                }) as Box<dyn FnMut(_)>);

                match init_data.canvas.add_event_listener_with_callback(
                    "click",
                    onclick_callback.as_ref().unchecked_ref(),
                ) {
                    Err(e) => utils::logger::error(&format!("Error: {:?}", e)),
                    Ok(_) => onclick_callback.forget(),
                }
            }

            // Renderer
            /*
            {
                let cloned_state = state_ref.clone();
                let mut s = cloned_state.borrow_mut();
                while s.active {
                    utils::logger::info("Render");
                    s.render();
                    sleep(Duration::new(SLEEPER_TIMEOUT, 0));
                }
            }
            */
        }
        Err(e) => utils::logger::error(&format!("Error: {:?}", e)),
    }
}
