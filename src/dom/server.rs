use crate::{Element, Node, Tag};
use askama_escape::{Html, MarkupDisplay, Text};
use std::fmt::Display;

fn escape_html(s: String) -> String {
    MarkupDisplay::new_unsafe(s, Html).to_string()
}

fn escape_text(s: String) -> String {
    MarkupDisplay::new_unsafe(s, Text).to_string()
}

pub fn render_element(element: Element) -> String {
    let mut s = String::new();

    for node in element.into_iter() {
        s.push_str(&render_node(node))
    }

    s
}

fn render_node(node: Node) -> String {
    match node {
        Node::Tag(tag) => render_tag(tag),
        Node::Text(text) => render_text(text),
    }
}

// Based on maud
// TODO: extract to html-builder repo
fn render_tag(tag: Tag) -> String {
    let mut s = String::new();
    let name = &escape_html(tag.name);
    // <tag>
    s.push_str("<");
    s.push_str(name);
    s.push_str(">");

    // TODO: attrs

    // children
    s.push_str(&render_element(tag.children));

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
