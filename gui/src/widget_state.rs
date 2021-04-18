use std::rc::Rc;

use crate::widget_operation::{Background, Border, DirtyFlags};

pub struct WidgetState {
    pub dirty_flags: DirtyFlags,
    pub background: Option<Rc<dyn Background>>,
    pub border: Option<Rc<dyn Border>>,
}

impl WidgetState {
    pub fn new() -> Self {
        Self {
            dirty_flags: DirtyFlags::ALL,
            background: None,
            border: None,
        }
    }

    pub fn unset_dirty_flag(&mut self, flag: DirtyFlags) -> bool {
        let mut dirty = self.dirty_flags;
        if (dirty & flag).is_empty() {
            return true;
        }
        dirty.remove(DirtyFlags::STYLE);
        return false;
    }
}
