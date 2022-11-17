use htree::htree::{HTree, Update, UpdateType};

fn main() {
    println!("Hello, world!");

    let mut t = HTree::new("root.com".to_string());

    let us = [Update {
        update_type: UpdateType::Add,
        pointer: "s1.n1.dc1.root.com:8081".to_string(),
        weight: 1,
        load: 1,
    }];
    t.update(&us);

    t.print();
}
