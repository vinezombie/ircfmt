use crate::{Color, Rgb};

#[repr(u16)]
#[non_exhaustive]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FormatFlag {
    Bold = 0b100,
    Italic = 0b1000,
    Underline = 0b10000,
    Strikethrough = 0b100000,
    Monospace = 0b1000000,
    Inverted = 0b10000000,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Format {
    /// In order of least to most significant:
    // HexFg, HexBg, Bold, Italic, Underline, Strikethrough, Monospace, Inverted
    // Upper 8 bits are unused for now.
    flags: u16,
    // We are very naughty and use "r" for a color code if not HexFg/HexBg.
    fg: Rgb,
    bg: Rgb,
}

impl Default for Format {
    fn default() -> Self {
        Format { flags: 0, fg: Rgb { r: 99, g: 0, b: 0 }, bg: Rgb { r: 99, g: 0, b: 0 } }
    }
}

impl Format {
    #[inline]
    pub fn is(&self, fmt: FormatFlag) -> bool {
        self.flags & (fmt as u16) != 0
    }
    /// Sets bold if underline, then unsets underline.
    ///
    /// Some applications may wish to reserve underlines for clickable text,
    /// e.g. links. This function is intended support such applications.
    pub fn underline_to_bold(&mut self) -> &mut Self {
        if self.is(FormatFlag::Underline) {
            self.flags |= FormatFlag::Bold as u16;
            self.flags &= !(FormatFlag::Underline as u16);
        }
        self
    }
    #[inline]
    pub fn set(&mut self, fmt: FormatFlag, value: bool) -> &mut Self {
        if value {
            self.flags |= fmt as u16;
        } else {
            self.flags &= !(fmt as u16);
        }
        self
    }
    /// Returns the foreground (text) color.
    pub fn fg(&self) -> Color {
        if self.flags & 0b1 != 0 {
            Color::Rgb(self.fg)
        } else {
            Color::from_color_code(self.fg.r)
        }
    }
    /// Returns the background color.
    pub fn bg(&self) -> Color {
        if self.flags & 0b10 != 0 {
            Color::Rgb(self.bg)
        } else {
            Color::from_color_code(self.bg.r)
        }
    }
    /// Sets the foreground (text) color.
    pub fn set_fg(&mut self, color: Color) -> &mut Self {
        match color.into_color_code() {
            Ok(cc) => {
                self.fg.r = cc;
                self.flags &= !0b1;
            }
            Err(color) => {
                self.fg = color;
                self.flags |= 0b1;
            }
        }
        self
    }
    /// Sets the background color.
    pub fn set_bg(&mut self, color: Color) -> &mut Self {
        match color.into_color_code() {
            Ok(cc) => {
                self.bg.r = cc;
                self.flags &= !0b10;
            }
            Err(color) => {
                self.bg = color;
                self.flags |= 0b10;
            }
        }
        self
    }
}
