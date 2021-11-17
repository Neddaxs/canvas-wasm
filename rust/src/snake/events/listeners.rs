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
    match game.running_state {
        game_state::RunningState::RUNNING => {}
        _ => {
            game.toggle_game();
            render(init_data, game);
        }
    }
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

fn mouse_down_handler(init_data: &mut RefMut<init::InitData>, event: web_sys::MouseEvent) {
    let x = event.client_x();
    let y = event.client_y();

    init_data.mouse_down(x, y);
}

fn mouse_up_handler(
    init_data: &mut RefMut<init::InitData>,
    game_data: &mut RefMut<game_state::State>,
    event: web_sys::MouseEvent,
) {
    let x = event.client_x();
    let y = event.client_y();

    let (start_x, start_y) = match &init_data.location {
        Some(val) => (val.x, val.y),
        _ => (x, y),
    };

    init_data.mouse_up();

    if x == start_x && y == start_y {
        return;
    }

    let x_diff = x - start_x;
    let y_diff = y - start_y;

    let direction = if x_diff > y_diff && x_diff > 0 {
        KeyValue::RightArrow
    } else if x_diff > y_diff {
        KeyValue::LeftArrow
    } else if y_diff > 0 {
        KeyValue::DownArrow
    } else {
        KeyValue::UpArrow
    };

    game_data.change_direction(direction);
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

            keyboard_handler(&mut game_state_ref_clone.borrow_mut(), event)
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

            click_handler(
                &mut init_data_ref_clone.borrow_mut(),
                &mut game_state_ref_clone.borrow_mut(),
                event,
            )
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
        // Mouse Down Events
        let init_data_ref_clone = init_data_ref.clone();
        let mouse_down_callback = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            event.prevent_default();

            mouse_down_handler(&mut init_data_ref_clone.borrow_mut(), event)
        }) as Box<dyn FnMut(_)>);

        let init_data = init_data_ref.borrow();

        match init_data.canvas.add_event_listener_with_callback(
            "mousedown",
            mouse_down_callback.as_ref().unchecked_ref(),
        ) {
            Err(e) => logger::error(&format!("Error: {:?}", e)),
            Ok(_) => mouse_down_callback.forget(),
        }
    }

    {
        // Mouse Up Events
        let init_data_ref_clone = init_data_ref.clone();
        let game_state_ref_clone = game_state_ref.clone();
        let mouse_up_callback = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            event.prevent_default();

            mouse_up_handler(
                &mut init_data_ref_clone.borrow_mut(),
                &mut game_state_ref_clone.borrow_mut(),
                event,
            )
        }) as Box<dyn FnMut(_)>);

        let init_data = init_data_ref.borrow();

        match init_data
            .canvas
            .add_event_listener_with_callback("mouseup", mouse_up_callback.as_ref().unchecked_ref())
        {
            Err(e) => logger::error(&format!("Error: {:?}", e)),
            Ok(_) => mouse_up_callback.forget(),
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
