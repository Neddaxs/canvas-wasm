use std::{cell::RefCell, rc::Rc, thread::sleep, time::Duration};

use wasm_bindgen::{prelude::Closure, JsCast};

mod events;
mod game_state;
mod init;
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

            let _game_state = game_state::State::new(None);

            let boxed_ctx = Box::new(init_data.ctx);
            let state_ref = Rc::new(RefCell::new(state::State::new(boxed_ctx)));

            let boxed_canvas = Box::new(init_data.canvas);
            let canvas_ref = Rc::new(RefCell::new(boxed_canvas));

            let root = Box::new(init_data.root);
            let root_ref = Rc::new(RefCell::new(root));

            events::listeners::register(&canvas_ref, &state_ref, &root_ref);

            // Renderer
            {
                let cloned_state = state_ref.clone();
                let mut s = cloned_state.borrow_mut();
                while s.active {
                    utils::logger::info("Render");
                    s.render();
                    sleep(Duration::new(SLEEPER_TIMEOUT, 0));
                }
            }
        }
        Err(e) => utils::logger::error(&format!("Error: {:?}", e)),
    }
}
