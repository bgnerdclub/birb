#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]

use birb::Module;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Key {
    Escape,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    Backquote,
    LeftParen,
    RightParen,
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,
    Key0,
    Hyphen,
    Underscore,
    Equals,
    Plus,
    Tab,
    Q,
    W,
    E,
    R,
    T,
    Y,
    U,
    I,
    O,
    P,
    A,
    S,
    D,
    F,
    G,
    H,
    J,
    K,
    L,
    Z,
    X,
    C,
    V,
    B,
    N,
    M,
    LeftSquare,
    RightSquare,
    LeftBracket,
    RightBracket,
    CapsLock,
    Colon,
    Semicolon,
    Apostrophe,
    At,
    Hash,
    Tilde,
    Pipe,
    Backslash,
    LeftAngle,
    RightAngle,
    Period,
    Comma,
    Slash,
    Question,
    LeftShift,
    RightShift,
    LeftControl,
    RightControl,
    LeftSuper,
    RightSuper,
    LeftAlt,
    RightAlt,
    Up,
    Down,
    Left,
    Right,
    Space,
}

impl Key {
    #[must_use]
    pub const fn from_char(c: char) -> Option<Self> {
        match c.to_ascii_lowercase() {
            '`' => Some(Self::Backquote),
            '(' => Some(Self::LeftParen),
            ')' => Some(Self::RightParen),
            '1' => Some(Self::Key1),
            '2' => Some(Self::Key2),
            '3' => Some(Self::Key3),
            '4' => Some(Self::Key4),
            '5' => Some(Self::Key5),
            '6' => Some(Self::Key6),
            '7' => Some(Self::Key7),
            '8' => Some(Self::Key8),
            '9' => Some(Self::Key9),
            '0' => Some(Self::Key0),
            '-' => Some(Self::Hyphen),
            '_' => Some(Self::Underscore),
            '=' => Some(Self::Equals),
            '+' => Some(Self::Plus),
            '\t' => Some(Self::Tab),
            'q' => Some(Self::Q),
            'w' => Some(Self::W),
            'e' => Some(Self::E),
            'r' => Some(Self::R),
            't' => Some(Self::T),
            'y' => Some(Self::Y),
            'u' => Some(Self::U),
            'i' => Some(Self::I),
            'o' => Some(Self::O),
            'p' => Some(Self::P),
            'a' => Some(Self::A),
            's' => Some(Self::S),
            'd' => Some(Self::D),
            'f' => Some(Self::F),
            'g' => Some(Self::G),
            'h' => Some(Self::H),
            'j' => Some(Self::J),
            'k' => Some(Self::K),
            'l' => Some(Self::L),
            'z' => Some(Self::Z),
            'x' => Some(Self::X),
            'c' => Some(Self::C),
            'v' => Some(Self::V),
            'b' => Some(Self::B),
            'n' => Some(Self::N),
            'm' => Some(Self::M),
            '[' => Some(Self::LeftSquare),
            ']' => Some(Self::RightSquare),
            '{' => Some(Self::LeftBracket),
            '}' => Some(Self::RightBracket),
            ':' => Some(Self::Colon),
            ';' => Some(Self::Semicolon),
            '\'' => Some(Self::Apostrophe),
            '@' => Some(Self::At),
            '#' => Some(Self::Hash),
            '~' => Some(Self::Tilde),
            '|' => Some(Self::Pipe),
            '\\' => Some(Self::Backslash),
            '<' => Some(Self::LeftAngle),
            '>' => Some(Self::RightAngle),
            '.' => Some(Self::Period),
            ',' => Some(Self::Comma),
            '/' => Some(Self::Slash),
            '?' => Some(Self::Question),
            ' ' => Some(Self::Space),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Event {
    KeyPress(Key),
    KeyRelease(Key),
}

#[derive(Debug, Default)]
pub struct Window {
    down: Vec<Key>,
}

impl Window {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn is_down(&self, key: Key) -> bool {
        self.down.contains(&key)
    }

    pub fn submit(&mut self, event: Event) {
        println!("{event:?}");
        match event {
            Event::KeyPress(key) => {
                if !self.down.contains(&key) {
                    self.down.push(key);
                }
            }
            Event::KeyRelease(key) => {
                if let Some(index) = self.down.iter().position(|x| *x == key) {
                    self.down.remove(index);
                }
            }
        }
    }
}

impl Module for Window {}
