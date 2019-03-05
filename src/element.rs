use std::collections::HashMap;

#[derive(Clone, Debug, Default)]
pub struct Element(Vec<Node>);

impl Element {
    pub fn new() -> Self {
        Element(Vec::new())
    }

    fn push<T: Into<Node>>(mut self, value: T) -> Self {
        self.0.push(value.into());
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

impl From<Tag> for Node {
    fn from(tag: Tag) -> Self {
        Node::Tag(tag)
    }
}

impl From<String> for Node {
    fn from(text: String) -> Self {
        Node::Text(text)
    }
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
    Element::new().push(Tag {
        name: name.into(),
        attributes,
        children,
    })
}
