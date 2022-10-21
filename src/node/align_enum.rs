use std::fmt;

pub enum AlignEnum {
    Left,
    Center,
    Right
}

impl AlignEnum {
    fn text(&self) -> String {
        match self {
            AlignEnum::Left => {
                "left".to_string()
            }
            AlignEnum::Center => {
                "center".to_string()
            }
            AlignEnum::Right => {
                "right".to_string()
            }
        }
    }
}

impl fmt::Display for AlignEnum {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.text(), f)
    }
}

impl fmt::Debug for AlignEnum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.text().to_string())
    }
}
