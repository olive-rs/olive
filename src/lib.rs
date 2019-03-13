mod view;
pub use view::{create_element, Element, Node, View};

pub mod dom;
pub use dom::{diff, patch};

pub trait Component {
    type Msg;
    type Props: Default;

    fn create(props: Self::Props) -> Self;
    fn update(&mut self, msg: Self::Msg);
    fn view(&self) -> View;
}
