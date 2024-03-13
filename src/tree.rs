use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};
use serde::Serialize;
use crate::tree_node_config::TreeNodeConfig;
use crate::tree_node_trait::TreeNodeTrait;

/// 树
/// 包含上下级节点信息
pub struct Tree<T> where T: Serialize + Clone {
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

    /// 树配置
    node_config: TreeNodeConfig,

    /// 上级节点
    parent: Option<Weak<Tree<T>>>,

    /// 子节点
    children: Vec<Rc<RefCell<Tree<T>>>>,

}


impl<T> Tree<T> where T: Serialize + Clone {
    pub fn new(config: Option<TreeNodeConfig>) -> Self {
        let node_config = config.unwrap_or(TreeNodeConfig::default());
        Self {
            id: "".to_string(),
            name: "".to_string(),
            parent_id: "".to_string(),
            weight: 0,
            extra: HashMap::new(),
            children: vec![],
            node_config,
            parent: None,
        }
    }

    pub fn get_config(&self) -> &TreeNodeConfig {
        &self.node_config
    }

    pub fn set_extra(&mut self, extra: HashMap<String, T>) -> &mut Self {
        for (key, value) in extra {
            self.put_extra(key, value);
        }
        self
    }

    pub fn put_extra(&mut self, key: String, value: T) -> &mut Self {
        self.extra.insert(key, value);
        self
    }

    pub fn add_child(&mut self, child: Rc<RefCell<Tree<T>>>) -> &mut Self {
        self.children.push(child);
        self
    }

    pub fn set_children(&mut self, children: Vec<Rc<RefCell<Tree<T>>>>) -> &mut Self {
        self.children = children;
        self
    }
}


impl<T> TreeNodeTrait for Tree<T> where T: Serialize + Clone {
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