use std::collections::HashMap;
use std::rc::{Rc, Weak};

use raylib::{RaylibHandle, RaylibThread};
use uuid::Uuid;

use crate::font::FontInfo;
use crate::widget::Widget;
use raylib::prelude::{FontLoadEx, Color, Vector2};
use raylib::drawing::RaylibDrawHandle;
use std::cell::{RefCell, Cell};
use crate::widget_operation::{LayoutableWidget, RenderableWidget, Size};
use crate::label::LabelPar;
use crate::widget::Widget::{Label, Pane};
use crate::pane::PanePar;
use ego_tree::Tree;
use generational_arena::{Arena, Index};
use raylib::core::text::Font;
use crate::widget_data::{WidgetData, WidgetDataProvider};

pub type RefGuiData = Rc<RefCell<GuiData>>;

pub struct Gui {
    gui_data: RefGuiData,
    tree: Tree<Widget>,
}


impl Gui {
    pub fn new(root_creator: impl FnOnce(RefGuiData) -> Widget) -> Gui {
        let gui_data = Rc::new(RefCell::new(GuiData::new()));
        let root = root_creator(gui_data.clone());
        return Gui {
            tree: Tree::new(root),
            gui_data,
        };
    }

    ///
    pub fn load_font(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread, font_file: &str, size: i32, nb_chars: i32) -> Result<Index, String> {
        let result = rl.load_font_ex(thread, font_file, size, FontLoadEx::Default(nb_chars));


        result.and_then(|font| {
            let idx = self.gui_data.borrow_mut().fonts.insert_with(|idx| { FontInfo::new(font, size) });
            Ok(idx)
        })
    }

    ///
    pub fn get_font(&self, font_id: Index) -> Option<FontInfo> {
        self.gui_data.borrow_mut().fonts.get(font_id).cloned()
    }

    ///
    pub fn create_label(&mut self, f: impl FnOnce(&mut LabelPar)) -> Widget {
        let mut par = LabelPar::new(self.gui_data.clone());
        f(&mut par);
        let node = Label(par);
        node
    }

    pub fn create_pane(&mut self, f: impl FnOnce(&mut PanePar)) -> Widget {
        let mut par = PanePar::new(self.gui_data.clone());
        f(&mut par);
        Pane(par)
    }


    pub fn layout(&mut self, available_size: &Size) {
        let mut root = self.tree.root_mut();
        root.value().layout(available_size);
    }

    pub fn render(&self, d: &mut RaylibDrawHandle<'_>) {
        let root = self.tree.root();
        root.value().render(d);
    }
}


pub struct GuiData {
    fonts: Arena<FontInfo>
}

impl GuiData {
    pub fn new() -> Self {
        Self {
            fonts: Default::default()
        }
    }

    pub fn measure_text(&self, font_id: Index, text: &str, spacing: f32) -> Size {
        if let Some(fi) = self.fonts.get(font_id) {
            return fi.measure_text(text, spacing);
        }
        Size::empty()
    }

    pub fn draw_text(&self, d: &mut RaylibDrawHandle, font_id: Index, text: &str, position: &Vector2, spacing: f32, color: Color) {
        if let Some(fi) = self.fonts.get(font_id) {
            fi.draw_text(d, text, position, spacing, color);
        }
    }
}
