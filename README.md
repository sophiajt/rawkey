Early release of a project to support raw key input in terminals. Currently, this supports the following raw keys across the major OSes:

* LShift - Left Shift (or either Shift on some OSes)
* LControl - Left Control
* Back - Backspace
* Space 
* Tab 
* Escape
* LeftArrow
* UpArrow
* RightArrow
* DownArrow

Rather than using events, rawkey offers a way to scan to see if the key is pressed or not.

```rust
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
    println!("");
}
```
