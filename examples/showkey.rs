use rawkey::{KeyCode, RawKey};

fn main() {
    let mut rawkey = RawKey::new();

    loop {
        if rawkey.is_pressed(KeyCode::Escape) {
            break;
        }
        if rawkey.is_pressed(KeyCode::UpArrow) {
            print!("Up ");
        }
        if rawkey.is_pressed(KeyCode::DownArrow) {
            print!("Down ");
        }
        if rawkey.is_pressed(KeyCode::LeftArrow) {
            print!("Left ");
        }
        if rawkey.is_pressed(KeyCode::RightArrow) {
            print!("Right ");
        }
        if rawkey.is_pressed(KeyCode::LShift) {
            print!("LShift ");
        }
        if rawkey.is_pressed(KeyCode::Back) {
            print!("Back ");
        }
        if rawkey.is_pressed(KeyCode::LControl) {
            print!("LControl ");
        }
        if rawkey.is_pressed(KeyCode::Tab) {
            print!("Tab ");
        }
        if rawkey.is_pressed(KeyCode::Space) {
            print!("Space ");
        }
        println!("");
    }
}
