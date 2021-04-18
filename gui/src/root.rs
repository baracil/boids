use crate::widget_data::{WidgetBase, WidgetData};

pub struct Root {
    data: WidgetData,
}

impl Root {
    pub fn new() -> Self {
        Root {
            data: WidgetData::new(),
        }
    }
}

impl WidgetBase for Root {
    fn widget_data(&self) -> &WidgetData {
        &self.data
    }

    fn widget_data_mut(&mut self) -> &mut WidgetData {
        &mut self.data
    }
}
