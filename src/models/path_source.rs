/// A source of a file path, which can be either static (a hardcoded string) or dynamic (a String that can be modified at runtime)
#[derive(Debug, Clone, PartialEq)]
pub enum PathSource {
    Static(&'static str),
    Dynamic(String),
}

impl PathSource {
    pub fn as_str(&self) -> &str {
        return match self {
            Self::Static(s) => s,
            Self::Dynamic(s) => s.as_str(),
        };
    }
}
