use std::cell::Cell;
use std::ops::BitAnd;
use std::ops::Deref;

use generational_arena::Index;
use raylib::prelude::*;
use std::ops::Add;
use crate::alignment::{HAlignment, VAlignment};
use crate::fill::Fill;
use crate::fill::Fill::{Disabled, Enabled};
use crate::gui::{Gui};
use crate::mouse::MouseState;
use crate::padding::Padding;
use crate::size::Size;
use crate::widget::Widget;
use crate::widget_geometry::WidgetGeometry;
use crate::widget_model::WidgetModel;
use crate::widget_operation::{DirtyFlags, UpdatableWidget, WidgetOp, LayoutableWidget, WidgetSpecific, WidgetDataProvider};
use crate::widget_state::WidgetState;
use crate::background::BackgroundRenderer;
use crate::border::BorderRenderer;
use crate::position::Coordinate;
use vec_tree::ChildrenIter;
use crate::event::Event::{Click};

pub struct WidgetData {
    pub tree_index: Option<Index>,
    pub state: WidgetState,
    pub geometry: WidgetGeometry,
    pub model: WidgetModel,
}


impl WidgetData {
    pub(crate) fn render_background_and_border(&self, d: &mut RaylibDrawHandle<'_>, offset: &Vector2) {
        let mut chrome_layout = self.geometry.widget_layout.to_owned().into_inner();
        chrome_layout.x += offset.x;
        chrome_layout.y += offset.y;
        {
            let borrowed_background = self.state.background.borrow();
            if let Some(background) = borrowed_background.as_deref() {
                let armed = self.state.armed.get();
                let hoovered = self.state.hoovered.get();
                let child_hoovered = self.state.child_hoovered.get();
                background.draw(d, &chrome_layout, hoovered && !child_hoovered, armed)
            }
        }
        {
            let borrowed_border = &self.state.border.borrow();
            if let Some(border) = borrowed_border.as_deref() {
                border.draw(d, &chrome_layout)
            }
        }
    }
}

impl WidgetData {
    fn get_parent<'a>(&self, gui: &'a Gui) -> Option<&'a Widget> {
        match self.tree_index {
            None => None,
            Some(idx) => {
                gui.get_parent_widget(idx)
            }
        }
    }

    pub fn compute_target(position: f32, absolute: bool, available: f32) -> f32 {
        if absolute {
            return position;
        }
        position * 0.01 * available
    }

    pub fn is_relative_coordinate_y(&self) -> bool {
        self.model.position.get().is_y_relative()
    }

    pub fn is_relative_coordinate_x(&self) -> bool {
        self.model.position.get().is_x_relative()
    }


    pub fn invalidate_style(&self) {
        self.set_dirty_flag(DirtyFlags::STYLE)
    }

    pub fn invalidate_preferred_size(&self, gui: &Gui) {
        self.invalidate_flag(gui, DirtyFlags::PREFERRED_SIZE);
    }

    pub fn invalidate_content_size(&self, gui: &Gui) {
        self.invalidate_flag(gui, DirtyFlags::CONTENT_SIZE);
    }

    pub fn invalidate_position(&self, gui: &Gui) {
        self.invalidate_flag(gui, DirtyFlags::POSITION);
    }

    fn invalidate_flag(&self, gui: &Gui, flag: DirtyFlags) {
        if self.is_dirty_flag_set(flag) {
            return;
        }
        self.set_dirty_flag(flag);
        if let Some(parent) = self.get_parent(gui) {
            parent.widget_data().invalidate_flag(gui, flag)
        }
    }

    pub fn update_style(&self, gui: &Gui) {
        if self.dirty_flag_clean(DirtyFlags::STYLE) {
            return;
        }

        self.update_text_style(gui);
        self.update_background(gui);
        self.update_border(gui);

        self.invalidate_preferred_size(gui)
    }

    fn update_text_style(&self, gui: &Gui) {
        let borrowed = self.model.text_style_name.borrow();
        let text_style = gui.get_text_style(borrowed.deref());
        self.state.text_style.replace(text_style);
    }

    fn update_background(&self, gui: &Gui) {
        let borrowed = self.model.back_style_name.borrow();
        let background = gui.get_background(borrowed.deref());
        self.state.background.replace(background);
    }

    fn update_border(&self, gui: &Gui) {
        let borrowed = self.model.border_style_name.borrow();
        let border = gui.get_border(borrowed.deref());
        self.state.border.replace(border);
    }

    pub fn is_fill_height(&self) -> bool {
        self.model.fill_height.get().is_enabled()
    }
    pub fn is_fill_width(&self) -> bool {
        self.model.fill_width.get().is_enabled()
    }
}

