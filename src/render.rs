use crate::{Component, Node};
use virtual_dom_rs::{VText, VirtualNode};
use wasm_bindgen::JsValue;
use web_sys::Node as DomNode;

pub fn render<C: Component>(component: C, mount_point: DomNode) -> Result<(), JsValue> {
    let node = component.view();

    match node {
        Node::Element(elem) => {
            let vnode = VirtualNode::Element(elem.into());
            render_vnode(vnode, mount_point)?;
        }

        Node::Text(text) => {
            let vnode = VirtualNode::Text(VText { text });
            render_vnode(vnode, mount_point)?;
        }

        Node::Fragment(nodes) => {
            unimplemented!("TODO: virtual_dom_rs is unimplemented for Fragments")
        }
    }

    Ok(())
}


fn render_vnode(vnode: VirtualNode, mount_point: DomNode) -> Result<(), JsValue> {
    let dom_node = vnode.create_dom_node();

    mount_point.append_child(&dom_node).map(|_|())
    
}