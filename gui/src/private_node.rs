use crate::node::{Parent, NodePar, Item, Node, Size};
use raylib::math::Vector2;
use crate::alignment::Alignment;
use crate::mouse::MouseState;
use raylib::drawing::RaylibDrawHandle;

pub struct InnerNode {
    item: Item,
    parent: Option<Box<dyn Node>>,
    children: Vec<Box<dyn Node>>,
}

pub fn get_node_trait_mut(item: &mut Item) -> &mut dyn NodePar {
    match item {
        Item::Button(a) => a,
        Item::Label(a) => a,
    }
}

pub fn get_nodepar_trait(item: &Item) -> &dyn NodePar {
    match item {
        Item::Button(a) => a,
        Item::Label(a) => a
    }
}

impl Parent for InnerNode {
    fn children(&self) -> &[Box<dyn Node>] {
        return self.children.as_slice();
    }
}


impl Node for InnerNode {
    fn parent(&self) -> &Option<Box<dyn Node>> {
        &self.parent
    }

    fn item(&self) -> &Item {
        &self.item
    }

    fn item_mut(&mut self) -> &mut Item {
        &mut self.item
    }
}

//Check for macro derive. this should be better than this
impl NodePar for InnerNode {

    fn content_width(&self) -> f32 {
        get_nodepar_trait(&self.item).content_height()
    }

    fn content_height(&self) -> f32 {
        get_nodepar_trait(&self.item).content_height()
    }

    fn padding(&self) -> f32 {
        get_nodepar_trait(&self.item).padding()
    }

    fn layout(&mut self) {
        get_node_trait_mut(&mut self.item).layout();
    }

    fn update(&mut self, mouse_position: &Vector2, mouse_state: &MouseState) {
        get_node_trait_mut(&mut self.item)
            .update(&mouse_position, &mouse_state);
    }

    fn render(&self, d: &mut RaylibDrawHandle<'_>) {
        get_nodepar_trait(&self.item).render(d);
    }

    fn set_position(&mut self, point: &Vector2, alignment: Alignment) -> &mut dyn NodePar{
        get_node_trait_mut(&mut self.item)
            .set_position(point, alignment)
    }

    fn set_padding(&mut self, padding: f32) -> &mut dyn NodePar {
        get_node_trait_mut(&mut self.item)
            .set_padding(padding)
    }

    fn clear_requested_size(&mut self) -> &mut dyn NodePar {
        get_node_trait_mut(&mut self.item).clear_requested_size()
    }


    fn set_requested_height(&mut self, height: f32) -> &mut dyn NodePar {
        get_node_trait_mut(&mut self.item).set_requested_height(height)
    }

    fn set_requested_width(&mut self, width: f32) -> &mut dyn NodePar {
        get_node_trait_mut(&mut self.item).set_requested_width(width)
    }

    fn set_requested_size(&mut self, size: Size) -> &mut dyn NodePar {
        get_node_trait_mut(&mut self.item).set_requested_size(size)
    }
}
