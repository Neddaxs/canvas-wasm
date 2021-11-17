#[derive(Debug)]
pub enum KeyValue {
    LeftArrow,
    RightArrow,
    UpArrow,
    DownArrow,
    SpaceBar,
    Unknown,
}

pub fn get_key(key: &str) -> KeyValue {
    match key {
        "ArrowLeft" | "a" | "A" => KeyValue::LeftArrow,
        "ArrowRight" | "d" | "D" => KeyValue::RightArrow,
        "ArrowDown" | "s" | "S" => KeyValue::DownArrow,
        "ArrowUp" | "w" | "W" => KeyValue::UpArrow,
        " " | "Spacebar" => KeyValue::SpaceBar,
        _ => KeyValue::Unknown,
    }
}
