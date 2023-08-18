/// 3+1 bit colors, typically used by terminals.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum SystemColor {
    Black = 0,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    Gray,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightGray,
}

impl SystemColor {
    pub fn from_color_code(code: u8) -> Option<Self> {
        match code {
            0 => Some(SystemColor::BrightGray),
            1 => Some(SystemColor::Black),
            2 => Some(SystemColor::Blue),
            3 => Some(SystemColor::Green),
            4 => Some(SystemColor::BrightRed),
            5 => Some(SystemColor::Red),
            6 => Some(SystemColor::Magenta),
            7 => Some(SystemColor::Yellow),
            8 => Some(SystemColor::BrightYellow),
            9 => Some(SystemColor::BrightGreen),
            10 => Some(SystemColor::Cyan),
            11 => Some(SystemColor::BrightCyan),
            12 => Some(SystemColor::BrightBlue),
            13 => Some(SystemColor::BrightMagenta),
            14 => Some(SystemColor::BrightBlack),
            15 => Some(SystemColor::Gray),
            _ => None,
        }
    }
    pub fn into_4bit_offset(self) -> u8 {
        self as u8
    }
    /// Returns a fallback 24-bit color for the given color code.
    ///
    /// Applications should allow users to customize these colors,
    /// or define their own mappings to fit the overall theme of the application.
    ///
    /// These colors were chosen as foreground colors against a dark-gray background.
    pub fn into_24bit(self) -> Rgb {
        match self {
            SystemColor::Black => (0x11, 0x11, 0x11),
            SystemColor::Red => (0xDD, 0x00, 0x11),
            SystemColor::Green => (0x00, 0x88, 0x33),
            SystemColor::Yellow => (0xFF, 0x88, 0x00),
            SystemColor::Blue => (0x00, 0x22, 0xEE),
            SystemColor::Magenta => (0xEE, 0x33, 0x88),
            SystemColor::Cyan => (0x00, 0x77, 0x99),
            SystemColor::Gray => (0xBB, 0xBB, 0xBB),
            SystemColor::BrightBlack => (0x66, 0x66, 0x66),
            SystemColor::BrightRed => (0xFF, 0x22, 0x33),
            SystemColor::BrightGreen => (0x00, 0xDD, 0x44),
            SystemColor::BrightYellow => (0xFF, 0xEE, 0x00),
            SystemColor::BrightBlue => (0x33, 0x55, 0xFF),
            SystemColor::BrightMagenta => (0xFF, 0x55, 0xBB),
            SystemColor::BrightCyan => (0x00, 0xCC, 0xEE),
            SystemColor::BrightGray => (0xEE, 0xEE, 0xEE),
        }
        .into()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
#[repr(u8)]
pub enum Tint {
    Darker = 0,
    #[default]
    Neutral,
    Lighter,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[repr(u8)]
pub enum Hue {
    Red = 0,
    RedYellow, // Yes, this could be "orange", but let's be consistent.
    Yellow,
    GreenYellow,
    Green,
    GreenCyan,
    Cyan,
    BlueCyan,
    Blue,
    BlueMagenta,
    Magenta,
    RedMagenta,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
#[repr(u8)]
pub enum Color {
    #[default]
    Default = 99,
    System(SystemColor) = 0,
    Dark(Tint, Hue) = 16,
    Light(Tint, Hue) = 52,
    Black = 88,
    DarkGray(Tint) = 89,
    Gray(Tint) = 92,
    LightGray(Tint) = 95,
    White = 98,
    Rgb(Rgb) = 100,
}

impl Color {
    pub fn from_color_code(mut code: u8) -> Self {
        // Not great, but this is what happens when one doesn't have an Enum typeclass.
        if let Some(syscolor) = SystemColor::from_color_code(code) {
            Color::System(syscolor)
        } else if code < 88 {
            code -= 16;
            let (dark, tint) = match code / 12 {
                0 => (true, Tint::Darker),
                1 => (true, Tint::Neutral),
                2 => (true, Tint::Lighter),
                3 => (false, Tint::Darker),
                4 => (false, Tint::Neutral),
                5 => (false, Tint::Lighter),
                _ => panic!("Tint value out of bounds; this is a bug"),
            };
            let hue = match code % 12 {
                0x0 => Hue::Red,
                0x1 => Hue::RedYellow,
                0x2 => Hue::Yellow,
                0x3 => Hue::GreenYellow,
                0x4 => Hue::Green,
                0x5 => Hue::GreenCyan,
                0x6 => Hue::Cyan,
                0x7 => Hue::BlueCyan,
                0x8 => Hue::Blue,
                0x9 => Hue::BlueMagenta,
                0xA => Hue::Magenta,
                0xB => Hue::RedMagenta,
                _ => panic!("Hue value out of bounds; this is a bug"),
            };
            if dark {
                Color::Dark(tint, hue)
            } else {
                Color::Light(tint, hue)
            }
        } else {
            match code {
                88 => Color::Black,
                89 => Color::DarkGray(Tint::Darker),
                90 => Color::DarkGray(Tint::Neutral),
                91 => Color::DarkGray(Tint::Lighter),
                92 => Color::Gray(Tint::Darker),
                93 => Color::Gray(Tint::Neutral),
                94 => Color::Gray(Tint::Lighter),
                95 => Color::LightGray(Tint::Darker),
                96 => Color::LightGray(Tint::Neutral),
                97 => Color::LightGray(Tint::Lighter),
                98 => Color::White,
                _ => Color::Default,
            }
        }
    }

    pub fn into_color_code(self) -> Result<u8, Rgb> {
        Ok(match self {
            Color::Default => 99,
            Color::System(sc) => sc as u8,
            Color::Dark(t, c) => 16 + 12 * t as u8 + c as u8,
            Color::Light(t, c) => 52 + 12 * t as u8 + c as u8,
            Color::Black => 88,
            Color::DarkGray(t) => 89 + t as u8,
            Color::Gray(t) => 92 + t as u8,
            Color::LightGray(t) => 95 + t as u8,
            Color::White => 98,
            Color::Rgb(rgb) => return Err(rgb),
        })
    }

    pub fn into_24bit_color(self, f: impl FnOnce(SystemColor) -> Option<Rgb>) -> Option<Rgb> {
        let code = match self {
            Color::Default => return None,
            Color::System(sc) => return Some(f(sc).unwrap_or_else(|| sc.into_24bit())),
            Color::Rgb(rgb) => return Some(rgb),
            c => c.into_color_code().unwrap(),
        };
        Some(COLOR_LUT[code as usize - 16].into())
    }

    pub fn into_8bit_color(self) -> Option<u8> {
        let code = match self {
            Color::Default => return None,
            Color::System(sc) => return Some(sc as u8),
            Color::Rgb(_) => return None,
            c => c.into_color_code().unwrap(),
        };
        Some(ANSI_LUT[code as usize - 16])
    }
}

static COLOR_LUT: [(u8, u8, u8); 83] = [
    // Dark
    (0x47, 0x00, 0x00),
    (0x47, 0x21, 0x00),
    (0x47, 0x47, 0x00),
    (0x32, 0x47, 0x00),
    (0x00, 0x47, 0x00),
    (0x00, 0x47, 0x2c),
    (0x00, 0x47, 0x47),
    (0x00, 0x27, 0x47),
    (0x00, 0x00, 0x47),
    (0x2e, 0x00, 0x47),
    (0x47, 0x00, 0x47),
    (0x47, 0x00, 0x2a),
    (0x74, 0x00, 0x00),
    (0x74, 0x3a, 0x00),
    (0x74, 0x74, 0x00),
    (0x51, 0x74, 0x00),
    (0x00, 0x74, 0x00),
    (0x00, 0x74, 0x49),
    (0x00, 0x74, 0x74),
    (0x00, 0x40, 0x74),
    (0x00, 0x00, 0x74),
    (0x4b, 0x00, 0x74),
    (0x74, 0x00, 0x74),
    (0x74, 0x00, 0x45),
    (0xb5, 0x00, 0x00),
    (0xb5, 0x63, 0x00),
    (0xb5, 0xb5, 0x00),
    (0x7d, 0xb5, 0x00),
    (0x00, 0xb5, 0x00),
    (0x00, 0xb5, 0x71),
    (0x00, 0xb5, 0xb5),
    (0x00, 0x63, 0xb5),
    (0x00, 0x00, 0xb5),
    (0x75, 0x00, 0xb5),
    (0xb5, 0x00, 0xb5),
    (0xb5, 0x00, 0x6b),
    // Light
    (0xff, 0x00, 0x00),
    (0xff, 0x8c, 0x00),
    (0xff, 0xff, 0x00),
    (0xb2, 0xff, 0x00),
    (0x00, 0xff, 0x00),
    (0x00, 0xff, 0xa0),
    (0x00, 0xff, 0xff),
    (0x00, 0x8c, 0xff),
    (0x00, 0x00, 0xff),
    (0xa5, 0x00, 0xff),
    (0xff, 0x00, 0xff),
    (0xff, 0x00, 0x98),
    (0xff, 0x59, 0x59),
    (0xff, 0xb4, 0x59),
    (0xff, 0xff, 0x71),
    (0xcf, 0xff, 0x60),
    (0x6f, 0xff, 0x6f),
    (0x65, 0xff, 0xc9),
    (0x6d, 0xff, 0xff),
    (0x59, 0xb4, 0xff),
    (0x59, 0x59, 0xff),
    (0xc4, 0x59, 0xff),
    (0xff, 0x66, 0xff),
    (0xff, 0x59, 0xbc),
    (0xff, 0x9c, 0x9c),
    (0xff, 0xd3, 0x9c),
    (0xff, 0xff, 0x9c),
    (0xe2, 0xff, 0x9c),
    (0x9c, 0xff, 0x9c),
    (0x9c, 0xff, 0xdb),
    (0x9c, 0xff, 0xff),
    (0x9c, 0xd3, 0xff),
    (0x9c, 0x9c, 0xff),
    (0xdc, 0x9c, 0xff),
    (0xff, 0x9c, 0xff),
    (0xff, 0x94, 0xd3),
    // Grays
    (0x00, 0x00, 0x00),
    (0x13, 0x13, 0x13),
    (0x28, 0x28, 0x28),
    (0x36, 0x36, 0x36),
    (0x4d, 0x4d, 0x4d),
    (0x65, 0x65, 0x65),
    (0x81, 0x81, 0x81),
    (0x9f, 0x9f, 0x9f),
    (0xbc, 0xbc, 0xbc),
    (0xe2, 0xe2, 0xe2),
    (0xff, 0xff, 0xff),
];

static ANSI_LUT: [u8; 83] = [
    // Dark
    52, 94, 100, 58, 22, 29, 23, 24, 17, 54, 53, 89, 88, 130, 142, 64, 28, 35, 30, 25, 18, 91, 90,
    125, 124, 166, 184, 106, 34, 49, 37, 33, 19, 129, 127, 161, // Light
    196, 208, 226, 154, 46, 86, 51, 75, 21, 171, 201, 198, 203, 215, 227, 191, 83, 122, 87, 111, 63,
    177, 207, 205, 217, 223, 229, 193, 157, 158, 159, 153, 147, 183, 219, 212, // Grays
    16, 233, 235, 237, 239, 241, 244, 247, 250, 254, 231,
];

/// A 24-bit color.
#[derive(Clone, Copy, PartialEq, Eq, Default)]
#[repr(C)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl From<(u8, u8, u8)> for Rgb {
    fn from(value: (u8, u8, u8)) -> Self {
        Rgb { r: value.0, g: value.1, b: value.2 }
    }
}

impl std::fmt::Debug for Rgb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

impl std::fmt::Display for Rgb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }
}
