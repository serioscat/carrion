use abi_stable::{
    StableAbi
};
use serde::{Serialize, Deserialize};

pub const MOD_SHIFT: u8 = 1;
pub const MOD_CTRL: u8 = 2;
pub const MOD_ALT: u8 = 4;
pub const MOD_SUPER: u8 = 8;

#[repr(u16)]
#[derive(StableAbi, Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Key {
    Unknown = 0,
    A = 1, B, C, D, E, F, G, H, I, J, K, L, M,
    N, O, P, Q, R, S, T, U, V, W, X, Y, Z,
    Num0 = 100, Num1, Num2, Num3, Num4, Num5, Num6, Num7, Num8, Num9,
    Enter = 200, Escape, Backspace, Tab, Space,
    Left, Right, Up, Down,
    F1 = 300, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12,
}

#[repr(C)]
#[derive(StableAbi, Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct KeyEvent {
    pub key: Key,
    pub mods: u8,
    pub pressed: bool,
    pub repeat: bool,
}

#[repr(u8)]
#[derive(StableAbi, Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Other
}

#[repr(u8)]
#[derive(StableAbi, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Event {
    Key(KeyEvent),
    Char(u32),
    MouseMove { x: f32, y: f32 },
    MouseButton { button: MouseButton, pressed: bool }
}


#[repr(u8)]
#[derive(StableAbi, Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Action {
    /// Handle the action at the end (don't call it)
    Handled,
    /// Inputs or outputs have been overridden with new values
    Continue,
    /// Continue with the original action
    Changed,
    /// Immediately stop the hook chain and handle the original
    Stop,
}