use std::{cell::RefCell, rc::Rc, thread::sleep, time::Duration};

use wasm_bindgen::{prelude::Closure, JsCast};

mod events;
mod game_state;
mod init;
mod renderer;
mod state;
mod utils;

extern crate web_sys;

const SLEEPER_TIMEOUT: u64 = 1;

pub fn run(root_id: &str) {
    utils::logger::info("snake running!");

    match init::InitData::new(root_id) {
        Ok(init_data) => {
            utils::logger::info("Successfully initialized data!");

            let mut game_state = game_state::State::new(None);
            let init_data_ref = Rc::new(RefCell::new(init_data));

            events::listeners::register(&init_data_ref, &game_state);

            renderer::render(&init_data_ref, &game_state);

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
