use std::collections::HashMap;
use std::rc::Rc;

use raylib::{RaylibHandle, RaylibThread};
use uuid::Uuid;

use tree::tree::{create_tree, RefRegistry, Tree, TreeBase, RefNode, create_tree_node};

use crate::font::FontInfo;
use crate::widget::Widget;
use raylib::prelude::{FontLoadEx, Color, Vector2};
use raylib::drawing::RaylibDrawHandle;
use std::cell::RefCell;
use crate::widget_operation::{LayoutableWidget, RenderableWidget, Size};
use crate::label::LabelPar;
use crate::widget::Widget::Label;

pub trait Gui : Tree<Widget> {
    /// Load a font and save it internally. Returns
    /// an id to get the associated font int
    fn load_font(
        &mut self,
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
        font_file: &str,
        font_size: i32,
        nb_chars: i32,
    ) -> Result<Uuid, String>;

    /// Return the font information associated with the provided id.
    fn get_font(&self, font_id: &Uuid) -> Option<FontInfo>;

    /// Create a Label
    fn create_label(&mut self, f:impl Fn(&mut LabelPar) -> ()) -> RefNode<Widget>;

    fn layout(&mut self);

    fn render(&self, d:&mut RaylibDrawHandle);
}

pub fn create_gui() -> impl Gui {
    return InnerGui::new();
}

pub struct InnerGui {
    gui_data: Rc<RefCell<GuiData>>,
    tree: TreeBase<Widget>,
}

impl InnerGui {
    pub fn new() -> InnerGui {
        return InnerGui {
            tree: create_tree(),
            gui_data: Rc::new(RefCell::new(GuiData{fonts:HashMap::new()}))
        };
    }
}

impl Tree<Widget> for InnerGui {
    fn registry(&self) -> RefRegistry<Widget> {
        self.tree.registry()
    }

    fn root(&self) -> Option<RefNode<Widget>> {
        self.tree.root()
    }

    fn set_root(&mut self, root: RefNode<Widget>) {
        self.tree.set_root(root.clone());
    }
}

pub struct GuiData {
    fonts: HashMap<Uuid, FontInfo>
}
impl GuiData {
    pub fn measure_text(&self, font_id:&Uuid, text: &str, spacing: f32) -> Size {
        if let Some(fi) = self.fonts.get(font_id) {
            return fi.measure_text(text,spacing);
        }
        Size::empty()
    }

    pub fn draw_text(&self, d:&mut RaylibDrawHandle,font_id:&Uuid,text:&str, position:&Vector2, spacing:f32, color:Color) {
        if let Some(fi) = self.fonts.get(font_id) {
            fi.draw_text(d,text,position,spacing,color);
        }
    }
}


impl Gui for InnerGui {

    fn load_font(
        &mut self,
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
        font_file: &str,
        size: i32,
        nb_chars: i32,
    ) -> Result<Uuid, String> {
        let result = rl.load_font_ex(thread, font_file, size, FontLoadEx::Default(nb_chars));

        result.and_then(|font| -> Result<Uuid, String> {
            let uuid = Uuid::new_v4();
            self.gui_data.borrow_mut().fonts.insert(
                uuid.clone(),
                FontInfo {
                    font: Rc::new(font),
                    size,
                },
            );
            Ok(uuid)
        })
    }

    fn get_font(&self, font_id: &Uuid) -> Option<FontInfo> {
        self.gui_data.borrow_mut().fonts
            .get(font_id)
            .and_then(|f| -> Option<FontInfo> { Some(f.clone()) })
    }
    fn create_label(&mut self, f: impl Fn(&mut LabelPar)) -> RefNode<Widget> {
        let mut par = LabelPar::new(self.gui_data.clone());
        f(&mut par);
        let l = create_tree_node(Label(par));
        self.add_node(l.clone());
        l
    }


    fn layout(&mut self) {
        if let Some(p) = self.tree.root() {
            RefCell::borrow_mut(&p).layout()
        }
    }

    fn render(&self, d: &mut RaylibDrawHandle<'_>) {
        if let Some(p) = self.tree.root() {
            RefCell::borrow(&p).render(d)
        }
    }

}
