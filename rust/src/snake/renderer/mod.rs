use std::{cell::RefCell, rc::Rc};

use wasm_bindgen::JsValue;

use super::{game_state, init};

pub fn render(init_data_ref: &Rc<RefCell<init::InitData>>, state: &game_state::State) {
    let init_data = init_data_ref.clone().borrow();

    let ctx = &init_data.ctx;

    let tile_width = game_state::tile_size() as f64;
    let tile_size = tile_width * init_data.aspect as f64;

    for tile in state.board() {
        match tile.state {
            game_state::Tile::SNAKE => {
                ctx.set_fill_style(&JsValue::from_str("blue"));
            }
            game_state::Tile::EMPTY => {
                ctx.set_fill_style(&JsValue::from_str("green"));
            }
            game_state::Tile::APPLE => {
                ctx.set_fill_style(&JsValue::from_str("red"));
            }
        }

        ctx.fill_rect(
            (tile.col as f64) * tile_size,
            (tile.row as f64) * tile_size,
            tile_size,
            tile_size,
        );
    }
}
