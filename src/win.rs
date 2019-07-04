use crate::KeyCode;
use user32::GetAsyncKeyState;
use winapi::winuser;

pub struct RawKey;

impl RawKey {
    pub fn new() -> RawKey {
        RawKey
    }
    pub fn is_pressed(&self, key: KeyCode) -> bool {
        match key {
            KeyCode::LeftArrow => query_keystate(winuser::VK_LEFT),
            KeyCode::RightArrow => query_keystate(winuser::VK_RIGHT),
            KeyCode::UpArrow => query_keystate(winuser::VK_UP),
            KeyCode::DownArrow => query_keystate(winuser::VK_DOWN),
            KeyCode::A => query_keystate(winuser::VK_A),
            KeyCode::S => query_keystate(winuser::VK_S),
            KeyCode::Z => query_keystate(winuser::VK_Z),
            KeyCode::X => query_keystate(winuser::VK_X),
            KeyCode::Q => query_keystate(winuser::VK_Q),
        }
    }
}

fn query_keystate(key: u8) -> bool {
    if GetAsyncKeyState(key) as u32 & 0x8000 != 0 {
        true
    } else {
        false
    }
}
