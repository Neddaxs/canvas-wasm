use std::{
    cell::{RefCell, RefMut},
    rc::Rc,
};
use wasm_bindgen::{prelude::Closure, JsCast};

use crate::snake::{
    game_state, init,
    utils::{keys, logger},
};

extern crate web_sys;

fn keyboard_handler(state: &mut RefMut<game_state::State>, event: web_sys::KeyboardEvent) {
    logger::info("keydown");

    let key = keys::get_key(event.key().as_str());
    state.change_direction(key);
}

// fn click_handler(state: &mut RefMut<game_state::State>, _event: web_sys::Event) {
//     logger::info("click");
//
//     state.toggle_game();
// }

fn resize_handler(init_data: &mut RefMut<init::InitData>) {
    let height: u32 = init_data.root.offset_height().try_into().unwrap();
    let width: u32 = init_data.root.offset_height().try_into().unwrap();

    init_data.canvas.set_width(width);
    init_data.canvas.set_height(height);
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

    //{
    //    // Click Events
    //    let game_state_ref_clone = game_state_ref.clone();

    //    let onclick_callback = Closure::wrap(Box::new(move |event: web_sys::Event| {
    //        event.prevent_default();
    //        let mut game_state = game_state_ref_clone.borrow_mut();

    //        click_handler(&mut game_state, event)
    //    }) as Box<dyn FnMut(_)>);

    //    match init_data
    //        .canvas
    //        .add_event_listener_with_callback("click", onclick_callback.as_ref().unchecked_ref())
    //    {
    //        Err(e) => logger::error(&format!("Error: {:?}", e)),
    //        Ok(_) => onclick_callback.forget(),
    //    }
    //}

    {
        let init_data_ref_clone = init_data_ref.clone();

        let resize_callback = Closure::wrap(Box::new(move |_nothing: i64| {
            resize_handler(&mut init_data_ref_clone.borrow_mut())
        }) as Box<dyn FnMut(_)>);

        let init_data = init_data_ref.borrow();

        match init_data
            .root
            .add_event_listener_with_callback("resize", resize_callback.as_ref().unchecked_ref())
        {
            Err(e) => logger::error(&format!("Error: {:?}", e)),
            Ok(_) => resize_callback.forget(),
        }
    }
}
