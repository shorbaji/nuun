
mod path;
mod object;
mod symboltable;

use std::cell::RefCell;
use std::rc::{Rc, Weak};
use thiserror;

use object::Object;
use path::Path;
use symboltable::SymbolTable;

#[derive(Debug, thiserror::Error)]
enum DustError {
    SymbolTableError(String),
}

impl std::fmt::Display for DustError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "DustError: {}", self)
    }
}

struct Dust {
    head: Rc<RefCell<SymbolTable>>,
    current: Rc<RefCell<SymbolTable>>,
}

impl Dust {
    pub fn new() -> Self {
        let root = SymbolTable::new(None, Path::Absolute(vec!["/".to_string()]));

        let head = Rc::new(RefCell::new(root));
        let current = head.clone();

        Dust {
            head,
            current,
        }
    }

    pub fn make_node(&mut self, path: Path) -> Result<Option<Weak<RefCell<SymbolTable>>>, DustError> {
        Ok(SymbolTable::make_node(self.current.clone(), path))
    }

    pub fn change_node(&mut self, path: Path) -> Result<(), DustError> {
        let (mut current, v) = match path.clone() {
            Path::Absolute(v) => (self.head.clone(), v),
            Path::Relative(v) => (self.current.clone(), v),
        };

        for key in v.iter() {
            let child = current.borrow().get_child(key.as_str())
                .and_then(|child| Some(child.upgrade().unwrap()));

            match child {
                Some(child) => current = child,
                None => return Err(DustError::SymbolTableError(format!("Path not found: {:?}", path))),
            }
        }

        self.current = current;
        Ok(())
    }

    pub fn delete_node(&mut self, path: Path) {
        unimplemented!("delete")
    }

    pub fn get(&self, key: &str) -> Option<Rc<RefCell<Object>>> {
        self.current.borrow().get(key)
    }

    pub fn set(&mut self, key: &str, value: Object) {
        self.current.borrow_mut().set(key, Rc::new(RefCell::new(value)));
    }
}
