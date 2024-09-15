#[repr(C)]
#[derive(PartialEq, Eq, Copy, Clone)]
pub enum TerminalColor {
    Default = 0,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
    // Extended colors
    DarkGreen,
    LightGreen,
    NeonGreen,
    PaleCyan,
    Brown,
    PaleGreen,
    PaleOrange,
    DarkOrange,
    Purple,
    PalePurple,
    BrightPurple,
    Pink,
    PaleYellow,
    LightBlue,
    LightGrey,
    PaleGrey,
    Grey,
}

impl TerminalColor {
    pub fn color(self) -> &'static str {
        match self {
            TerminalColor::Default => "",
            TerminalColor::Black => "\x1b[30m",
            TerminalColor::Red => "\x1b[31m",
            TerminalColor::Green => "\x1b[32m",
            TerminalColor::Yellow => "\x1b[33m",
            TerminalColor::Blue => "\x1b[34m",
            TerminalColor::Magenta => "\x1b[35m",
            TerminalColor::Cyan => "\x1b[36m",
            TerminalColor::White => "\x1b[37m",
            TerminalColor::BrightBlack => "\x1b[30m;1m",
            TerminalColor::BrightRed => "\x1b[31m;1m",
            TerminalColor::BrightGreen => "\x1b[32m;1m",
            TerminalColor::BrightYellow => "\x1b[33m;1m",
            TerminalColor::BrightBlue => "\x1b[34m;1m",
            TerminalColor::BrightMagenta => "\x1b[35m;1m",
            TerminalColor::BrightCyan => "\x1b[36m;1m",
            TerminalColor::BrightWhite => "\x1b[37m;1m",
            TerminalColor::DarkGreen => "\x1b[38;5;22m",
            TerminalColor::LightGreen => "\x1b[38;5;29m",
            TerminalColor::PaleCyan => "\x1b[38;5;30m",
            TerminalColor::PaleGreen => "\x1b[38;5;35m",
            TerminalColor::PalePurple => "\x1b[38;5;53m",
            TerminalColor::Purple => "\x1b[38;5;55m",
            TerminalColor::BrightPurple => "\x1b[38;5;57m",
            TerminalColor::Brown => "\x1b[38;5;94m",
            TerminalColor::PaleYellow => "\x1b[38;5;100m",
            TerminalColor::LightBlue => "\x1b[38;5;110m",
            TerminalColor::NeonGreen => "\x1b[38;5;118m",
            TerminalColor::PaleOrange => "\x1b[38;5;130m",
            TerminalColor::Pink => "\x1b[38;5;177m",
            TerminalColor::DarkOrange => "\x1b[38;5;202m",
            TerminalColor::LightGrey => "\x1b[38;5;235m",
            TerminalColor::PaleGrey => "\x1b[38;5;240m",
            TerminalColor::Grey => "\x1b[38;5;250m",
        }
    }

    pub fn reset() -> &'static str {
        "\x1b[0m"
    }
}