impl WidgetData {
    pub fn new() -> Self {
        Self {
            tree_index: None,
            state: WidgetState::new(),
            geometry: WidgetGeometry::new(),
            model: WidgetModel::new(),
        }
    }

    pub fn set_alignment(&self, gui: &Gui, valignment: VAlignment, haligment: HAlignment) {
        let mut current_alignment = self.model.alignment.get();
        if current_alignment.vertical.eq(&valignment) && current_alignment.horizontal.eq(&haligment) {
            return;
        }
        current_alignment.vertical = valignment;
        current_alignment.horizontal = haligment;
        self.model.alignment.set(current_alignment);
        self.invalidate_position(gui)
    }

    pub fn set_tree_index(&mut self, tree_index: Index) {
        self.tree_index = Some(tree_index);
    }

    pub fn clear_tree_index(&mut self) {
        self.tree_index = None
    }

    pub fn fill_width(&self) -> Fill {
        self.model.fill_width.get()
    }

    pub fn fill_height(&self) -> Fill {
        self.model.fill_height.get()
    }

    fn disable_fill(&self, gui: &Gui, fill: &Cell<Fill>) {
        if fill.get().is_disabled() {
            return;
        }
        fill.set(Disabled);
        self.invalidate_preferred_size(gui)
    }

    fn enable_fill(&self, gui: &Gui, fill_cell: &Cell<Fill>, fill: Fill) {
        let current_fill = fill_cell.get();
        if current_fill.eq(&fill) {
            return;
        }

        fill_cell.set(fill);
        self.invalidate_preferred_size(gui)
    }

    pub fn update_hoovered(&self, gui: &Gui, offset: &Vector2, mouse_position: &Vector2, mouse_state: &MouseState) -> bool {
        let mut abs_widget_layout = self.geometry.widget_layout.get();
        abs_widget_layout.x += offset.x;
        abs_widget_layout.y += offset.y;

        let new_hoovered = abs_widget_layout.check_collision_point_rec(mouse_position);
        let old_hoovered = self.state.hoovered.get();
        self.state.hoovered.set(new_hoovered);
        let mut child_hoovered = false;

        match (self.tree_index, old_hoovered, new_hoovered) {
            (_, false, false) | (None, _, _) => {}

            (Some(idx), _, _) => {
                let padding = self.model.padding.get();
                let child_offset = Vector2::new(abs_widget_layout.x+padding.left, abs_widget_layout.y+padding.top);
                for child_index in gui.get_widget_children(idx) {
                    if let Some(w) = gui.get_widget(child_index) {
                        child_hoovered |= w.widget_data().update_hoovered(gui, &child_offset, mouse_position, mouse_state)
                    }
                }
            }
        }
        self.state.child_hoovered.set(child_hoovered);
        new_hoovered
    }

    pub fn update_action(&self, gui:&Gui, offset: &Vector2, mouse_position: &Vector2, mouse_state: &MouseState)  {
        let mut armed = self.state.armed.get();
        let clickable = self.model.clickable.get();
        let hoovered = self.state.hoovered.get();

        if mouse_state.left.released && clickable && hoovered {
            let action_id = self.model.action_id.clone().into_inner();
            match (armed, action_id) {
                (true, Some(action_id)) => {
                    gui.add_event(Click{action_id })
                }
                _ => {}
            }
        }

        if mouse_state.left.pressed && hoovered {
            armed = clickable;
        }

        armed &= mouse_state.left.down;

        self.state.armed.set(armed);

        if let Some(idx) = self.tree_index {
            for child_index in gui.get_widget_children(idx) {
                let widget_layout = self.geometry.widget_layout.get();
                let child_offset = Vector2::new(widget_layout.x+offset.x, widget_layout.y+offset.y);
                if let Some(w) = gui.get_widget(child_index) {
                    w.widget_data().update_action(gui,&child_offset,mouse_position,mouse_state);
                }

            }
        }


    }

    pub fn set_dirty_flag(&self, flag: DirtyFlags) {
        self.state.dirty_flags.set(self.state.dirty_flags.get() | flag);
    }

    pub fn is_dirty_flag_set(&self, flag: DirtyFlags) -> bool {
        self.state.dirty_flags.get().bitand(flag).eq(&flag)
    }

    pub fn dirty_flag_clean(&self, flag: DirtyFlags) -> bool {
        self.state.dirty_flag_clean(flag)
    }

    pub fn dirty_flag_dirty(&self, flag: DirtyFlags) -> bool {
        !self.state.dirty_flag_clean(flag)
    }

    pub fn copy_size_to_layout(&self) {
        let borrowed_widget_size = self.geometry.widget_size.borrow();
        let widget_size = borrowed_widget_size.size();

        {
            WidgetGeometry::copy_size(widget_size, &self.geometry.widget_layout);
            let content_size = widget_size.without_padding(&self.model.padding.get());
            WidgetGeometry::copy_size(&content_size, &self.geometry.content_layout);
        }
    }

