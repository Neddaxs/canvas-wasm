use std::{
    cell::{RefCell, RefMut},
    rc::Rc,
};
use wasm_bindgen::{prelude::Closure, JsCast};

use crate::snake::state::State;
use crate::snake::utils::{keys, logger};

extern crate web_sys;

fn keyboard_event(state: &mut RefMut<State>, event: web_sys::KeyboardEvent) {
    logger::info("keydown");

    let key = keys::get_key(event.key().as_str());
    state.change_direction(key);
}

fn click_event(state: &mut RefMut<State>, _event: web_sys::Event) {
    logger::info("click");

    state.toggle_game();
}

pub fn register(
    canvas_ref: &Rc<RefCell<Box<web_sys::HtmlCanvasElement>>>,
    state_ref: &Rc<RefCell<State>>,
) {
    let canvas = canvas_ref.borrow_mut();

    {
        // Keyboard Events
        let state = state_ref.clone();
        let keydown_callback = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
            event.prevent_default();
            let mut mutable_state = state.borrow_mut();

            keyboard_event(&mut mutable_state, event)
        }) as Box<dyn FnMut(_)>);

        match canvas
            .add_event_listener_with_callback("keydown", keydown_callback.as_ref().unchecked_ref())
        {
            Err(e) => logger::error(&format!("Error: {:?}", e)),
            Ok(_) => keydown_callback.forget(),
        }
    }

    {
        // Click Events
        let state = state_ref.clone();
        let onclick_callback = Closure::wrap(Box::new(move |event: web_sys::Event| {
            event.prevent_default();
            let mut mutable_state = state.borrow_mut();

            click_event(&mut mutable_state, event)
        }) as Box<dyn FnMut(_)>);

        match canvas
            .add_event_listener_with_callback("click", onclick_callback.as_ref().unchecked_ref())
        {
            Err(e) => logger::error(&format!("Error: {:?}", e)),
            Ok(_) => onclick_callback.forget(),
        }
    }
}
