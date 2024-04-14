use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "jisp.pest"]
pub struct JispParser;

fn main() {
    println!("Hello, world!");
}
