#[derive(Debug)]
pub enum KeyValue {
    LeftArrow,
    RightArrow,
    UpArrow,
    DownArrow,
    Unknown,
}

pub fn get_key(key: &str) -> KeyValue {
    match key {
        "ArrowLeft" => KeyValue::LeftArrow,
        "a" => KeyValue::LeftArrow,
        "A" => KeyValue::LeftArrow,
        "ArrowRight" => KeyValue::RightArrow,
        "d" => KeyValue::RightArrow,
        "D" => KeyValue::RightArrow,
        "ArrowDown" => KeyValue::DownArrow,
        "s" => KeyValue::DownArrow,
        "S" => KeyValue::DownArrow,
        "ArrowUp" => KeyValue::UpArrow,
        "w" => KeyValue::DownArrow,
        "W" => KeyValue::DownArrow,
        _ => KeyValue::Unknown,
    }
}