    pub(crate) fn compute_default_target(&self, available_size: &Size) {
        let position = self.model.position.get().compute_absolute(available_size);
        let offset = self.compute_alignment_offset();

        self.set_widget_target(&position.add(offset));
    }

    pub fn set_widget_target(&self, target: &Vector2) {
        let padding = self.model.padding.get();
        let mut widget_layout = self.geometry.widget_layout.get();
        let mut content_layout = self.geometry.content_layout.get();

        widget_layout.x = target.x;
        widget_layout.y = target.y;

        content_layout.x = target.x + padding.left;
        content_layout.y = target.y + padding.top;

        self.geometry.widget_layout.set(widget_layout);
        self.geometry.content_layout.set(content_layout);
    }

    pub fn compute_alignment_offset(&self) -> Vector2 {
        let borrowed_widget_size = self.geometry.widget_size.borrow();
        let widget_size = borrowed_widget_size.size();
        let alignment = self.model.alignment.get();

        let mut offset = Vector2::default();

        offset.x = widget_size.width() * alignment.horizontal.shift_factor();
        offset.y = widget_size.height() * alignment.vertical.shift_factor();

        offset
    }
}

impl<N: WidgetDataProvider> UpdatableWidget for N {
    fn update_with_mouse_information(&self, gui: &Gui, offset: &Vector2, mouse_position: &Vector2, mouse_state: &MouseState) {
        self.widget_data().update_hoovered(gui, offset, mouse_position, mouse_state);
        self.widget_data().update_action(gui,offset,mouse_position,mouse_state);
    }
}

impl<N: WidgetSpecific + WidgetDataProvider> LayoutableWidget for N {
    fn get_computed_size(&self, gui: &Gui) -> Size {
        if self.widget_data().dirty_flag_dirty(DirtyFlags::PREFERRED_SIZE) {
            let size = self.compute_size(gui);
            self.widget_data().geometry.computed_size.set(size);
            self.widget_data().invalidate_content_size(gui);
            return size;
        }

        return self.widget_data().geometry.computed_size.get();
    }


    fn update_content_size(&self, gui: &Gui, available_space: &Size) {
        let content_invalid = {
            let content_cache = self.widget_data().geometry.widget_size.borrow();
            let clean_flag = self.widget_data().dirty_flag_clean(DirtyFlags::CONTENT_SIZE);
            let cache_valid = available_space.eq(content_cache.reference());
            !clean_flag || !cache_valid
        };

        if content_invalid {
            let mut content_size = self.widget_data().geometry.computed_size.clone().into_inner();

            if let Enabled { .. } = self.widget_data().model.fill_width.get() {
                content_size.set_width(available_space.width())
            }
            if let Enabled { .. } = self.widget_data().model.fill_height.get() {
                content_size.set_height(available_space.height())
            }
            content_size.min_mut(&available_space);


            self.compute_child_content_size(gui, content_size);

            {
                let mut content_cache = self.widget_data().geometry.widget_size.borrow_mut();
                content_cache.set_reference(available_space.clone());
                content_cache.set_size(content_size);
            }

            self.widget_data().copy_size_to_layout();
            self.widget_data().invalidate_position(gui);
        }
    }

    fn update_child_positions(&self, gui: &Gui) {
        if self.widget_data().state.dirty_flag_clean(DirtyFlags::POSITION) {
            return;
        }
        self.compute_child_positions(gui);
    }
}

impl WidgetOp for WidgetData {
    fn set_text_style(&self, text_style_name: &str) -> &dyn WidgetOp {
        self.model.text_style_name.replace(text_style_name.to_string());
        self.invalidate_style();
        self
    }

    fn set_background_style(&self, background_style_name: &str) -> &dyn WidgetOp {
        self.model.back_style_name.replace(background_style_name.to_string());
        self.invalidate_style();
        self
    }

    fn set_action_id(&self, action_id: &str) -> &dyn WidgetOp {
        self.model.action_id.replace(Some(action_id.to_string()));
        self
    }

    fn clear_action_id(&self) -> &dyn WidgetOp {
        self.model.action_id.replace(None);
        self
    }

    fn set_clickable(&self, clickable: bool) -> &dyn WidgetOp {
        self.model.clickable.set(clickable);
        self
    }


    fn set_position(&self, gui: &Gui, x: &Coordinate, y: &Coordinate) -> &dyn WidgetOp {
        let mut current_position = self.model.position.get();
        if current_position.get_x().eq(&x) && current_position.get_y().eq(&y) {
            return self;
        }
        current_position.set_x(x);
        current_position.set_y(y);
        self.model.position.set(current_position);
        self.invalidate_position(gui);
        self
    }

