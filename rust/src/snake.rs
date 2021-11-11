use gloo_events::EventListener;
use std::ops::Deref;
use wasm_bindgen::JsCast;
use web_sys::CanvasRenderingContext2d;

extern crate web_sys;

struct State {
    clicks: i32,
    ctx: CanvasRenderingContext2d,
    aspect: i32,
}

impl State {
    fn click(&self) {
        self.clicks += 1;
    }
    fn set_aspect(&self, ratio: i32) {
        self.aspect = ratio;
    }
    fn set_ctx(&self, ctx: CanvasRenderingContext2d) {
        self.ctx = ctx;
    }
}

pub fn run(canvas_id: &str) -> impl FnOnce() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    let parent_container = document.get_element_by_id(canvas_id).unwrap();

    let canvas_node = document.get_element_by_id("canvas").unwrap();

    let canvas = canvas_node
        .clone()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    let mut state = State {
        clicks: 0,
        ctx: context,
        aspect: 1,
    };

    let append = parent_container.append_child(canvas_node.clone().deref());
    assert_eq!(append.is_ok(), true);

    let render = || {
        state.ctx.rect(
            0.0,
            0.0,
            state.ctx.canvas().unwrap().offset_width().into(),
            state.ctx.canvas().unwrap().offset_height().into(),
        );

        let x = (5 * state.aspect).into();
        let y = (3 * state.aspect).into();

        let rendered_text = state.ctx.fill_text(state.clicks.to_string().as_str(), x, y);
        assert_eq!(rendered_text.is_ok(), true);

        state.ctx.fill_rect(x, x, x, x);
    };

    let onresize = EventListener::new(&window, "resize", |_event| {
        let offset_width = state
            .ctx
            .canvas()
            .unwrap()
            .offset_width()
            .try_into()
            .unwrap();
        let offset_height = state
            .ctx
            .canvas()
            .unwrap()
            .offset_height()
            .try_into()
            .unwrap();

        if offset_width > offset_height {
            state.ctx.canvas().unwrap().set_width(offset_height);
            state.ctx.canvas().unwrap().set_height(offset_height);
        } else {
            state.ctx.canvas().unwrap().set_width(offset_width);
            state.ctx.canvas().unwrap().set_height(offset_width);
        }
        render()
    });

    let onclick = EventListener::new(&canvas, "onclick", |_event| {
        state.click();
        render();
    });

    let cleanup = move || {
        onclick.forget();
        let removed = parent_container.remove_child(canvas_node.clone().deref());
        assert_eq!(removed.is_ok(), true);
    };

    return cleanup;
}
