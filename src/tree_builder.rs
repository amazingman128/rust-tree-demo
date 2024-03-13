use std::cell::{RefCell};
use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;
use serde::Serialize;
use crate::parser::node_parser_trait::NodeParserTrait;
use crate::tree::Tree;
use crate::tree_node::TreeNode;
use crate::tree_node_config::TreeNodeConfig;
use crate::tree_node_trait::TreeNodeTrait;

/// 树创建器
pub struct TreeBuilder<T> where T: Serialize + Clone {
    /// 根节点
    root: Tree<T>,

    /// 树字典
    /// key为节点ID
    id_tree_map: Rc<RefCell<HashMap<String, Rc<RefCell<Tree<T>>>>>>,

    /// 是否已创建
    is_build: bool,

}

impl<T> TreeBuilder<T> where T: Serialize + Clone {
    pub fn new(root_id: String, tree_config: TreeNodeConfig) -> Self {
        let mut root_tree = Tree::new(Some(tree_config));
        root_tree.set_id(root_id);
        Self {
            root: root_tree,
            id_tree_map: Rc::new(RefCell::new(Default::default())),
            is_build: false,
        }
    }

    pub fn of(root_id: String) -> Self {
        TreeBuilder::new(root_id, TreeNodeConfig::default())
    }

    pub fn set_id(&mut self, root_id: String) -> &mut Self {
        self.root.set_id(root_id);
        self
    }

    pub fn set_parent_id(&mut self, parent_id: String) -> &mut Self {
        self.root.set_parent_id(parent_id);
        self
    }

    pub fn set_name(&mut self, name: String) -> &mut Self {
        self.root.set_name(name);
        self
    }
    pub fn set_weight(&mut self, weight: u32) -> &mut Self {
        self.root.set_weight(weight);
        self
    }

    pub fn put_extra(&mut self, key: String, value: T) -> &mut Self {
        self.root.put_extra(key, value);
        self
    }

    pub fn add_map(&mut self, map: HashMap<String, Tree<T>>) -> &mut Self {
        if !map.is_empty() {
            self.check_build();
            for (key, value) in map {
                self.id_tree_map.borrow_mut().insert(key, Rc::new(RefCell::new(value)));
            }
        }
        self
    }

    pub fn reset(&mut self) -> &mut Self {
        self.id_tree_map.borrow_mut().clear();
        self.root.set_children(vec![]);
        self.is_build = false;
        self
    }

    /// 添加节点列表
    /// node_list 树节点
    /// node_parser 节点转换器
    pub fn add_node_list(&mut self, node_list: Vec<TreeNode<T>>, node_parser: Box<dyn NodeParserTrait<T>>) -> &mut Self {
        if !node_list.is_empty() {
            self.check_build();
            let tree_config = self.root.get_config();
            let mut map: HashMap<String, Tree<T>> = HashMap::new();
            for item in node_list {
                let tree_id = item.get_id().clone();
                let mut tree = Tree::new(Some(tree_config.clone()));
                node_parser.parse(item, &mut tree);
                map.insert(tree_id, tree);
            }
            self.add_map(map);
        }
        self
    }

    /// 检查是否已经构建
    pub fn check_build(&self) {
        if self.is_build {
            panic!("当前树已构建")
        }
    }
}