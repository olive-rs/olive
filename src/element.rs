use std::collections::HashMap;

pub struct Element(Vec<Node>);

impl Element {
    fn new() -> Self {
        Element(Vec::new())
    }

    fn push(&mut self, value: Node) {
        self.0.push(value)
    }
}

pub enum Node {
    Tag(Tag),
    Text(String),
}

pub struct Tag {
    pub name: String,
    // TODO: more type safety
    pub attributes: HashMap<String, String>,
    pub children: Element,
}

pub fn create_element(name: String, attributes: HashMap<String, String>, children: Element) {
    Element::new().push(Node::Tag(Tag {
        name,
        attributes,
        children,
    }))
}
