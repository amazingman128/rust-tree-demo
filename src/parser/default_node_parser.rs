use serde::Serialize;
use crate::parser::node_parser_trait::NodeParserTrait;
use crate::tree::Tree;
use crate::tree_node::TreeNode;
use crate::tree_node_trait::TreeNodeTrait;

/// 默认节点转换
pub struct DefaultNodeParser;

impl<T> NodeParserTrait<T> for DefaultNodeParser where T: Serialize + Clone {
    fn parse(&self, from: TreeNode<T>, to: &mut Tree<T>) {
        to.set_id(from.get_id());
        to.set_parent_id(from.get_parent_id());
        to.set_name(from.get_name());
        to.set_weight(from.get_weight());

    }
}