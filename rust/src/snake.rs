use gloo_events::EventListener;
use std::{
    cell::{Cell, RefCell},
    ops::Deref,
    rc::Rc,
};
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::CanvasRenderingContext2d;

extern crate web_sys;

struct State {
    clicks: i32,
    aspect: i32,
    ctx: Box<CanvasRenderingContext2d>,
}

impl State {
    fn render(&mut self) {
        let ctx = self.ctx.as_mut();
        ctx.set_fill_style(&JsValue::from_str("red"));
        ctx.fill_rect(10.0, 10.0, 100.0, 100.0);
    }
}

/// Runs the snake game
///
/// # Arguments
///
/// * `canvas_id` - the string of the canvas element to use
///
/// # Errors
///
/// Returns nothing is everything went fine, or an javascript value if there was an error
///
/// # Examples
///
/// ```
/// snake::run("my-canvas-id")?
/// ```
pub fn run(canvas_id: &str) -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document
        .get_element_by_id(canvas_id)
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()?;

    let context = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()?;

    let context = Box::new(context);

    {
        let context = context.clone();
        let state = Rc::new(RefCell::new(State {
            clicks: 0,
            aspect: 0,
            ctx: context,
        }));

        {
            let state = state.clone();
            let callback = Closure::wrap(Box::new(move |_event: web_sys::Event| {
                let mut s = state.borrow_mut();
                s.clicks += 1;
                s.render();
            }) as Box<dyn FnMut(_)>);
            canvas
                .add_event_listener_with_callback("onclick", callback.as_ref().unchecked_ref())?;
            callback.forget();
        }

        // {
        //     let state = state.clone();
        // }
    }

    // {
    //     let context = context.clone();
    //     let pressed = pressed.clone();
    //     let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
    //         if pressed.get() {
    //             context.line_to(event.offset_x() as f64, event.offset_y() as f64);
    //             context.stroke();
    //             context.begin_path();
    //             context.move_to(event.offset_x() as f64, event.offset_y() as f64);
    //         }
    //     }) as Box<dyn FnMut(_)>);
    //     canvas.add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())?;
    //     closure.forget();
    // }
    // {
    //     let context = context.clone();
    //     let pressed = pressed.clone();
    //     let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
    //         pressed.set(false);
    //         context.line_to(event.offset_x() as f64, event.offset_y() as f64);
    //         context.stroke();
    //     }) as Box<dyn FnMut(_)>);
    //     canvas.add_event_listener_with_callback("mouseup", closure.as_ref().unchecked_ref())?;
    //     closure.forget();
    // }

    Ok(())
}
