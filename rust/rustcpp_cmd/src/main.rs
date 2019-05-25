extern crate hello_lib;
extern crate rustcpp_lib;

use hello_lib::greeter;
use rustcpp_lib::infoprinter;

fn main() {
    let hello = greeter::Greeter::new("\nHello");
    hello.greet("from Rust!\n");

    let printer: &str = &infoprinter::InfoPrinter::new();
    println!("{}\n", printer);
}
