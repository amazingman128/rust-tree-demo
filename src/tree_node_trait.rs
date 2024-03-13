use std::cmp::Ordering;

/// 树节点特征
pub trait TreeNodeTrait {
    /// 设置节点ID
    fn set_id(&mut self, id: String) -> &mut Self;
    /// 获取树节点ID
    fn get_id(&self) -> String;

    /// 设置节点上级节点ID
    fn set_parent_id(&mut self, parent_id: String) -> &mut Self;

    /// 获取树节点父节点ID
    fn get_parent_id(&self) -> String;

    /// 设置节点名称
    fn set_name(&mut self, name: String) -> &mut Self;

    /// 获取节点名称
    fn get_name(&self) -> String;


    /// 设置节点权重
    fn set_weight(&mut self, weight: u32) -> &mut Self;

    /// 获取节点权重
    fn get_weight(&self) -> u32;

    /// 比较两个节点的权重
    fn cmp(&self, other: &Self) -> Ordering {
        self.get_weight().cmp(&other.get_weight())
    }
}