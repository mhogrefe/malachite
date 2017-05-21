extern crate malachite_test;

use malachite_test::natural::arithmetic::add::demo_exhaustive_natural_add;
use malachite_test::natural::arithmetic::add::demo_random_natural_add;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 && args.len() != 4 {
        panic!("Usage: demo [limit] [demo name]");
    }
    let limit = if args.len() == 4 {
        args[2].parse().unwrap()
    } else {
        usize::max_value()
    };
    let demo_name = &*args.last().unwrap();
    match demo_name.as_ref() {
        "exhaustive_natural_add" => demo_exhaustive_natural_add(limit),
        "random_natural_add" => demo_random_natural_add(limit),

        _ => panic!("Invalid demo name: {}", demo_name),
    }
}
