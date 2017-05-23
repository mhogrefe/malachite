extern crate malachite_test;

use malachite_test::natural::arithmetic::add::benchmark_exhaustive_natural_add;
use malachite_test::natural::arithmetic::add::benchmark_random_natural_add;
use malachite_test::natural::arithmetic::add::demo_exhaustive_natural_add;
use malachite_test::natural::arithmetic::add::demo_random_natural_add;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 && args.len() != 4 {
        panic!("Usage: [demo|bench] [limit] [demo name]");
    }
    let limit = if args.len() == 4 {
        args[2].parse().unwrap()
    } else {
        usize::max_value()
    };
    let item_name = &*args.last().unwrap();
    match args[1].as_ref() {
        "demo" => {
            match item_name.as_ref() {
                "exhaustive_natural_add" => demo_exhaustive_natural_add(limit),
                "random_natural_add" => demo_random_natural_add(limit),

                _ => panic!("Invalid demo name: {}", item_name),
            }
        }
        "bench" => {
            match item_name.as_ref() {
                "exhaustive_natural_add" => benchmark_exhaustive_natural_add(limit, "temp.gp"),
                "random_natural_add" => benchmark_random_natural_add(limit, 1024, "temp.gp"),

                "all" => {
                    benchmark_exhaustive_natural_add(100000, "exhaustive_natural_add.gp");
                    benchmark_random_natural_add(100000, 1024, "random_natural_add.gp");
                }

                _ => panic!("Invalid bench name: {}", item_name),
            }
        }
        _ => panic!("Invalid item_type: {}", args[1]),
    }
}
