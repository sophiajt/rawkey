use crate::KeyCode;
use readkey;

pub struct RawKey;

impl RawKey {
    pub fn new() -> RawKey {
        RawKey
    }

    pub fn is_pressed(&self, key: KeyCode) -> bool {
        match key {
            KeyCode::LeftArrow => readkey::Keycode::Left.is_pressed(),
            KeyCode::RightArrow => readkey::Keycode::Right.is_pressed(),
            KeyCode::UpArrow => readkey::Keycode::Up.is_pressed(),
            KeyCode::DownArrow => readkey::Keycode::Down.is_pressed(),
            KeyCode::LShift => readkey::Keycode::Shift.is_pressed(),
            KeyCode::LControl => readkey::Keycode::Control.is_pressed(),
            KeyCode::Escape => readkey::Keycode::Escape.is_pressed(),
            KeyCode::Space => readkey::Keycode::Space.is_pressed(),
            KeyCode::Tab => readkey::Keycode::Tab.is_pressed(),
            KeyCode::Back => readkey::Keycode::Delete.is_pressed(),
            KeyCode::PageUp => readkey::Keycode::PageUp.is_pressed(),
            KeyCode::PageDown => readkey::Keycode::PageDown.is_pressed(),
        }
    }
}
