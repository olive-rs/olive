// TODO: Move out to separate crate?

use crate::{Element, Node};
use askama_escape::{Html, MarkupDisplay, Text};
use std::fmt::Display;

fn escape_html(s: String) -> String {
    MarkupDisplay::new_unsafe(s, Html).to_string()
}

fn escape_text(s: String) -> String {
    MarkupDisplay::new_unsafe(s, Text).to_string()
}

// TODO: Combine with render_node?
pub fn render_nodes(nodes: Vec<Node>) -> String {
    let mut s = String::new();

    for node in nodes.into_iter() {
        s += &render_node(node)
    }

    s
}

pub fn render_node(node: Node) -> String {
    match node {
        Node::Element(element) => render_element(element),
        Node::Text(text) => render_text(text),
        Node::Fragment(nodes) => render_nodes(nodes),
    }
}

// Based on maud
// TODO: extract to html-builder repo
fn render_element(element: Element) -> String {
    let mut s = String::new();
    let name = &escape_html(element.name);
    // <tag>
    s.push_str("<");
    s.push_str(name);
    s.push_str(">");

    // TODO: attrs

    // children
    s.push_str(&render_nodes(element.children));

    // </tag>
    s.push_str("</");
    s.push_str(name);
    s.push_str(">");

    s
}

fn render_text(text: String) -> String {
    escape_text(text)
}

impl Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", render_element(self.clone()))
    }
}
