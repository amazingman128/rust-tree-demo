use std::cell::RefCell;
use std::collections::HashMap;

use std::rc::{Rc};

use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;
use crate::tree_node_trait::TreeNodeTrait;

/// 树节点
#[derive(Debug)]
pub struct TreeNode<T>
    where
        T: Serialize + Clone,
{
    /// 节点ID
    id: String,

    /// 节点名称
    name: String,

    /// 上级节点ID
    parent_id: String,

    ///权重
    weight: u32,

    /// 扩展数据
    extra: HashMap<String, T>,

    children: Vec<Rc<RefCell<TreeNode<T>>>>,
}


impl<T> Serialize for TreeNode<T> where T: Serialize + Clone {
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

impl<T> TreeNode<T> where T: Serialize + Clone {
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
        let map = map.borrow_mut();
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


impl<T> TreeNodeTrait for TreeNode<T> where T: Serialize + Clone {
    fn set_id(&mut self, id: String) -> &mut Self {
        self.id = id;
        self
    }

    fn get_id(&self) -> String {
        self.id.clone()
    }

    fn set_parent_id(&mut self, parent_id: String) -> &mut Self {
        self.parent_id = parent_id;
        self
    }

    fn get_parent_id(&self) -> String {
        self.parent_id.clone()
    }

    fn set_name(&mut self, name: String) -> &mut Self {
        self.name = name;
        self
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn set_weight(&mut self, weight: u32) -> &mut Self {
        self.weight = weight;
        self
    }

    fn get_weight(&self) -> u32 {
        self.weight
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