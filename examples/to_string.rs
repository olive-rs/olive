use olive::*;
use std::collections::HashMap;

fn main() {
    let elem = create_element(
        "h1",
        HashMap::new(),
        create_element("a", HashMap::new(), Element::new()),
    )
    .to_string();

    println!("{}", elem)
}