    fn set_valignment(&self, gui: &Gui, valignment: VAlignment) -> &dyn WidgetOp {
        let current_alignment = self.model.alignment.get();
        self.set_alignment(gui, valignment, current_alignment.horizontal);
        self
    }

    fn set_halignment(&self, gui: &Gui, halignment: HAlignment) -> &dyn WidgetOp {
        let current_alignment = self.model.alignment.get();
        self.set_alignment(gui, current_alignment.vertical, halignment);
        self
    }

    fn set_padding(&self, gui: &Gui, padding: Padding) -> &dyn WidgetOp {
        let current_padding = self.model.padding.get();
        if current_padding.eq(&padding) {
            return self;
        }
        self.model.padding.set(padding);
        self.invalidate_preferred_size(gui);
        self
    }

    fn set_preferred_height(&self, gui: &Gui, height: f32) -> &dyn WidgetOp {
        let size = self.model.preferred_size.get().with_height(height);
        self.set_preferred_size(gui, size);
        self
    }

    fn set_preferred_width(&self, gui: &Gui, width: f32) -> &dyn WidgetOp {
        let size = self.model.preferred_size.get().with_width(width);
        self.set_preferred_size(gui, size);
        self
    }

    fn set_preferred_size(&self, gui: &Gui, size: Size) -> &dyn WidgetOp {
        let current = self.model.preferred_size.get();
        if current.eq(&size) {
            return self;
        }
        self.model.preferred_size.set(size);
        self.invalidate_preferred_size(gui);
        self
    }

    fn disable_fill_width(&self, gui: &Gui) -> &dyn WidgetOp {
        self.disable_fill(gui, &self.model.fill_width);
        self
    }

    fn disable_fill_height(&self, gui: &Gui) -> &dyn WidgetOp {
        self.disable_fill(gui, &self.model.fill_height);
        self
    }

    fn enable_fill_width(&self, gui: &Gui, fill: Fill) -> &dyn WidgetOp {
        self.enable_fill(gui, &self.model.fill_width, fill);
        self
    }

    fn enable_fill_height(&self, gui: &Gui, fill: Fill) -> &dyn WidgetOp {
        self.enable_fill(gui, &self.model.fill_height, fill);
        self
    }
}

impl<M: WidgetDataProvider> WidgetOp for M {
    fn set_text_style(&self, text_style_name: &str) -> &dyn WidgetOp {
        self.widget_data().set_text_style(text_style_name)
    }

    fn set_background_style(&self, background_style_name: &str) -> &dyn WidgetOp {
        self.widget_data().set_background_style(background_style_name)
    }

    fn set_action_id(&self, action_id: &str) -> &dyn WidgetOp {
        self.widget_data().set_action_id(action_id)
    }

    fn clear_action_id(&self) -> &dyn WidgetOp {
        self.widget_data().clear_action_id()
    }

    fn set_clickable(&self,clickable:bool) -> &dyn WidgetOp {
        self.widget_data().set_clickable(clickable)
    }


    fn set_position(&self, gui: &Gui, x: &Coordinate, y: &Coordinate) -> &dyn WidgetOp {
        self.widget_data().set_position(gui, x, y)
    }

    fn set_valignment(&self, gui: &Gui, valignment: VAlignment) -> &dyn WidgetOp {
        self.widget_data().set_valignment(gui, valignment)
    }

    fn set_halignment(&self, gui: &Gui, halignment: HAlignment) -> &dyn WidgetOp {
        self.widget_data().set_halignment(gui, halignment)
    }

    fn set_padding(&self, gui: &Gui, padding: Padding) -> &dyn WidgetOp {
        self.widget_data().set_padding(gui, padding)
    }

    fn set_preferred_height(&self, gui: &Gui, height: f32) -> &dyn WidgetOp {
        self.widget_data().set_preferred_height(gui, height)
    }

    fn set_preferred_width(&self, gui: &Gui, width: f32) -> &dyn WidgetOp {
        self.widget_data().set_preferred_width(gui, width)
    }

    fn set_preferred_size(&self, gui: &Gui, size: Size) -> &dyn WidgetOp {
        self.widget_data().set_preferred_size(gui, size)
    }

    fn disable_fill_width(&self, gui: &Gui) -> &dyn WidgetOp {
        self.widget_data().disable_fill_width(gui)
    }

    fn disable_fill_height(&self, gui: &Gui) -> &dyn WidgetOp {
        self.widget_data().disable_fill_height(gui)
    }

    fn enable_fill_width(&self, gui: &Gui, fill: Fill) -> &dyn WidgetOp {
        self.widget_data().enable_fill_width(gui, fill)
    }

    fn enable_fill_height(&self, gui: &Gui, fill: Fill) -> &dyn WidgetOp {
        self.widget_data().enable_fill_height(gui, fill)
    }
}