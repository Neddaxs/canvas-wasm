use std::{
    cell::{RefCell, RefMut},
    rc::Rc,
};
use wasm_bindgen::{prelude::Closure, JsCast};

use crate::snake::state::State;
use crate::snake::utils::{keys, logger};

extern crate web_sys;

fn keyboard_handler(state: &mut RefMut<State>, event: web_sys::KeyboardEvent) {
    logger::info("keydown");

    let key = keys::get_key(event.key().as_str());
    state.change_direction(key);
}

fn click_handler(state: &mut RefMut<State>, _event: web_sys::Event) {
    logger::info("click");

    state.toggle_game();
}

fn resize_handler(
    canvas: &mut RefMut<Box<web_sys::HtmlCanvasElement>>,
    root: &mut RefMut<Box<web_sys::HtmlDivElement>>,
) {
    let height: u32 = root.offset_height().try_into().unwrap();
    let width: u32 = root.offset_height().try_into().unwrap();

    canvas.set_width(width);
    canvas.set_height(height);
}

pub fn register(
    canvas_ref: &Rc<RefCell<Box<web_sys::HtmlCanvasElement>>>,
    state_ref: &Rc<RefCell<State>>,
    root_ref: &Rc<RefCell<Box<web_sys::HtmlDivElement>>>,
) {
    let canvas = canvas_ref.borrow_mut();
    let root = root_ref.borrow_mut();

    {
        // Keyboard Events
        let state = state_ref.clone();
        let keydown_callback = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
            event.prevent_default();
            let mut mutable_state = state.borrow_mut();

            keyboard_handler(&mut mutable_state, event)
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

            click_handler(&mut mutable_state, event)
        }) as Box<dyn FnMut(_)>);

        match canvas
            .add_event_listener_with_callback("click", onclick_callback.as_ref().unchecked_ref())
        {
            Err(e) => logger::error(&format!("Error: {:?}", e)),
            Ok(_) => onclick_callback.forget(),
        }
    }

    {
        // Resize Events
        let root_clone = root_ref.clone();
        let canvas_clone = canvas_ref.clone();

        let resize_callback = Closure::wrap(Box::new(move |_nothing: i64| {
            let mut mutable_root = root_clone.borrow_mut();
            let mut mutable_canvas = canvas_clone.borrow_mut();

            resize_handler(&mut mutable_canvas, &mut mutable_root)
        }) as Box<dyn FnMut(_)>);

        match root
            .add_event_listener_with_callback("resize", resize_callback.as_ref().unchecked_ref())
        {
            Err(e) => logger::error(&format!("Error: {:?}", e)),
            Ok(_) => resize_callback.forget(),
        }
    }
}
