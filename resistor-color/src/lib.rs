use int_enum::IntEnum;
use std::fmt::{self, Debug};
use enum_iterator::{all, Sequence};

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntEnum, Sequence)]
pub enum ResistorColor {
    Black = 0,
    Brown = 1,
    Red = 2,
    Orange = 3,
    Yellow = 4,
    Green = 5,
    Blue = 6,
    Violet = 7,
    Grey = 8,
    White = 9,
}

impl fmt::Display for ResistorColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

pub fn color_to_value(_color: ResistorColor) -> usize {
    return _color.int_value() as usize;
}

pub fn value_to_color_string(value: usize) -> String {
    let color_value = ResistorColor::from_int(value as u8);
    let enum_val = match color_value {
        Ok(color_value) => color_value.to_string(),
        Err(_e) => String::from("value out of range"),
    };
    return enum_val.to_string();
}

pub fn colors() -> Vec<ResistorColor> {
    all::<ResistorColor>().collect::<Vec<_>>()
}
