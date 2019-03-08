use super::patch::{Patch, Patches};
use super::vnode::VNode;
use crate::{Node, Tag};

pub fn diff(old: VNode, new: VNode) -> Patches {
    match (old.node, new.node) {
        (Node::Text(old_text), Node::Tag(new_tag)) => {
            vec![Patch::Replace()]
        },

        (Node::Tag(old_tag), Node::Text(new_text)) => {
            vec![Patch::Replace()]
        },

        (Node::Text(old_text), Node::Text(new_text)) =>  {
            vec![Patch::ChangeText()]
        },
        (Node::Tag(old_tag), Node::Tag(new_tag)) => {
            diff_tag(old_tag, new_tag)
        },
    }
}

fn diff_tag(old: Tag, new: Tag) -> Patches {
    Patches::new()
}