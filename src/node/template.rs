use std::fs;
use std::str::FromStr;
use roxmltree::Node;
use crate::node::{AlignEnum, AttrEnum, NodeElement, NodeEnum};

pub fn read_template(file_path: String) -> Result<Vec<NodeElement>, String> {
    fn closure_children<'a>(x: Node) -> Vec<NodeElement> {
        let mut vec_node_element: Vec<NodeElement> = Vec::new();
        for a in x.children().filter(|n| n.is_element()) {
            match read_element(a) {
                Some(mut node_element) => {
                    if a.clone().has_children() {
                        for b in closure_children(a) {
                            node_element.child.push(b);
                        }
                    }
                    vec_node_element.push(node_element);
                },
                None => {}
            }
        }
        vec_node_element
    }

    /*
    let mut file_path = PathBuf::new();
    file_path.push(env::current_dir().unwrap().as_path());
    file_path.push("examples");
    file_path.push("template.an");
    */

    let text = fs::read_to_string(file_path.clone()).unwrap();

    let opt = roxmltree::ParsingOptions {
        allow_dtd: true,
    };

    let doc = match roxmltree::Document::parse_with_options(&text, opt) {
        Ok(doc) => {
            //print!("{:?}", &doc);
            doc
        },
        Err(e) => return Err(format!("Error: {}.", e).to_string()),
    };

    let mut vec_node_element: Vec<NodeElement> = Vec::new();
    for a in doc.root().children().filter(|n| n.is_element()) {
        //print!("{:?} {:?} {:?} {:?}\n", a, a.has_children(), a.text(), a.tag_name());
        match read_element(a) {
            Some(mut node_element) => {
                if a.clone().has_children() {
                    for b in closure_children(a) {
                        node_element.child.push(b);
                    }
                }
                vec_node_element.push(node_element);            },
            None => {}
        }
    }

    Ok(vec_node_element)
}

fn read_element(a: Node) -> Option<NodeElement> {
    let mut node_element = NodeElement {
        node: NodeEnum::Unknow,
        child: vec![]
    };
    let mut vec_attr: Vec<AttrEnum> = Vec::new();
    for a_attr in a.attributes() {
        let attr_result = AttrEnum::from_str(a_attr.name());
        match attr_result {
            Ok(ok) => {
                match ok {
                    AttrEnum::Width(default) => {
                        let value:i32 = a_attr.value().parse().unwrap_or(default);
                        vec_attr.push(AttrEnum::Width(value))
                    }
                    AttrEnum::Height(default) => {
                        let value:i32 = a_attr.value().parse().unwrap_or(default);
                        vec_attr.push(AttrEnum::Height(value))
                    }
                    AttrEnum::Border(default) => {
                        let value:i32 = a_attr.value().parse().unwrap_or(default);
                        vec_attr.push(AttrEnum::Border(value))
                    }
                    AttrEnum::Align(default) => {
                        let value = match a_attr.value() {
                            "left" => AlignEnum::Left,
                            "center" => AlignEnum::Center,
                            "right" => AlignEnum::Right,
                            _ => default
                        };
                        vec_attr.push(AttrEnum::Align(value))
                    }
                    AttrEnum::Text(default) => {
                        vec_attr.push(AttrEnum::Text(a_attr.value().parse().unwrap_or(default)))
                    }
                }
            },
            Err(err) => {
                eprintln!("{:?}", err)
            }
        }
    }
    match NodeEnum::from_str(a.tag_name().name()) {
        Ok(ok) => {
            match ok {
                NodeEnum::Table(_) => {
                    node_element.node = NodeEnum::Table(vec_attr);
                    Some(node_element)
                }
                NodeEnum::Tr(_) => {
                    node_element.node = NodeEnum::Tr(vec_attr);
                    Some(node_element)
                }
                NodeEnum::Td(_) => {
                    node_element.node = NodeEnum::Td(vec_attr);
                    Some(node_element)
                }
                NodeEnum::Div(_) => {
                    node_element.node = NodeEnum::Div(vec_attr);
                    Some(node_element)
                }
                NodeEnum::Unknow => {
                    unreachable!()
                }
            }
        },
        Err(err) => {
            eprintln!("{:?}", err);
            None
        }
    }
}
