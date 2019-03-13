use crate::{Element, Node};
use std::collections::HashMap;
use virtual_dom_rs::{Events, VElement, VText, VirtualNode};

// TO VirtualNode
// VirtualNode should be considered an implementation detail and removed
// when switching to a custom Virtual DOM Algorithm

impl From<Element> for VElement {
    fn from(elem: Element) -> VElement {
        VElement {
            tag: elem.name,
            attrs: elem.attributes,
            events: Events(HashMap::new()),
            children: elem.children.into_iter().map(|n| n.into()).collect(),
        }
    }
}

impl From<Node> for VirtualNode {
    fn from(node: Node) -> VirtualNode {
        match node {
            Node::Text(text) => VirtualNode::Text(VText { text }),
            Node::Element(elem) => VirtualNode::Element(elem.into()),
            Node::Fragment(_) => unimplemented!("TODO: virtual_dom_rs does not yet have support!"),
        }
    }
}
