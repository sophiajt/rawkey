use rawkey::{KeyCode, RawKey};

fn main() {
    let mut rawkey = RawKey::new();

    loop {
        if rawkey.is_pressed(KeyCode::Q) {
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
        println!("");
    }
}
