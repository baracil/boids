use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use std::process::id;
use uuid::Uuid;

type RefNode<T> = Rc<RefCell<T>>;
type Registry<T> = HashMap<Uuid,RefNode<T>>;
type RefRegistry<T> = Rc<RefCell<Registry<T>>>;

pub struct Tree<T> where T: TreeNode<T> {
    registry: RefRegistry<T>,
}

impl<T> Tree<T> where T: TreeNode<T> {
    pub fn new() -> Self {
        Self { registry: Rc::new(RefCell::new(HashMap::new())) }
    }

    pub fn add_node(&mut self, node:RefNode<T>) {
        assert!(node.borrow().tree_data().tree.is_none());
        node.borrow_mut().tree_data_mut().tree = Some(self.registry.clone());

        self.registry.borrow_mut().insert(node.borrow().id().clone(),node.clone());
    }
}

pub trait TreeDataProvider<T> where T: TreeNode<T> {
    fn tree_data(&self) -> &TreeData<T>;
    fn tree_data_mut(&mut self) -> &mut TreeData<T>;
}

pub trait TreeNode<T>: TreeDataProvider<T> where T: TreeNode<T> {
    fn id(&self) -> &Uuid;
    fn parent(&self) -> Option<RefNode<T>>;
    fn add_child<'a, N>(&mut self, child: &'a mut N) -> Result<&'a mut N, String> where N: TreeNode<T>;
    fn detach(&mut self);
}


pub struct TreeData<T> where T: TreeNode<T> {
    id: Uuid,
    tree: Option<RefRegistry<T>>,
    parent_id: Option<Uuid>,
    children_id: Vec<Uuid>,
}

impl<T> TreeData<T> where T: TreeNode<T> {
    pub fn new() -> Self {
        TreeData { tree: None, id: Uuid::new_v4(), parent_id: None, children_id: vec![] }
    }

    fn get_registry(&self) -> RefRegistry<T> {
        self.tree.as_ref().unwrap().clone()
    }

    fn get_node(&self, id:&Uuid) -> Option<RefNode<T>> {
        self.tree.as_ref().unwrap().borrow().get(id).cloned()
    }

    fn get_parent(&self) -> Option<RefNode<T>> {
        let registry = self.get_registry();
        match &self.parent_id {
            None => None,
            Some(s) => registry.borrow().get(s).cloned()
        }
    }

    fn set_parent_id(&mut self, parent_id: Uuid) {
        self.parent_id = Some(parent_id);
    }

    fn add_child(&mut self, child_id: &Uuid) -> bool {
        let child = self.get_node(child_id);
        match child {
            None => false,
            Some(rc) => {
                rc.borrow_mut().detach();
                rc.borrow_mut().tree_data_mut().set_parent_id(self.id);
                self.children_id.push(child_id.clone());
                true
            }
        }
    }

    fn detach(&mut self) {
        match &self.parent_id {
            None => {}
            Some(p) => {
                match self.get_registry().borrow().get(p) {
                    None => {}
                    Some(np) => {
                        np.borrow_mut().tree_data_mut().children_id.retain(|x| -> bool { *x != self.id });
                        self.parent_id = None;
                    }
                }
            }
        }
    }
}


impl<T: TreeNode<T>> TreeNode<T> for T {
    fn id(&self) -> &Uuid {
        &self.tree_data().id
    }

    fn parent(&self) -> Option<Rc<RefCell<T>>> {
        self.tree_data().get_parent()
    }

    fn add_child<'a, N>(&mut self, child: &'a mut N) -> Result<&'a mut N, String> where N: TreeNode<T> {
        if self.tree_data_mut().add_child(child.id()) {
            return Ok(child);
        }
        Err(String::from("This child is not part of the tree"))
    }

    fn detach(&mut self) {
        self.tree_data_mut().detach();
    }
}
