# olive

A WIP Rust frontend framework. 🕊

TODO: Props Example

```rust
// lib.rs
use olive::{Component, Element, rsx};

struct Counter {
    count: i32;
}

enum Msg {
    Increment,
    Decrement,
}

impl Component for Counter {
    type Msg = Msg;
    type Props = ();

    fn create(_: ()) -> Self {
        Counter { count: 0 }
    }

    fn update(&mut self, msg: Msg) {
        match msg {
            Msg::Increment => self.count += 1,
            Msg::Decrement => self.count -= 1,
        }
    }

    fn view(&self) -> Element {
        rsx! {
            <h1> Counter </h1>
            <p>{self.count}</p>
            <button onclick=Msg::Increment> +1 </button>
            <button onclick=Msg::Decrement> -1 </button>
        }
    }
}
```
