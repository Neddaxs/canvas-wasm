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
        "ArrowLeft" | "a" | "A" | "h" => KeyValue::LeftArrow,
        "ArrowRight" | "d" | "D" | "l" => KeyValue::RightArrow,
        "ArrowDown" | "s" | "S" | "j" => KeyValue::DownArrow,
        "ArrowUp" | "w" | "W" | "k" => KeyValue::UpArrow,
        " " | "Spacebar" => KeyValue::SpaceBar,
        _ => KeyValue::Unknown,
    }
}
