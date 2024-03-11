use std::cell::RefCell;
use std::collections::HashMap;

use std::rc::{Rc, Weak};

use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;

#[derive(Debug)]
pub struct TreeNode<T>
    where
        T: Serialize,
{
    id: String,
    name: String,
    parent_id: String,
    // parent: Option<Weak<TreeNode<T>>>,
    weight: u32,
    extra: HashMap<String, T>,
    children: Vec<Rc<RefCell<TreeNode<T>>>>,
}


impl<T> Serialize for TreeNode<T> where T: Serialize {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut state = serializer.serialize_struct("TreeNode", 6)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("parent_id", &self.parent_id)?;
        // state.serialize_field("parent", &self.parent)?;
        state.serialize_field("weight", &self.weight)?;
        state.serialize_field("extra", &self.extra)?;

        // 使用子节点的 Serialize 特性实现序列化
        state.serialize_field("children", &self.children.iter().map(|child| child.as_ref()).collect::<Vec<_>>())?;

        state.end()
    }
}

impl<T> TreeNode<T> where T: Serialize {
    pub fn new(id: String, name: String, parent_id: String, weight: u32) -> TreeNode<T> {
        TreeNode {
            id,
            name,
            parent_id,
            // parent: None,
            weight,
            extra: HashMap::new(),
            children: Vec::new(),
        }
    }

    pub fn add_child(&mut self, child: Rc<RefCell<Self>>) {
        self.children.push(child.clone());
        // child.parent = Some(Rc::downgrade(self));
    }

    pub fn build_from_list(list: Vec<TreeNode<T>>) -> HashMap<String, Rc<RefCell<TreeNode<T>>>> {
        let map: Rc<RefCell<HashMap<String, Rc<RefCell<TreeNode<T>>>>>> =
            Rc::new(RefCell::new(list.into_iter().map(|x| (x.id.clone(), Rc::new(RefCell::new(x)))).collect()));
        let mut map = map.borrow_mut();
        for (_, node) in map.clone().iter_mut() {
            let parent_id = node.borrow().parent_id.clone();
            if let Some(parent) = map.clone().get_mut(&parent_id) {
                parent.borrow_mut().add_child(Rc::clone(node));
            }
        }
        let x = map.clone();
        x
    }
}


#[cfg(test)]
mod tests {
    use std::ops::Deref;
    use crate::tree_node::TreeNode;

    #[test]
    pub fn test_build_from_list() {
        let list = vec![
            TreeNode::<String>::new("1".to_string(), "1".to_string(), "0".to_string(), 0),
            TreeNode::<String>::new("2".to_string(), "2".to_string(), "1".to_string(), 0),
            TreeNode::<String>::new("3".to_string(), "3".to_string(), "1".to_string(), 0),
            TreeNode::<String>::new("4".to_string(), "4".to_string(), "2".to_string(), 0),
            TreeNode::<String>::new("5".to_string(), "5".to_string(), "3".to_string(), 0),
        ];
        let root = TreeNode::build_from_list(list);
        for (_, item) in root.iter() {
            let tmp = item.as_ref().borrow();
            let data = tmp.deref();
            println!("{}", serde_json::to_string(&data).unwrap());
        }
    }
}