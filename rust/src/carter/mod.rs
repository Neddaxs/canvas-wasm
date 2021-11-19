mod dom;
mod logger;
mod lowlevel;
mod types;

use nalgebra::{Vector2, Vector3};

extern crate nalgebra as na;

pub fn run(root_id: &str) {
    console_error_panic_hook::set_once();
    logger::info("Starting up!");

    match dom::setup(root_id) {
        Ok((_dom, _gl)) => {
            let vertex = types::Vertex::new(
                Vector3::new(0.0, 0.0, 0.0),
                Vector3::new(0.0, 0.0, 0.0),
                Vector2::new(0.0, 0.0),
            );

            logger::info(&format!("{}", &lowlevel::FRAGMENT_SHADER_SOURCE));
            logger::info(&format!("{}", &lowlevel::VERTEX_SHADER_SOURCE));
            logger::info(&format!("{:?}", &vertex));
        }
        Err(e) => logger::error(&format!("{:?}", e)),
    }
}
