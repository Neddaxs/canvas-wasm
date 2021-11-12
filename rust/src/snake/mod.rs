mod state;
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::prelude::*;
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};

extern crate web_sys;

// give us console.log
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

pub fn run(parent_id: &str) -> Result<(), JsValue> {
    // TODO
    // decide how we want out snake game to actually function
    //      o events
    //          onresize: emit(EVENT.RESIZE, ())
    //          onclick: emit(EVENT.CLICKED, ClickData {})
    //
    //      o functions
    //          canvas.onclick = wrap(onclick, render)
    //          window.onkeypress = onkeypress
    //          interval(wrap(update, render))
    //
    //      o observable?
    //          useEffect(() => manager::update(&state), [state]);
    //          useInterval(manager::render(), 1000.0 / 8.0);

    // web_sys::window() gives us the "window" object from javascript
    // (in an option, ie:
    //  Option(window)
    // )
    // unwrap just says, assume that my option was successful and give me that value
    //
    // TODO: remove all unwraps and properly handle them or even just log and panic again so we can
    // do some simple debugging
    let document = web_sys::window().unwrap().document().unwrap();
    let div = document.get_element_by_id(parent_id).unwrap();

    log("snake running!");

    let canvas = document
        .create_element("canvas")
        .unwrap()
        // dyn_into takes the value, and AT RUN TIME, tries to turn it unto the given type
        // in this case, we are saying something like this:
        // ```
        // const canvas = createCanvas();
        //
        // function createCanvas() {
        //  const canvas = document.createElement('canvas'); // implicitly HTMLElement
        //  validateThatThisIsAnActualCanvasElement(canvas);
        //  return canvas;
        // }
        // ```
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap();

    // the )? in this case is a way of saying, throw an error if this fails, if its not here, rust
    // will make use handle the potential error
    div.append_child(&canvas)?;

    let context = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()?;

    // Boxes are essentially a cpp unique_pointer,
    // in other words
    //
    // if we were to do this in golang it would be like:
    // context = *context;
    // but instead of the pointer being to the place in memory, or stack
    // the pointer is to a place on the heap
    //
    // what this really means is that we can have context shared across multiple stacks
    let context = Box::new(context);

    {
        // clone the pointer, ie, let us us context on this stack
        let context = context.clone();

        // Rc is a reference kinda like &variable,
        // but in this case where only one thing can have access to a variable at a time, Rc allows
        // a value to be "shared" in this case, it kinda like a shared reference
        //
        // RefCell makes a value dynamically mutable
        //
        // tldr: this is saying, state is a something that is going to be shared, and its contents
        // are mutable for everyone
        //
        // https://stackoverflow.com/questions/61997859/understanding-usage-of-rcrefcellsomestruct-in-rust
        let state = Rc::new(RefCell::new(state::State::new(context)));

        {
            // TODO: figure out how to create a wrap function ie:
            // ```
            // function wrap(a, b) {
            //  const value = a();
            //  b();
            //  return value;
            // }
            // ```
            //
            // this would simplify a lot of out code, because we could just wrap a: render() in it,
            // or if we want, we can somehow use some sort of observable / value that we can set,
            // ie:
            // state::State::new(context, render)

            // TODO: figure out how to wrap this up
            // something like
            // ```
            // let callback = utils::use_closure<Event>(func: Box<dyn FnMut(_)>)
            // let forget = utils::use_event(element: &?, event_name: &str, callback: &?);
            // ```
            // ^ if something like the above is possible at all
            let state = state.clone();

            // this closure and all of this i dont quite understand too well, but im sure we will
            // figure it out
            let callback = Closure::wrap(Box::new(move |_event: web_sys::Event| {
                let mut s = state.borrow_mut();
                s.click();
                s.render();
                log("clicked!!!!");
            }) as Box<dyn FnMut(_)>);

            canvas.add_event_listener_with_callback("click", callback.as_ref().unchecked_ref())?;

            // this hands memory management of the callback to javascript
            //
            // ie, because we have attached the callback to the html dom element, javascript now
            // has control of the element and the callback, because of this, we tell wasm to not
            // bother freeing the callback memory, but to just let javascript do it.
            //
            // see
            // callback.into_js_value();
            callback.forget();
        }
    }

    Ok(())
}
