use std::{cell::RefCell, rc::Rc};

mod events;
mod game_state;
mod init;
mod renderer;
mod utils;

extern crate web_sys;

pub fn run(root_id: &str) {
    utils::logger::info("snake running!");

    match init::InitData::new(root_id) {
        Ok(init_data) => {
            init_data.canvas.focus().ok();
            utils::logger::info("Successfully initialized data!");

            let game_state = Rc::new(RefCell::new(game_state::State::new(None)));
            let init_data_ref = Rc::new(RefCell::new(init_data));

            events::listeners::register(&init_data_ref, &game_state);
            renderer::handle_renders(&init_data_ref, &game_state);
        }
        Err(e) => utils::logger::error(&format!("Error: {:?}", e)),
    }
}
