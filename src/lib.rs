mod element;
pub use element::Element;

pub trait Component {
    type Msg;
    type Props: Default;

    fn create(props: Self::Props) -> Self;
    fn update(&mut self, msg: Self::Msg);
    fn view(&self) -> Element;
}
