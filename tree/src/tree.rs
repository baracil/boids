use std::borrow::Borrow;
use std::cell::{RefCell, Ref};
use std::collections::HashMap;
use std::rc::Rc;

use uuid::Uuid;

pub type RefNode<T> = Rc<RefCell<T>>;
pub type Registry<T> = HashMap<Uuid,RefNode<T>>;
pub type RefRegistry<T> = Rc<RefCell<Registry<T>>>;

pub trait Tree<T> where T : TreeNode<T> {
    fn registry(&self) -> RefRegistry<T>;

    fn root(&self) -> Option<RefNode<T>>;

    fn add_node(&mut self, node:RefNode<T>) {
        node.borrow_mut().tree_data().borrow_mut().tree_registry = Some(self.registry());
        self.registry().borrow_mut().insert(node.borrow_mut().id().clone(),node.clone());
    }

    fn set_root(&mut self, root: RefNode<T>);
}

pub fn create_tree_node<T>(value:T) -> RefNode<T> {
    Rc::new(RefCell::new(value))
}

pub fn create_tree<T>() -> TreeBase<T> where T : TreeNode<T> {
    return TreeBase::new();
}

pub struct TreeBase<T> where T: TreeNode<T> {
    registry: RefRegistry<T>,
    root: Option<RefNode<T>>
}

impl<T> Tree<T> for TreeBase<T> where T : TreeNode<T> {
    fn registry(&self) -> RefRegistry<T> {
        self.registry.clone()
    }

    fn root(&self) -> Option<RefNode<T>> {
        self.root.clone()
    }

    fn set_root(&mut self, root: RefNode<T>) {
        self.root = Some(root);
    }
}

impl<T> TreeBase<T> where T: TreeNode<T> {
    fn new() -> Self {
        Self { registry: Rc::new(RefCell::new(HashMap::new())), root:None }
    }
}

pub trait TreeDataProvider<T> where T: TreeNode<T> {
    fn tree_data(&self) -> Rc<RefCell<TreeData<T>>>;
}

pub trait TreeNode<T> : TreeDataProvider<T> where T: TreeNode<T> {
    fn id(&self) -> Uuid;
    fn parent(&self) -> Option<RefNode<T>>;
    fn detach(&mut self);
    fn add_child(&mut self, child: RefNode<T>) -> Result<RefNode<T>, String>;
}


pub struct TreeData<T> where T: TreeNode<T> {
    id: Uuid,
    tree_registry: Option<RefRegistry<T>>,
    parent_id: Option<Uuid>,
    children_id: Vec<Uuid>,
}

impl<T> TreeData<T> where T: TreeNode<T> {
    pub fn new() -> Self {
        TreeData { tree_registry: None, id: Uuid::new_v4(), parent_id: None, children_id: vec![] }
    }

    fn get_registry(&self) -> RefRegistry<T> {
        self.tree_registry.as_ref().unwrap().clone()
    }

    fn get_node(&self, id:&Uuid) -> Option<RefNode<T>> {
        match &self.tree_registry {
            None => panic!("Node not attached to a tree"),
            Some(p) => {
                let a:&RefCell<Registry<T>> = p.borrow();
                a.borrow().get(id).cloned()
            }
        }
    }

    fn get_parent(&self) -> Option<RefNode<T>> {
        let registry = self.get_registry();
        match &self.parent_id {
            None => None,
            Some(s) => {
                let a:&RefCell<Registry<T>> = registry.borrow();
                a.borrow().get(s).cloned()
            }
        }
    }

    fn set_parent_id(&mut self, parent_id: Uuid) {
        self.parent_id = Some(parent_id);
    }

    fn add_child(&mut self, child_id: Uuid) -> bool {
        let child = self.get_node(&child_id);
        match child {
            None => false,
            Some(rc) => {
                rc.borrow_mut().detach();
                rc.borrow_mut().tree_data().borrow_mut().set_parent_id(self.id);
                self.children_id.push(child_id.clone());
                true
            }
        }
    }

    fn detach(&mut self) {
        match &self.parent_id {
            None => {}
            Some(p) => {
                match self.get_node(p) {
                    None => {}
                    Some(np) => {
                        np.borrow_mut().tree_data().borrow_mut().children_id.retain(|x| -> bool { *x != self.id });
                        self.parent_id = None;
                    }
                }
            }
        }
    }
}


impl<T: TreeDataProvider<T>> TreeNode<T> for T {
    fn id(&self) -> Uuid {
        RefCell::borrow(&self.tree_data()).borrow().id
    }

    fn parent(&self) -> Option<Rc<RefCell<T>>> {
        RefCell::borrow(&self.tree_data()).borrow().get_parent()
    }

    fn detach(&mut self) {
        self.tree_data().borrow_mut().detach();
    }

    fn add_child(&mut self, child: RefNode<T>) -> Result<RefNode<T>, String> {
        let ref_cell_node: &RefCell<T> = child.borrow();
        if self.tree_data().borrow_mut().add_child(ref_cell_node.borrow().id()) {
            return Ok(child);
        }
        Err(String::from("This child is not part of the tree"))
    }
}
