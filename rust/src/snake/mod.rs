use std::{cell::RefCell, rc::Rc, thread::sleep, time::Duration};

mod events;
mod game_state;
mod init;
mod renderer;
mod state;
mod utils;

extern crate web_sys;

pub fn run(root_id: &str) {
    utils::logger::info("snake running!");

    match init::InitData::new(root_id) {
        Ok(init_data) => {
            utils::logger::info("Successfully initialized data!");

            let game_state = Rc::new(RefCell::new(game_state::State::new(None)));
            let init_data_ref = Rc::new(RefCell::new(init_data));

            events::listeners::register(&init_data_ref, &game_state);

            // Renderer

            loop {
                let mut game = game_state.borrow_mut();
                match game.move_snake() {
                    Ok(_) => {
                        renderer::render(&init_data_ref, &game);
                        // sleep(Duration::from_secs(1));
                    }
                    Err(e) => {
                        utils::logger::error(&format!("Error: {:?}", e));
                        break;
                    }
                }

                break;
            }
        }
        Err(e) => utils::logger::error(&format!("Error: {:?}", e)),
    }
}
