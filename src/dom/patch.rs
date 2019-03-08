// TODO
use super::vnode::VNode;

pub type Patches = Vec<Patch>;

pub enum Patch {
    // Text (old, new)
    ChangeText(String, String),

    // Tag (old, new)
    Rename(String, String),
    Replace(),

    AppendChild(),
    RemoveChild(),
    ReplaceChild(),

    SetAttribute(),
    RemoveAttribute(),
    
    SetEvent(),
    SetProperty(),
}

pub fn patch(vnode: VNode, patches: Patches) {
    let node = vnode.node;

    for patch in patches {
        match patch {
            // Text
            Patch::ChangeText(old, new) =>
        }
    }
}