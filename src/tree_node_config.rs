use serde::{Deserialize, Serialize};

pub const DEFAULT_ID_KEY: &str = "id"; //默认的id的key

pub const DEFAULT_NAME_KEY: &str = "name"; //默认的name的key

pub const DEFAULT_PARENT_ID_KEY: &str = "parent_id"; //默认的parentId的key

pub const DEFAULT_WEIGHT_KEY: &str = "weight"; //默认的weight的key

pub const DEFAULT_CHILDREN_KEY: &str = "children"; //默认的children的key

pub const DEFAULT_DEEP: u32 = 0; //默认的递归深度

pub const DEFAULT_EXTRA_KEY: &str = "extra"; //默认的extra的key

/// 树状结构配置信息
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TreeNodeConfig {
    /// ID Key
    id_key: String,

    /// 名称 Key
    name_key: String,

    /// 上级ID Key
    parent_id_key: String,

    /// 排序 Key
    weight_key: String,

    /// 子集 Key
    children_key: String,

    // 可以配置递归深度 从0开始计算 默认此配置为空,即不限制
    deep: u32,

    extra_key: String,
}

impl Default for TreeNodeConfig {
    fn default() -> Self {
        Self {
            id_key: DEFAULT_ID_KEY.to_string(),
            name_key: DEFAULT_NAME_KEY.to_string(),
            parent_id_key: DEFAULT_PARENT_ID_KEY.to_string(),
            weight_key: DEFAULT_WEIGHT_KEY.to_string(),
            children_key: DEFAULT_CHILDREN_KEY.to_string(),
            deep: DEFAULT_DEEP,
            extra_key: DEFAULT_EXTRA_KEY.to_string(),
        }
    }
}

impl TreeNodeConfig {
    pub fn get_id_key(&self) -> String {
        return self.id_key.clone();
    }

    pub fn get_name_key(&self) -> String {
        return self.name_key.clone();
    }

    pub fn get_parent_id_key(&self) -> String {
        return self.parent_id_key.clone();
    }

    pub fn get_weight_key(&self) -> String {
        return self.weight_key.clone();
    }

    pub fn get_children_key(&self) -> String {
        return self.children_key.clone();
    }

    pub fn get_deep(&self) -> u32 {
        return self.deep;
    }

    pub fn set_deep(&mut self, deep: u32) -> &mut Self {
        self.deep = deep;
        self
    }

    pub fn set_id_key(&mut self, id_key: String) -> &mut Self {
        self.id_key = id_key;
        self
    }

    pub fn set_name_key(&mut self, name_key: String) -> &mut Self {
        self.name_key = name_key;
        self
    }

    pub fn set_parent_id_key(&mut self, parent_id_key: String) -> &mut Self {
        self.parent_id_key = parent_id_key;
        self
    }

    pub fn set_weight_key(&mut self, weight_key: String) -> &mut Self {
        self.weight_key = weight_key;
        self
    }

    pub fn set_children_key(&mut self, children_key: String) -> &mut Self {
        self.children_key = children_key;
        self
    }

    pub fn set_extra_key(&mut self, extra_key: String) -> &mut Self {
        self.extra_key = extra_key;
        self
    }

    pub fn get_extra_key(&self) -> String {
        return self.extra_key.clone();
    }
}

#[cfg(test)]
mod tests {
    use crate::tree_node_config::TreeNodeConfig;

    #[test]
    fn test_default() {
        let config = TreeNodeConfig::default();
        assert_eq!(config.get_id_key(), "id".to_string());
        assert_eq!(config.get_name_key(), "name".to_string());
        assert_eq!(config.get_parent_id_key(), "parent_id".to_string());
        assert_eq!(config.get_weight_key(), "weight".to_string());
        assert_eq!(config.get_children_key(), "children".to_string());
        assert_eq!(config.get_deep(), 0);
    }

    #[test]
    fn test_setters() {
        let mut config = TreeNodeConfig::default();
        config
            .set_id_key("custom_id".to_string())
            .set_name_key("custom_name".to_string())
            .set_parent_id_key("custom_parent_id".to_string())
            .set_weight_key("custom_weight".to_string())
            .set_children_key("custom_children".to_string())
            .set_deep(1);
        assert_eq!(config.get_id_key(), "custom_id".to_string());
        assert_eq!(config.get_name_key(), "custom_name".to_string());
        assert_eq!(config.get_parent_id_key(), "custom_parent_id".to_string());
        assert_eq!(config.get_weight_key(), "custom_weight".to_string());
        assert_eq!(config.get_children_key(), "custom_children".to_string());
        assert_eq!(config.get_deep(), 1);
    }
}