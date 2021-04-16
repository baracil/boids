use crate::alignment::VAlignment::Center;
use crate::alignment::HAlignment::Middle;

#[derive(Copy, Clone, PartialEq)]
pub enum VAlignment {
    Top,
    Center,
    Bottom,
}

#[derive(Copy, Clone, PartialEq)]
pub enum HAlignment {
    Left,
    Middle,
    Right,
}

#[derive(Copy, Clone, PartialEq)]
pub struct Alignment {
    vertical_alignment: VAlignment,
    horizontal_alignment: HAlignment,
}

impl Alignment {
    pub fn new() -> Self {
        Self{vertical_alignment:Center, horizontal_alignment:Middle}
    }
}


impl VAlignment {
    pub fn shift_factor(&self) -> f32 {
        match self {
            VAlignment::Top => 0.,
            VAlignment::Center => 0.5,
            VAlignment::Bottom => 1.0,
        }
    }
}

impl HAlignment {
    pub fn shift_factor(&self) -> f32 {
        match self {
            HAlignment::Left => 0.,
            HAlignment::Middle => 0.5,
            HAlignment::Right => 1.0,
        }
    }
}
