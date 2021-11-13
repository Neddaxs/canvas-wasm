mod console {
    use wasm_bindgen::prelude::wasm_bindgen;

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = console)]
        pub fn log(s: &str);

        #[wasm_bindgen(js_namespace = console)]
        pub fn error(s: &str);
    }
}

fn format(level: &str, message: &str) -> String {
    format!("[SNAKE] {} - {}", level, message)
}

pub fn info(message: &str) {
    console::log(&format("INFO", message));
}

pub fn error(message: &str) {
    console::error(&format("ERROR", message));
}
