use bevy::input::keyboard::ScanCode;

pub enum KeyboardKey {
    W,
    A,
    S,
    D,
    // Nk stands for Number key
    Nk1,
    Nk2,
    Nk3,
    Nk4,
    Nk5,
}

impl KeyboardKey {
    pub fn scan_code(&self) -> ScanCode {
        #[cfg(target_arch = "wasm32")]
        let id = match self {
            Self::W => 188,
            Self::A => 65,
            Self::S => 79,
            Self::D => 69,
            Self::Nk1 => 49,
            Self::Nk2 => 219,
            Self::Nk3 => 222,
            Self::Nk4 => 191,
            Self::Nk5 => 53,
        };
        #[cfg(not(target_arch = "wasm32"))]
        let id = match self {
            Self::W => 17,
            Self::A => 30,
            Self::S => 31,
            Self::D => 32,
            Self::Nk1 => 2,
            Self::Nk2 => 3,
            Self::Nk3 => 4,
            Self::Nk4 => 5,
            Self::Nk5 => 6,
        };
        ScanCode(id)
    }
}
