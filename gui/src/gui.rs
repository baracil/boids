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
use crate::widget_data::{SizeableWidget, WidgetDataProvider};
use crate::widget_operation::{RenderableWidget};

pub struct Gui {
    data: GuiData,
    tree: VecTree<Widget>,
}


pub struct GuiData {
    fonts: HashMap<String,FontInfo>,
    text_styles: HashMap<String,TextStyle>,
    background: HashMap<String,Background>,
    border: HashMap<String,Border>,
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

    pub fn get_parent(&self, node_id:Index) -> Option<Index> {
        self.tree.parent(node_id)
    }

    pub fn get_parent_widget(&self, node_id:Index) -> Option<&Widget> {
        match self.tree.parent(node_id) {
            None => None,
            Some(w) => self.get_node(w)
        }
    }

    pub fn get_children(&self, node_id:Index) -> ChildrenIter<'_, Widget> {
        self.tree.children(node_id)
    }

    pub fn insert_root(&mut self, root: Widget) {
        let root_index = self.tree.insert_root(root);
        self.tree.get_mut(root_index)
            .unwrap()
            .widget_data_mut().tree_index = Some(root_index);
    }

    ///
    pub fn load_font(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread, font_name:&str, font_file: &str, size: i32, nb_chars: i32) -> Result<String, String> {
        if self.data.fonts.contains_key(font_file) {
            return Err(format!("A font with the name '{}' exists already",font_name));
        }

        let result = rl.load_font_ex(thread, font_file, size, FontLoadEx::Default(nb_chars));

        result.and_then(|font| {
            let owned_font_name = font_name.to_owned();
            self.data.fonts.insert(owned_font_name.clone(), FontInfo{font:Rc::new(font),size:size as f32});
            Ok(owned_font_name)
        })
    }

    ///
    pub fn get_font(&self, font_name: String) -> Option<FontInfo> {
        self.data.fonts.get(&font_name).cloned()
    }

    fn get_node(&self, node_idx: Index) -> Option<&Widget> {
        self.tree.get(node_idx)
    }


    pub fn layout(&mut self, available_size: &Size) {
        let option_root = self.tree.get_root_index();
        if option_root.is_none() {
            return;
        }
        let root_index = option_root.unwrap();

        self.update_content_size(root_index,&available_size);
        self.update_widget_size(root_index);
    }

    fn update_content_size(&mut self, root_index: Index, available_size: &Size) {
        let root = self.get_node(root_index).unwrap();
        root.update_content_size(&self, available_size);
    }

    fn update_widget_size(&self, root_index: Index) {
        let root = self.get_node(root_index).unwrap();
        root.widget_data().update_widget_size(&self);
    }

    pub fn render(&self, d: &mut RaylibDrawHandle<'_>, position:Vector2) {
        if let Some(idx) = self.tree.get_root_index() {
            let root = self.tree.get(idx).unwrap();
            root.render(&self, d, position)
        }
    }

}

