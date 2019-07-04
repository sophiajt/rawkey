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
            KeyCode::A => readkey::Keycode::A.is_pressed(),
            KeyCode::S => readkey::Keycode::S.is_pressed(),
            KeyCode::Q => readkey::Keycode::Q.is_pressed(),
            KeyCode::X => readkey::Keycode::X.is_pressed(),
            KeyCode::Z => readkey::Keycode::Z.is_pressed(),
        }
    }
}
