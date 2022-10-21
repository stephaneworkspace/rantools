use crate::node::NodeEnum;

#[derive(Debug)]
pub struct NodeElement {
    pub node: NodeEnum,
    pub child: Vec<NodeElement>
}
