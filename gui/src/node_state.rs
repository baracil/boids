use crate::node::{DirtyFlags, Background, Border};
use std::rc::Rc;

pub struct NodeState {
    pub dirty_flags: DirtyFlags,
    pub background: Option<Rc<dyn Background>>,
    pub border: Option<Rc<dyn Border>>,
}

impl NodeState {
    pub fn new() -> Self {
        Self { dirty_flags: DirtyFlags::ALL, background: None, border: None }
    }
}
