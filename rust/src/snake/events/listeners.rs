use std::{
    cell::{RefCell, RefMut},
    rc::Rc,
};
use wasm_bindgen::{prelude::Closure, JsCast};

use crate::snake::{
    game_state, init,
    renderer::render,
    utils::{
        keys::{get_key, KeyValue},
        logger,
    },
};

extern crate web_sys;

fn keyboard_handler(game: &mut RefMut<game_state::State>, event: web_sys::KeyboardEvent) {
    logger::info("keydown");

    let key = get_key(event.key().as_str());
    match key {
        KeyValue::SpaceBar => {
            game.toggle_game();
        }
        KeyValue::DownArrow | KeyValue::LeftArrow | KeyValue::RightArrow | KeyValue::UpArrow => {
            game.change_direction(key)
        }
        _ => {}
    }
}

fn click_handler(
    init_data: &mut RefMut<init::InitData>,
    game: &mut RefMut<game_state::State>,
    _event: web_sys::Event,
) {
    logger::info("click");

    game.toggle_game();
    render(init_data, game);
}

fn resize_handler(
    init_data: &mut RefMut<init::InitData>,
    game_data: &mut RefMut<game_state::State>,
) {
    let height: u32 = init_data.root.offset_height().try_into().unwrap();
    let width: u32 = init_data.root.offset_width().try_into().unwrap();

    init_data.canvas.set_width(width);
    init_data.canvas.set_height(height);

    render(init_data, game_data);
}

pub fn register(
    init_data_ref: &Rc<RefCell<init::InitData>>,
    game_state_ref: &Rc<RefCell<game_state::State>>,
) {
    {
        // Keyboard Events
        let init_data = init_data_ref.borrow();
        let game_state_ref_clone = game_state_ref.clone();
        let keydown_callback = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
            event.prevent_default();
            let mut game_state = game_state_ref_clone.borrow_mut();

            keyboard_handler(&mut game_state, event)
        }) as Box<dyn FnMut(_)>);

        match init_data
            .canvas
            .add_event_listener_with_callback("keydown", keydown_callback.as_ref().unchecked_ref())
        {
            Err(e) => logger::error(&format!("Error: {:?}", e)),
            Ok(_) => keydown_callback.forget(),
        }
    }

    {
        // Click Events
        let init_data_ref_clone = init_data_ref.clone();
        let game_state_ref_clone = game_state_ref.clone();

        let onclick_callback = Closure::wrap(Box::new(move |event: web_sys::Event| {
            event.prevent_default();
            let mut init_data = init_data_ref_clone.borrow_mut();
            let mut game_state = game_state_ref_clone.borrow_mut();

            click_handler(&mut init_data, &mut game_state, event)
        }) as Box<dyn FnMut(_)>);

        let init_data = init_data_ref.borrow();

        match init_data
            .canvas
            .add_event_listener_with_callback("click", onclick_callback.as_ref().unchecked_ref())
        {
            Err(e) => logger::error(&format!("Error: {:?}", e)),
            Ok(_) => onclick_callback.forget(),
        }
    }

    {
        // Resize Events
        let init_data_ref_clone = init_data_ref.clone();
        let game_state_ref_clone = game_state_ref.clone();

        let resize_callback = Closure::wrap(Box::new(move |_nothing: f64| {
            resize_handler(
                &mut init_data_ref_clone.borrow_mut(),
                &mut game_state_ref_clone.borrow_mut(),
            )
        }) as Box<dyn FnMut(_)>);

        let init_data = init_data_ref.borrow();

        match init_data
            .window
            .add_event_listener_with_callback("resize", resize_callback.as_ref().unchecked_ref())
        {
            Err(e) => logger::error(&format!("Error: {:?}", e)),
            Ok(_) => resize_callback.forget(),
        }
    }
}
