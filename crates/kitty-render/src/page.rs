use crate::DomNode;

#[derive(Debug, Clone)]
pub struct Page {
    pub title: String,
    pub root: DomNode,
}

impl Page {
    pub fn new(title: impl Into<String>, root: DomNode) -> Self {
        Self {
            title: title.into(),
            root,
        }
    }
}
