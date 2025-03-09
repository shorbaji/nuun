use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};

use crate::object::Object;
use crate::path::Path;

pub struct SymbolTable {
    parent: Option<Rc::<RefCell<SymbolTable>>>,
    table: HashMap<String, Rc<RefCell<Object>>>,
    children: HashMap<String, Weak<RefCell<SymbolTable>>>,
    path: Path,
}

impl SymbolTable {
    pub fn new(parent: Option<Rc<RefCell<SymbolTable>>>, path: Path  ) -> Self {
        SymbolTable {
            parent,
            table: HashMap::new(),
            children: HashMap::new(),
            path,
        }
    }

    pub fn get(&self, key: &str) -> Option<Rc<RefCell<Object>>> {
        match self.table.get(key) {
            Some(value) => Some(value.clone()),
            None => match &self.parent {
                Some(parent) => parent.borrow().get(key),
                None => None,
            },
        }
    }

    pub fn set(&mut self, key: &str, value: Rc<RefCell<Object>>) {
        self.table.insert(key.to_string(), value);
    }

    pub fn make_node(self_ref: Rc<RefCell<Self>>, path: Path) -> Option<Weak<RefCell<SymbolTable>>> {
        let mut current = self_ref.clone();
        
        for key in path.as_vector() {
            current = current.clone().borrow().get_child(key.as_str())
            .and_then(|child| Some(child.upgrade().unwrap()))
            .unwrap_or(
                SymbolTable::new_child(current.clone(), key.as_str()).unwrap().upgrade().unwrap()
            );
        }

        return Some(Rc::<RefCell<SymbolTable>>::downgrade(&current));
    }

    pub fn new_child(self_ref: Rc<RefCell<Self>>, key: &str) -> Option<Weak<RefCell<SymbolTable>>>{
        let path = self_ref.borrow().path.clone() + key;

        let parent = Some(self_ref.clone());
    
        let child = SymbolTable::new(
            parent,
            path,
        );

        self_ref.borrow_mut().children.insert(key.to_string(), Rc::downgrade(&Rc::new(RefCell::new(child))))
    }

    pub fn get_child(&self, key: &str) -> Option<Weak<RefCell<SymbolTable>>> {
        match self.children.get(key) {
            Some(value) => Some(value.clone()),
            None => None,
        }
    }

}

