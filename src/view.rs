use std::collections::HashMap;

pub type View = Node;

#[derive(Clone, Debug)]
pub enum Node {
    Element(Element),
    Text(String),
    Fragment(Vec<Node>),
}

#[derive(Clone, Debug)]
pub struct Element {
    pub name: String,
    // TODO: more type safety
    pub attributes: HashMap<String, String>,
    pub children: Vec<Node>,
}

impl From<Element> for Node {
    fn from(element: Element) -> Self {
        Node::Element(element)
    }
}

impl From<String> for Node {
    fn from(text: String) -> Self {
        Node::Text(text)
    }
}

pub fn create_element<T: Into<String>>(
    name: T,
    attributes: HashMap<String, String>,
    children: Vec<Node>,
) -> Element {
    Element {
        name: name.into(),
        attributes,
        children,
    }
}
