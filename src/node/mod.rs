mod node_element;
mod node_enum;
mod attr_enum;
mod align_enum;
mod template;

// any error type implementing Display is acceptable.
type ParseError = &'static str;

pub use self::node_element::NodeElement;
pub use self::node_enum::NodeEnum;
pub use self::attr_enum::AttrEnum;
pub use self::align_enum::AlignEnum;
pub use self::template::read_template;