#[cfg(windows)]
mod win;
#[cfg(windows)]
pub use win::RawKey;

#[cfg(unix)]
mod linux;
#[cfg(unix)]
pub use linux::RawKey;

#[cfg(macos)]
mod macos;
#[cfg(macos)]
pub use macos::RawKey;

pub enum KeyCode {
    Q,
    UpArrow,
    DownArrow,
    LeftArrow,
    RightArrow,
    A,
    S,
    Z,
    X,
}
