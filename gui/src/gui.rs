use std::collections::HashMap;
use std::hint::unreachable_unchecked;
use std::path::Prefix::UNC;
use std::rc::Rc;

use raylib::{RaylibHandle, RaylibThread};
use raylib::core::text::Font;
use raylib::text::FontLoadEx;
use uuid::Uuid;

use crate::font::FontInfo;
use crate::node::Node;

pub trait Gui<'a> :  {
    /// Load a font and save it internally. Returns
    /// an id to get the associated font int
    fn load_font(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread, font_file: &str, size: i32, nb_chars: i32) -> Result<String, String>;

    /// Return the font information associated with the provided id.
    fn get_font(&self, font_id: &str) -> Option<FontInfo>;

    /// Create a Label
    fn create_label(&mut self, text: &str, font_id: &str) -> &'a mut dyn Node;

}

pub fn create_gui<'a>() -> impl Gui<'a> {
    return InnerGui::new();
}


struct InnerGui<'a> {
    root: Option<&'a dyn Node>,
    fonts: HashMap<String, FontInfo>,
}

impl InnerGui<'_> {
    pub fn new<'a>() -> InnerGui<'a> {
        return InnerGui {
            root: None,
            fonts: HashMap::new(),
        };
    }
}

impl<'a> Gui<'a> for InnerGui<'a> {
    fn load_font(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread, font_file: &str, size: i32, nb_chars: i32) -> Result<String, String> {
        let result = rl.load_font_ex(thread, font_file, size, FontLoadEx::Default(nb_chars));

        result.and_then(|font| -> Result<String, String> {
            let uuid = Uuid::new_v4().to_string();
            self.fonts.insert(uuid.clone(), FontInfo { font: Rc::new(font), size });
            Ok(uuid.to_string())
        })
    }

    fn get_font(&self, font_id: &str) -> Option<FontInfo> {
        self.fonts.get(font_id).and_then(|f| -> Option<FontInfo> {
            Some(f.clone())
        })
    }

    fn create_label(&mut self, text: &str, font_id: &str) -> &'a mut dyn Node {
        todo!()
    }
}