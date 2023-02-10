use std::path::Path;
use crate::models::graph::Graph;

mod models;

fn main() {
    let graph = Graph::from(Path::new("./test.txt"));
    println!("Hello, world!");
}
