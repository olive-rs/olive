use std::borrow::Cow;
use std::collections::HashMap;

#[derive(Clone, Debug, Default)]
pub struct Element(Vec<Node>);

impl Element {
    pub fn new() -> Self {
        Element(Vec::new())
    }

    fn push(mut self, value: Node) -> Self {
        self.0.push(value);
        self
    }
}

impl IntoIterator for Element {
    type Item = Node;
    type IntoIter = ::std::vec::IntoIter<Node>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[derive(Clone, Debug)]
pub enum Node {
    Tag(Tag),
    Text(String),
}

#[derive(Clone, Debug)]
pub struct Tag {
    pub name: String,
    // TODO: more type safety
    pub attributes: HashMap<String, String>,
    pub children: Element,
}

pub fn create_element<T: Into<String>>(
    name: T,
    attributes: HashMap<String, String>,
    children: Element,
) -> Element {
    Element::new().push(Node::Tag(Tag {
        name: name.into(),
        attributes,
        children,
    }))
}
