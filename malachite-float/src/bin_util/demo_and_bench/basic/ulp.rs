// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_float::test_util::bench::bucketers::float_complexity_bucketer;
use malachite_float::test_util::common::to_hex_string;
use malachite_float::test_util::generators::{
    float_gen, float_gen_var_12, float_gen_var_13, float_gen_var_3,
};
use malachite_float::{ComparableFloat, ComparableFloatRef};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_float_ulp);
    register_demo!(runner, demo_float_ulp_debug);
    register_demo!(runner, demo_float_ulp_extreme);
    register_demo!(runner, demo_float_ulp_extreme_debug);
    register_demo!(runner, demo_float_increment);
    register_demo!(runner, demo_float_increment_debug);
    register_demo!(runner, demo_float_increment_extreme);
    register_demo!(runner, demo_float_increment_extreme_debug);
    register_demo!(runner, demo_float_decrement);
    register_demo!(runner, demo_float_decrement_debug);
    register_demo!(runner, demo_float_decrement_extreme);
    register_demo!(runner, demo_float_decrement_extreme_debug);

    register_bench!(runner, benchmark_float_ulp);
    register_bench!(runner, benchmark_float_increment);
    register_bench!(runner, benchmark_float_decrement);
}

fn demo_float_ulp(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        println!("ulp({}) = {:?}", x, x.ulp());
    }
}

fn demo_float_ulp_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        println!(
            "ulp({:#x}) = {}",
            ComparableFloatRef(&x),
            x.ulp().map_or("None".to_string(), |f| to_hex_string(&f))
        );
    }
}

fn demo_float_ulp_extreme(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen_var_12().get(gm, config).take(limit) {
        println!("ulp({}) = {:?}", x, x.ulp());
    }
}

fn demo_float_ulp_extreme_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen_var_12().get(gm, config).take(limit) {
        println!(
            "ulp({:#x}) = {}",
            ComparableFloatRef(&x),
            x.ulp().map_or("None".to_string(), |f| to_hex_string(&f))
        );
    }
}

fn demo_float_increment(gm: GenMode, config: &GenConfig, limit: usize) {
    for mut x in float_gen_var_3().get(gm, config).take(limit) {
        let old_x = x.clone();
        x.increment();
        println!("x := {old_x}; x.increment(); x = {x}");
    }
}

fn demo_float_increment_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for mut x in float_gen_var_3().get(gm, config).take(limit) {
        let old_x = x.clone();
        x.increment();
        println!(
            "x := {:#x}; x.increment(); x = {:#x}",
            ComparableFloat(old_x),
            ComparableFloat(x)
        );
    }
}

fn demo_float_increment_extreme(gm: GenMode, config: &GenConfig, limit: usize) {
    for mut x in float_gen_var_13().get(gm, config).take(limit) {
        let old_x = x.clone();
        x.increment();
        println!("x := {old_x}; x.increment(); x = {x}");
    }
}

fn demo_float_increment_extreme_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for mut x in float_gen_var_13().get(gm, config).take(limit) {
        let old_x = x.clone();
        x.increment();
        println!(
            "x := {:#x}; x.increment(); x = {:#x}",
            ComparableFloat(old_x),
            ComparableFloat(x)
        );
    }
}

fn demo_float_decrement(gm: GenMode, config: &GenConfig, limit: usize) {
    for mut x in float_gen_var_3().get(gm, config).take(limit) {
        let old_x = x.clone();
        x.increment();
        println!("x := {old_x}; x.increment(); x = {x}");
    }
}

fn demo_float_decrement_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for mut x in float_gen_var_3().get(gm, config).take(limit) {
        let old_x = x.clone();
        x.increment();
        println!(
            "x := {:#x}; x.increment(); x = {:#x}",
            ComparableFloat(old_x),
            ComparableFloat(x)
        );
    }
}

fn demo_float_decrement_extreme(gm: GenMode, config: &GenConfig, limit: usize) {
    for mut x in float_gen_var_13().get(gm, config).take(limit) {
        let old_x = x.clone();
        x.increment();
        println!("x := {old_x}; x.increment(); x = {x}");
    }
}

fn demo_float_decrement_extreme_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for mut x in float_gen_var_13().get(gm, config).take(limit) {
        let old_x = x.clone();
        x.increment();
        println!(
            "x := {:#x}; x.increment(); x = {:#x}",
            ComparableFloat(old_x),
            ComparableFloat(x)
        );
    }
}

fn benchmark_float_ulp(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "Float.ulp()",
        BenchmarkType::Single,
        float_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &float_complexity_bucketer("x"),
        &mut [("Malachite", &mut |x| no_out!(x.ulp()))],
    );
}

fn benchmark_float_increment(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "Float.increment()",
        BenchmarkType::Single,
        float_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &float_complexity_bucketer("x"),
        &mut [("Malachite", &mut |mut x| x.increment())],
    );
}

fn benchmark_float_decrement(gm: GenMode, config: &GenConfig, limit: usize, file_name: &str) {
    run_benchmark(
        "Float.decrement()",
        BenchmarkType::Single,
        float_gen_var_3().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &float_complexity_bucketer("x"),
        &mut [("Malachite", &mut |mut x| x.decrement())],
    );
}
