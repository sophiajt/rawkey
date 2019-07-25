use crate::KeyCode;
use user32::GetAsyncKeyState;
use winapi::um::winuser;

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
            KeyCode::LeftShift => query_keystate(winuser::VK_LSHIFT),
            KeyCode::Space => query_keystate(winuser::VK_SPACE),
            KeyCode::BackSpace => query_keystate(winuser::VK_BACK),
            KeyCode::LeftControl => query_keystate(winuser::VK_LCONTROL),
            KeyCode::Tab => query_keystate(winuser::VK_TAB),
            KeyCode::Escape => query_keystate(winuser::VK_ESCAPE),
            KeyCode::PageDown => query_keystate(winuser::VK_NEXT),
            KeyCode::PageUp => query_keystate(winuser::VK_PRIOR),
            KeyCode::Alt => query_keystate(winuser::VK_MENU),
        }
    }
}

fn query_keystate(key: i32) -> bool {
    unsafe {
        if GetAsyncKeyState(key) as u32 & 0x8000 != 0 {
            true
        } else {
            false
        }
    }
}
