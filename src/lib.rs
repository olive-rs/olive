mod element;
pub use element::{create_element, Element, Node, Tag};

pub mod dom;

pub trait Component {
    type Msg;
    type Props: Default;

    fn create(props: Self::Props) -> Self;
    fn update(&mut self, msg: Self::Msg);
    fn view(&self) -> Element;
}
