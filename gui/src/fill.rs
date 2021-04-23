use crate::fill::Fill::Disabled;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum  Fill {
    Disabled,
    Enabled(u32),
}

impl Fill {

    pub fn is_disabled(&self) -> bool {
        matches!(self,Disabled)
    }
}
