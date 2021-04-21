use std::collections::HashMap;
use std::rc::{Rc, Weak};

use raylib::{RaylibHandle, RaylibThread};
use uuid::Uuid;

use crate::font::FontInfo;
use crate::widget::Widget;
use raylib::prelude::{FontLoadEx, Color, Vector2};
use raylib::drawing::RaylibDrawHandle;
use std::cell::{RefCell, Cell};
use crate::widget_operation::{LayoutableWidget, RenderableWidget, Size, DirtyFlags};
use crate::label::LabelPar;
use crate::widget::Widget::{Label, Pane};
use crate::pane::PanePar;
use generational_arena::{Arena, Index};
use raylib::core::text::Font;
use crate::widget_data::{WidgetData, WidgetDataProvider, SizeableWidget};
use vec_tree::VecTree;

pub type RefGuiData = Rc<RefCell<GuiData>>;

pub struct Gui {
    gui_data: RefGuiData,
    tree: VecTree<Widget>,
}


impl Gui {
    pub fn new(root_create: impl FnOnce(RefGuiData) -> Widget) -> (Gui, Index) {
        let gui_data = Rc::new(RefCell::new(GuiData::new()));
        let mut tree = VecTree::new();
        let root = root_create(gui_data.clone());
        let root_index = tree.insert_root(root);
        return (Gui {
            tree,
            gui_data,
        }, root_index);
    }

    pub fn insert_root(&mut self, root: Widget) -> Index {
        self.tree.insert_root(root)
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
        if let Some(index) = self.tree.get_root_index() {
            self.layout_rec(index, available_size)
        }
    }

    fn layout_rec(&mut self, mut node: Index, available_size: &Size) {
        let widget = self.tree.get_mut(node).unwrap();

        widget.widget_data_mut().compute_style();

        if !widget.widget_data_mut().dirty_flag_clean(DirtyFlags::CONTENT_SIZE) {
            let size = widget.compute_content_size(available_size);
            widget.widget_data_mut().geometry.content_size = size
        }

        widget.widget_data_mut().compute_item_size();
        widget.widget_data_mut().compute_position();
    }


    pub fn render(&self, d: &mut RaylibDrawHandle<'_>) {
        if let Some(idx) = self.tree.get_root_index() {
            self.render_rec(idx, d)
        }
    }

    pub fn render_rec(&self, mut node: Index, d: &mut RaylibDrawHandle<'_>) {
        let widget = self.tree.get(node).unwrap();
        widget.render(d)
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
