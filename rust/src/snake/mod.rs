use wasm_bindgen::JsValue;

mod init;
mod state;
mod utils;

extern crate web_sys;

pub fn run(root_id: &str) {
    utils::logger::info("snake running!");

    match init::InitData::new(root_id) {
        Ok(init_data) => {
            utils::logger::info("Successfully initialized data!");

            init_data.ctx.set_fill_style(&JsValue::from_str("orange"));
            init_data.ctx.fill_rect(19.0, 20.0, 300.0, 100.0);

            let boxed_ctx = Box::new(init_data.ctx);

            let mut s = state::State::new(boxed_ctx);

            utils::logger::info(&format!("state has clicks: {}", s.clicks));

            s.click();

            utils::logger::info(&format!("state has clicks: {}", s.clicks));

            utils::logger::info(&format!("state has width: {}", init_data.canvas.width()));

            s.render();
        }
        Err(e) => utils::logger::error(&format!("Error: {:?}", e)),
    }
    /*

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
    */
}
