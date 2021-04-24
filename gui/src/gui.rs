use std::collections::HashMap;
use std::rc::Rc;

use generational_arena::{Index};
use raylib::prelude::*;
use vec_tree::{ChildrenIter, VecTree};

use crate::background::{Background};
use crate::border::{Border};
use crate::font::FontInfo;
use crate::size::Size;
use crate::text_style::TextStyle;
use crate::widget::Widget;
use crate::widget_operation::{RenderableWidget, LayoutableWidget, WidgetDataProvider};

pub struct Gui {
    data: GuiData,
    tree: VecTree<Widget>,
}


pub struct GuiData {
    fonts: HashMap<String,Rc<FontInfo>>,
    text_styles: HashMap<String,Rc<TextStyle>>,
    background: HashMap<String,Rc<Background>>,
    border: HashMap<String,Rc<Border>>,
}

impl GuiData {

}


impl Gui {
    pub fn new() -> Gui {
        let tree = VecTree::new();
        return Gui {
            data: GuiData {
                fonts:HashMap::new(),
                text_styles:HashMap::new(),
                background:HashMap::new(),
                border:HashMap::new()
            },
            tree,
        };
    }
    pub fn get_text_style(&self, text_style_name:&String) -> Option<Rc<TextStyle> >{
        self.data.text_styles.get(text_style_name).cloned()
    }
    pub fn get_background(&self, background_name:&String) -> Option<Rc<Background> >{
        self.data.background.get(background_name).cloned()
    }
    pub fn get_border(&self, border_name:&String) -> Option<Rc<Border> >{
        self.data.border.get(border_name).cloned()
    }


    /// tree management
    pub fn get_parent(&self, node_id:Index) -> Option<Index> {
        self.tree.parent(node_id)
    }
    pub fn get_parent_widget(&self, node_id:Index) -> Option<&Widget> {
        match self.tree.parent(node_id) {
            None => None,
            Some(w) => self.get_node(w)
        }
    }
    pub fn get_widget_children(&self, node_id:Index) -> ChildrenIter<'_, Widget> {
        self.tree.children(node_id)
    }
    pub fn get_widget(&self, node_id:Index) -> Option<&Widget> {
        self.tree.get(node_id)
    }
    fn get_node(&self, node_idx: Index) -> Option<&Widget> {
        self.tree.get(node_idx)
    }
    pub fn insert_root(&mut self, root: Widget) -> Index {
        let root_index = self.tree.insert_root(root);
        self.tree.get_mut(root_index)
            .unwrap()
            .widget_data_mut().tree_index = Some(root_index);
        root_index
    }
    pub fn add_child(&mut self, parent:Index, child:Widget) -> Index {
        let child_index = self.tree.insert(child, parent);
        self.tree.get_mut(child_index).unwrap().widget_data_mut().tree_index = Some(child_index);
        child_index
    }

    /// font management
    pub fn load_font(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread, font_name:&str, font_file: &str, size: i32, nb_chars: i32) -> Result<String, String> {
        if self.data.fonts.contains_key(font_file) {
            return Err(format!("A font with the name '{}' exists already",font_name));
        }

        let result = rl.load_font_ex(thread, font_file, size, FontLoadEx::Default(nb_chars));

        result.and_then(|font| {
            let owned_font_name = font_name.to_owned();
            self.data.fonts.insert(owned_font_name.clone(), Rc::new(FontInfo{font:Rc::new(font),size:size as f32}));
            Ok(owned_font_name)
        })
    }
    pub fn get_font(&self, font_name: &str) -> Option<Rc<FontInfo>> {
        self.data.fonts.get(font_name).cloned()
    }

    pub fn add_text_style(&mut self, text_style_name:&str, font_name:&str, color:Color, spacing:f32) {
        if let Some(rc_font) = self.get_font(font_name) {
            let text_style = TextStyle::new(rc_font, color, spacing);
            self.data.text_styles.insert(text_style_name.to_string(), Rc::new(text_style));
        }
    }

    pub fn add_border(&mut self, border_name:&str, border:Border) {
        self.data.border.insert(border_name.to_string(), Rc::new(border));
    }

    pub fn add_background(&mut self, background_name:&str, background:Background) {
        self.data.background.insert(background_name.to_string(), Rc::new(background));
    }

    /// layout & rendering
    pub fn layout(&self, available_size: &Size) {
        let option_root = self.tree.get_root_index();
        if option_root.is_none() {
            return;
        }
        let root_index = option_root.unwrap();

        self.update_styles(root_index);
        self.update_computed_size(root_index);
        self.update_content_size(root_index,&available_size);
        self.update_widget_positions(root_index,&available_size)
    }
    fn update_styles(&self, root_index:Index) {
        self.tree.descendants(root_index).for_each(|idx| {
            if let Some(w) = self.tree.get(idx) {
                w.widget_data().update_style(self)
            }
        })
    }

    fn update_computed_size(&self, root_index: Index) {
        let root = self.get_node(root_index).unwrap();
        root.get_computed_size(&self);
    }


    fn update_content_size(&self, root_index: Index, available_size: &Size) {
        let root = self.get_node(root_index).unwrap();
        root.update_content_size(&self, available_size);
    }

    fn update_widget_positions(&self, root_index: Index, available_space:&Size) {
        let root = self.get_node(root_index).unwrap();
        root.widget_data().compute_default_target(available_space);
        root.update_child_positions(&self);
    }
    pub fn render(&self, d: &mut RaylibDrawHandle<'_>, position:&Vector2, available_space:&Size) {
        if let Some(idx) = self.tree.get_root_index() {
            let root = self.tree.get(idx).unwrap();
            root.render(&self, d, position, available_space)
        }
    }

}

