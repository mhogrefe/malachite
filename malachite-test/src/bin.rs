extern crate malachite_test;

use std::env;

use malachite_test::common::{get_gm, get_no_special_gm, DemoBenchRegistry, ScaleType};
use malachite_test::tune::tune;

fn optionally_tune(args: &[String]) -> bool {
    if args.len() == 3 && args[1] == "tune" {
        tune(&args[2]);
        true
    } else {
        false
    }
}

pub fn main() {
    let args: Vec<String> = env::args().collect();
    if optionally_tune(&args) {
        return;
    }
    if args.len() != 3 && args.len() != 4 {
        panic!("Usage: [exhaustive|random|special_random] [limit] [demo/bench name]");
    }
    let generation_mode = &args[1];
    assert!(
        generation_mode == "exhaustive"
            || generation_mode == "random"
            || generation_mode == "special_random",
        "Bad generation mode"
    );
    let limit = if args.len() == 4 {
        args[2].parse().unwrap()
    } else {
        usize::max_value()
    };
    let item_name = args.last().unwrap();

    let mut registry = DemoBenchRegistry::default();
    malachite_test::register(&mut registry);

    if item_name == "all" {
        registry.benchmark_all(limit);
    } else if let Some(f) = registry.lookup_demo(item_name) {
        f(get_gm(generation_mode, ScaleType::Small), limit);
    } else if let Some(&(scale_type, f)) = registry.lookup_bench(item_name) {
        f(get_gm(generation_mode, scale_type), limit, "temp.gp");
    } else if let Some(f) = registry.lookup_no_special_demo(item_name) {
        f(get_no_special_gm(generation_mode, ScaleType::None), limit);
    } else if let Some(&(scale_type, f)) = registry.lookup_no_special_bench(item_name) {
        f(
            get_no_special_gm(generation_mode, scale_type),
            limit,
            "temp.gp",
        );
    } else {
        panic!("Invalid item: {}", item_name);
    }
}
