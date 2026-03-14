#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DomNode {
    pub tag: String,
    pub text: Option<String>,
    pub children: Vec<DomNode>,
}

impl DomNode {
    pub fn element(tag: impl Into<String>) -> Self {
        Self {
            tag: tag.into(),
            text: None,
            children: Vec::new(),
        }
    }

    pub fn text(tag: impl Into<String>, text: impl Into<String>) -> Self {
        Self {
            tag: tag.into(),
            text: Some(text.into()),
            children: Vec::new(),
        }
    }

    pub fn with_child(mut self, child: DomNode) -> Self {
        self.children.push(child);
        self
    }
}
