use crate::node::{AlignEnum, ParseError};
use std::fmt;
use std::str::FromStr;

pub enum AttrEnum {
    Width(i32),
    Height(i32),
    Border(i32),
    Align(AlignEnum),
    Text(String)
}


impl AttrEnum {
    fn text(&self) -> String {
        match self {
            AttrEnum::Width(_) => {
                "width".to_string()
            }
            AttrEnum::Height(_) => {
                "height".to_string()
            }
            AttrEnum::Border(_) => {
                "border".to_string()
            }
            AttrEnum::Align(_) => {
                "div".to_string()
            }
            AttrEnum::Text(_) => {
                "text".to_string()
            }
        }
    }

    fn debug(&self) -> String {
        match self {
            AttrEnum::Width(content) => {
                format!("{}, {:?}", self.text(), content)
            }
            AttrEnum::Height(content) => {
                format!("{}, {:?}", self.text(), content)
            }
            AttrEnum::Border(content) => {
                format!("{}, {:?}", self.text(), content)
            }
            AttrEnum::Align(content) => {
                format!("{}, {:?}", self.text(), content)
            }
            AttrEnum::Text(content) => {
                format!("{}, {:?}", self.text(), content)
            }
        }
    }
}

impl fmt::Display for AttrEnum {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.text(), f)
    }
}

impl fmt::Debug for AttrEnum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.debug().to_string())
    }
}

impl FromStr for AttrEnum {
    type Err = ParseError;
    fn from_str(attr: &str) -> Result<Self, Self::Err> {
        match attr {
            "width" => Ok(Self::Width(0)),
            "height" => Ok(Self::Height(0)),
            "border" => Ok(Self::Border(0)),
            "align" => Ok(Self::Align(AlignEnum::Left)),
            "text" => Ok(Self::Text("".to_string())),
            _ => Err("Could not parse a attribute"),
        }
    }
}
