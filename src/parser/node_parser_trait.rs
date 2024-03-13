use serde::Serialize;

use crate::tree::Tree;
use crate::tree_node::TreeNode;

/// 树节点转换特征
pub trait NodeParserTrait<T> where T: Serialize + Clone {
    /// 将树节点类型转化为树类型
    fn parse(&self, from: TreeNode<T>, to: &mut Tree<T>);
}