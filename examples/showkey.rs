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
        if rawkey.is_pressed(KeyCode::LeftShift) {
            print!("LeftShift ");
        }
        if rawkey.is_pressed(KeyCode::BackSpace) {
            print!("BackSpace ");
        }
        if rawkey.is_pressed(KeyCode::LeftControl) {
            print!("LeftControl ");
        }
        if rawkey.is_pressed(KeyCode::Tab) {
            print!("Tab ");
        }
        if rawkey.is_pressed(KeyCode::Space) {
            print!("Space ");
        }
        if rawkey.is_pressed(KeyCode::PageUp) {
            print!("PageUp ");
        }
        if rawkey.is_pressed(KeyCode::PageDown) {
            print!("PageDown ");
        }
        if rawkey.is_pressed(KeyCode::Alt) {
            print!("Alt ");
        }
        println!("");
    }
}
