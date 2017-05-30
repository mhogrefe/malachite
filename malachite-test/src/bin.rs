extern crate malachite_test;

use malachite_test::natural::arithmetic::add::*;
use malachite_test::natural::conversion::assign_integer::*;
use malachite_test::natural::conversion::clone_and_assign::*;
use malachite_test::natural::conversion::from_u32::*;
use malachite_test::natural::conversion::from_u64::*;
use malachite_test::natural::conversion::to_u32::*;
use malachite_test::natural::conversion::to_u64::*;
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
                "exhaustive_natural_assign" => demo_exhaustive_natural_assign(limit),
                "exhaustive_natural_assign_ref" => demo_exhaustive_natural_assign_ref(limit),
                "exhaustive_natural_assign_integer" => {
                    demo_exhaustive_natural_assign_integer(limit)
                }
                "exhaustive_natural_assign_integer_ref" => {
                    demo_exhaustive_natural_assign_integer_ref(limit)
                }
                "exhaustive_natural_clone" => demo_exhaustive_natural_clone(limit),
                "exhaustive_natural_clone_from" => demo_exhaustive_natural_clone_from(limit),
                "exhaustive_natural_from_u32" => demo_exhaustive_natural_from_u32(limit),
                "exhaustive_natural_from_u64" => demo_exhaustive_natural_from_u64(limit),
                "exhaustive_natural_to_u32" => demo_exhaustive_natural_to_u32(limit),
                "exhaustive_natural_to_u32_wrapping" => {
                    demo_exhaustive_natural_to_u32_wrapping(limit)
                }
                "exhaustive_natural_to_u64" => demo_exhaustive_natural_to_u64(limit),
                "exhaustive_natural_to_u64_wrapping" => {
                    demo_exhaustive_natural_to_u64_wrapping(limit)
                }
                "random_natural_add" => demo_random_natural_add(limit),
                "random_natural_assign" => demo_random_natural_assign(limit),
                "random_natural_assign_ref" => demo_random_natural_assign_ref(limit),
                "random_natural_assign_integer" => demo_random_natural_assign_integer(limit),
                "random_natural_assign_integer_ref" => {
                    demo_random_natural_assign_integer_ref(limit)
                }
                "random_natural_clone" => demo_random_natural_clone(limit),
                "random_natural_clone_from" => demo_random_natural_clone_from(limit),
                "random_natural_from_u32" => demo_random_natural_from_u32(limit),
                "random_natural_from_u64" => demo_random_natural_from_u64(limit),
                "random_natural_to_u32" => demo_random_natural_to_u32(limit),
                "random_natural_to_u32_wrapping" => demo_random_natural_to_u32_wrapping(limit),
                "random_natural_to_u64" => demo_random_natural_to_u64(limit),
                "random_natural_to_u64_wrapping" => demo_random_natural_to_u64_wrapping(limit),

                _ => panic!("Invalid demo name: {}", item_name),
            }
        }
        "bench" => {
            match item_name.as_ref() {
                "exhaustive_natural_add" => benchmark_exhaustive_natural_add(limit, "temp.gp"),
                "random_natural_add" => benchmark_random_natural_add(limit, 1024, "temp.gp"),
                "exhaustive_natural_from_u32" => {
                    benchmark_exhaustive_natural_from_u32(limit, "temp.gp")
                }
                "random_natural_from_u32" => benchmark_random_natural_from_u32(limit, "temp.gp"),
                "exhaustive_natural_from_u64" => {
                    benchmark_exhaustive_natural_from_u64(limit, "temp.gp")
                }
                "random_natural_from_u64" => benchmark_random_natural_from_u64(limit, "temp.gp"),

                "all" => {
                    benchmark_exhaustive_natural_add(100000, "exhaustive_natural_add.gp");
                    benchmark_random_natural_add(100000, 1024, "random_natural_add.gp");
                    benchmark_exhaustive_natural_from_u32(100000, "exhaustive_natural_from_u32.gp");
                    benchmark_random_natural_from_u32(100000, "random_natural_from_u32.gp");
                    benchmark_exhaustive_natural_from_u64(100000, "exhaustive_natural_from_u64.gp");
                    benchmark_random_natural_from_u64(100000, "random_natural_from_u64.gp");
                }

                _ => panic!("Invalid bench name: {}", item_name),
            }
        }
        _ => panic!("Invalid item_type: {}", args[1]),
    }
}
