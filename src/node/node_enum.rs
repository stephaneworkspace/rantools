use std::fmt;
use std::str::FromStr;
use crate::node::{AttrEnum, ParseError};

pub enum NodeEnum {
    Table(Vec<AttrEnum>),
    Tr(Vec<AttrEnum>),
    Td(Vec<AttrEnum>),
    Div(Vec<AttrEnum>),
    Unknow
}

impl NodeEnum {
    fn text(&self) -> String {
        match self {
            NodeEnum::Table(_) => {
                "table".to_string()
            }
            NodeEnum::Tr(_) => {
                "tr".to_string()
            }
            NodeEnum::Td(_) => {
                "td".to_string()
            }
            NodeEnum::Div(_) => {
                "div".to_string()
            },
            NodeEnum::Unknow => {
                "?".to_string()
            }
        }
    }
    fn debug(&self) -> String {
        match self {
            NodeEnum::Table(content) => {
                format!("{}, {:?}", self.text(), content)
            }
            NodeEnum::Tr(content) => {
                format!("{}, {:?}", self.text(), content)
            }
            NodeEnum::Td(content) => {
                format!("{}, {:?}", self.text(), content)
            }
            NodeEnum::Div(content) => {
                format!("{}, {:?}", self.text(), content)
            }
            NodeEnum::Unknow => {
                format!("{}", self.text())
            }
        }
    }
}

impl fmt::Display for NodeEnum {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.text(), f)
    }
}

impl fmt::Debug for NodeEnum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.debug().to_string())
    }
}

impl FromStr for NodeEnum {
    type Err = ParseError;
    fn from_str(attr: &str) -> Result<Self, Self::Err> {
        match attr {
            "table" => Ok(Self::Table(vec![])),
            "tr" => Ok(Self::Tr(vec![])),
            "td" => Ok(Self::Td(vec![])),
            "div" => Ok(Self::Div(vec![])),
            _ => Err("Could not parse a node"),
        }
    }
}