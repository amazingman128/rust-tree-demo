
pub mod tree_node;
pub mod tree_node_trait;
pub mod parser;
mod tree_node_config;
mod tree;
mod tree_builder;


pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
