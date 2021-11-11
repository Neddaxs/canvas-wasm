use gloo_events::EventListener;
use std::ops::Deref;
use wasm_bindgen::JsCast;
use web_sys::CanvasRenderingContext2d;

extern crate web_sys;

struct State {
    clicks: i32,
    aspect: i32,
    ctx: Option<CanvasRenderingContext2d>,
}

impl State {
    fn click(&mut self) {
        self.clicks += 1;
    }
    fn set_ctx(&mut self, ctx: CanvasRenderingContext2d) {
        self.ctx = Some(ctx);
    }
}

const STATE: State = State {
    clicks: 0,
    ctx: None,
    aspect: 1,
};

fn render() {
    match STATE.ctx {
        Some(ctx) => {
            ctx.rect(
                0.0,
                0.0,
                ctx.canvas().unwrap().offset_width().into(),
                ctx.canvas().unwrap().offset_height().into(),
            );

            let x = (5 * STATE.aspect).into();
            let y = (3 * STATE.aspect).into();

            let rendered_text = ctx.fill_text(STATE.clicks.to_string().as_str(), x, y);
            assert_eq!(rendered_text.is_ok(), true);

            ctx.fill_rect(x, x, x, x);
        }
        _ => {}
    }
}

fn handle_onclick() {
    STATE.click();
    render();
}

fn handle_resize() {
    match STATE.ctx {
        Some(ctx) => {
            let offset_width = ctx.canvas().unwrap().offset_width().try_into().unwrap();
            let offset_height = ctx.canvas().unwrap().offset_height().try_into().unwrap();

            if offset_width > offset_height {
                ctx.canvas().unwrap().set_width(offset_height);
                ctx.canvas().unwrap().set_height(offset_height);
            } else {
                ctx.canvas().unwrap().set_width(offset_width);
                ctx.canvas().unwrap().set_height(offset_width);
            }
            render()
        }
        _ => {}
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

    STATE.set_ctx(context);

    let append = parent_container.append_child(canvas_node.clone().deref());
    assert_eq!(append.is_ok(), true);

    let onresize = EventListener::new(&window, "resize", |_event| handle_resize());

    let onclick = EventListener::new(&canvas, "onclick", |_event| {
        handle_onclick();
    });

    let cleanup = move || {
        onclick.forget();
        onresize.forget();
        let removed = parent_container.remove_child(canvas_node.clone().deref());
        assert_eq!(removed.is_ok(), true);
    };

    return cleanup;
}
