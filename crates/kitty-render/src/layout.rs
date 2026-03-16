use crate::DomNode;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LayoutBox {
    pub tag: String,
    pub depth: usize,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct LayoutTree {
    pub boxes: Vec<LayoutBox>,
}

impl LayoutTree {
    pub fn from_dom(root: &DomNode) -> Self {
        let mut tree = Self::default();
        Self::walk(root, 0, &mut tree.boxes);
        tree
    }

    fn walk(node: &DomNode, depth: usize, out: &mut Vec<LayoutBox>) {
        out.push(LayoutBox {
            tag: node.tag.clone(),
            depth,
        });

        for child in &node.children {
            Self::walk(child, depth + 1, out);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{DomNode, LayoutTree};

    #[test]
    fn build_layout_tree_from_dom() {
        let dom = DomNode::element("html")
            .with_child(DomNode::element("body").with_child(DomNode::text("h1", "Kitty")));

        let layout = LayoutTree::from_dom(&dom);
        assert_eq!(layout.boxes.len(), 3);
        assert_eq!(layout.boxes[0].tag, "html");
        assert_eq!(layout.boxes[1].depth, 1);
        assert_eq!(layout.boxes[2].tag, "h1");
    }
}
