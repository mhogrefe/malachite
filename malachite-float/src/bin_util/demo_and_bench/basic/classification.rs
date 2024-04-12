// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_float::test_util::bench::bucketers::{
    float_complexity_bucketer, pair_2_float_complexity_bucketer,
};
use malachite_float::test_util::common::to_hex_string;
use malachite_float::test_util::generators::{float_gen, float_gen_rm};
use malachite_float::{ComparableFloat, ComparableFloatRef};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_float_is_nan);
    register_demo!(runner, demo_float_is_nan_debug);
    register_demo!(runner, demo_float_is_finite);
    register_demo!(runner, demo_float_is_finite_debug);
    register_demo!(runner, demo_float_is_infinite);
    register_demo!(runner, demo_float_is_infinite_debug);
    register_demo!(runner, demo_float_is_positive_zero);
    register_demo!(runner, demo_float_is_positive_zero_debug);
    register_demo!(runner, demo_float_is_negative_zero);
    register_demo!(runner, demo_float_is_negative_zero_debug);
    register_demo!(runner, demo_float_is_zero);
    register_demo!(runner, demo_float_is_zero_debug);
    register_demo!(runner, demo_float_is_normal);
    register_demo!(runner, demo_float_is_normal_debug);
    register_demo!(runner, demo_float_is_sign_positive);
    register_demo!(runner, demo_float_is_sign_positive_debug);
    register_demo!(runner, demo_float_is_sign_negative);
    register_demo!(runner, demo_float_is_sign_negative_debug);
    register_demo!(runner, demo_float_classify);
    register_demo!(runner, demo_float_classify_debug);
    register_demo!(runner, demo_float_into_non_nan);
    register_demo!(runner, demo_float_into_non_nan_debug);
    register_demo!(runner, demo_float_to_non_nan);
    register_demo!(runner, demo_float_to_non_nan_debug);
    register_demo!(runner, demo_float_into_finite);
    register_demo!(runner, demo_float_into_finite_debug);
    register_demo!(runner, demo_float_to_finite);
    register_demo!(runner, demo_float_to_finite_debug);

    register_bench!(runner, benchmark_float_is_nan_library_comparison);
    register_bench!(runner, benchmark_float_is_finite_library_comparison);
    register_bench!(runner, benchmark_float_is_infinite_library_comparison);
    register_bench!(runner, benchmark_float_is_positive_zero);
    register_bench!(runner, benchmark_float_is_negative_zero);
    register_bench!(runner, benchmark_float_is_zero_library_comparison);
    register_bench!(runner, benchmark_float_is_normal_library_comparison);
    register_bench!(runner, benchmark_float_is_sign_positive_library_comparison);
    register_bench!(runner, benchmark_float_is_sign_negative_library_comparison);
    register_bench!(runner, benchmark_float_classify_library_comparison);
    register_bench!(runner, benchmark_float_to_non_nan_evaluation_strategy);
    register_bench!(runner, benchmark_float_to_finite_evaluation_strategy);
}

fn demo_float_is_nan(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        if x.is_nan() {
            println!("{x} is NaN");
        } else {
            println!("{x} is not NaN");
        }
    }
}

fn demo_float_is_nan_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        if x.is_nan() {
            println!("{:#x} is NaN", ComparableFloat(x));
        } else {
            println!("{:#x} is not NaN", ComparableFloat(x));
        }
    }
}

fn demo_float_is_finite(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        if x.is_finite() {
            println!("{x} is finite");
        } else {
            println!("{x} is not finite");
        }
    }
}

fn demo_float_is_finite_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        if x.is_finite() {
            println!("{:#x} is finite", ComparableFloat(x));
        } else {
            println!("{:#x} is not finite", ComparableFloat(x));
        }
    }
}

fn demo_float_is_infinite(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        if x.is_finite() {
            println!("{x} is infinite");
        } else {
            println!("{x} is not infinite");
        }
    }
}

fn demo_float_is_infinite_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        if x.is_infinite() {
            println!("{:#x} is infinite", ComparableFloat(x));
        } else {
            println!("{:#x} is not infinite", ComparableFloat(x));
        }
    }
}

fn demo_float_is_positive_zero(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        if x.is_zero() {
            println!("{x} is positive zero");
        } else {
            println!("{x} is not positive zero");
        }
    }
}

fn demo_float_is_positive_zero_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        if x.is_finite() {
            println!("{:#x} is positive zero", ComparableFloat(x));
        } else {
            println!("{:#x} is not positive zero", ComparableFloat(x));
        }
    }
}

fn demo_float_is_negative_zero(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        if x.is_zero() {
            println!("{x} is negative zero");
        } else {
            println!("{x} is not negative zero");
        }
    }
}

fn demo_float_is_negative_zero_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        if x.is_finite() {
            println!("{:#x} is negative zero", ComparableFloat(x));
        } else {
            println!("{:#x} is not negative zero", ComparableFloat(x));
        }
    }
}

fn demo_float_is_zero(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        if x.is_zero() {
            println!("{x} is zero");
        } else {
            println!("{x} is not zero");
        }
    }
}

fn demo_float_is_zero_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        if x.is_finite() {
            println!("{:#x} is zero", ComparableFloat(x));
        } else {
            println!("{:#x} is not zero", ComparableFloat(x));
        }
    }
}

fn demo_float_is_normal(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        if x.is_normal() {
            println!("{x} is normal");
        } else {
            println!("{x} is not normal");
        }
    }
}

fn demo_float_is_normal_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        if x.is_normal() {
            println!("{:#x} is normal", ComparableFloat(x));
        } else {
            println!("{:#x} is not normal", ComparableFloat(x));
        }
    }
}

fn demo_float_is_sign_positive(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        if x.is_sign_positive() {
            println!("{x} has a positive sign");
        } else {
            println!("{x} does not have a positive sign");
        }
    }
}

fn demo_float_is_sign_positive_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        if x.is_sign_positive() {
            println!("{:#x} has a positive sign", ComparableFloat(x));
        } else {
            println!("{:#x} does not have a positive sign", ComparableFloat(x));
        }
    }
}

fn demo_float_is_sign_negative(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        if x.is_sign_negative() {
            println!("{x} has a negative sign");
        } else {
            println!("{x} does not have a negative sign");
        }
    }
}

fn demo_float_is_sign_negative_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        if x.is_sign_negative() {
            println!("{:#x} has a negative sign", ComparableFloat(x));
        } else {
            println!("{:#x} does not have a negative sign", ComparableFloat(x));
        }
    }
}

fn demo_float_classify(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        println!("classify({}) = {:?}", x, x.classify());
    }
}

fn demo_float_classify_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        println!(
            "classify({:#x}) = {:?}",
            ComparableFloatRef(&x),
            x.classify()
        );
    }
}

fn demo_float_into_non_nan(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        println!("into_non_nan({}) = {:?}", x.clone(), x.into_non_nan());
    }
}

fn demo_float_into_non_nan_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        println!(
            "into_non_nan({:#x}) = {:?}",
            ComparableFloat(x.clone()),
            x.into_non_nan()
                .map_or("None".to_string(), |f| to_hex_string(&f))
        );
    }
}

fn demo_float_to_non_nan(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        println!("to_non_nan({}) = {:?}", x, x.to_non_nan());
    }
}

fn demo_float_to_non_nan_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        println!(
            "to_non_nan({:#x}) = {:?}",
            ComparableFloatRef(&x),
            x.to_non_nan()
                .map_or("None".to_string(), |f| to_hex_string(&f))
        );
    }
}

fn demo_float_into_finite(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        println!("into_finite({}) = {:?}", x.clone(), x.into_finite());
    }
}

fn demo_float_into_finite_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        println!(
            "into_finite({:#x}) = {:?}",
            ComparableFloat(x.clone()),
            x.into_finite()
                .map_or("None".to_string(), |f| to_hex_string(&f))
        );
    }
}

fn demo_float_to_finite(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        println!("to_finite({}) = {:?}", x, x.to_finite());
    }
}

fn demo_float_to_finite_debug(gm: GenMode, config: &GenConfig, limit: usize) {
    for x in float_gen().get(gm, config).take(limit) {
        println!(
            "to_finite({:#x}) = {:?}",
            ComparableFloatRef(&x),
            x.to_finite()
                .map_or("None".to_string(), |f| to_hex_string(&f))
        );
    }
}

fn benchmark_float_is_nan_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.is_nan()",
        BenchmarkType::LibraryComparison,
        float_gen_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_float_complexity_bucketer("x"),
        &mut [
            ("Malachite", &mut |(_, x)| no_out!(x.is_nan())),
            ("rug", &mut |(x, _)| no_out!(x.is_nan())),
        ],
    );
}

fn benchmark_float_is_finite_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.is_finite()",
        BenchmarkType::LibraryComparison,
        float_gen_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_float_complexity_bucketer("x"),
        &mut [
            ("Malachite", &mut |(_, x)| no_out!(x.is_finite())),
            ("rug", &mut |(x, _)| no_out!(x.is_finite())),
        ],
    );
}

fn benchmark_float_is_infinite_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.is_infinite()",
        BenchmarkType::LibraryComparison,
        float_gen_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_float_complexity_bucketer("x"),
        &mut [
            ("Malachite", &mut |(_, x)| no_out!(x.is_infinite())),
            ("rug", &mut |(x, _)| no_out!(x.is_infinite())),
        ],
    );
}

fn benchmark_float_is_positive_zero(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.is_zero()",
        BenchmarkType::Single,
        float_gen_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_float_complexity_bucketer("x"),
        &mut [("Malachite", &mut |(_, x)| no_out!(x.is_positive_zero()))],
    );
}

fn benchmark_float_is_negative_zero(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.is_zero()",
        BenchmarkType::Single,
        float_gen_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_float_complexity_bucketer("x"),
        &mut [("Malachite", &mut |(_, x)| no_out!(x.is_positive_zero()))],
    );
}

fn benchmark_float_is_zero_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.is_zero()",
        BenchmarkType::LibraryComparison,
        float_gen_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_float_complexity_bucketer("x"),
        &mut [
            ("Malachite", &mut |(_, x)| no_out!(x.is_zero())),
            ("rug", &mut |(x, _)| no_out!(x.is_zero())),
        ],
    );
}

fn benchmark_float_is_normal_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.is_normal()",
        BenchmarkType::LibraryComparison,
        float_gen_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_float_complexity_bucketer("x"),
        &mut [
            ("Malachite", &mut |(_, x)| no_out!(x.is_normal())),
            ("rug", &mut |(x, _)| no_out!(x.is_normal())),
        ],
    );
}

fn benchmark_float_is_sign_positive_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.is_sign_positive()",
        BenchmarkType::LibraryComparison,
        float_gen_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_float_complexity_bucketer("x"),
        &mut [
            ("Malachite", &mut |(_, x)| no_out!(x.is_sign_positive())),
            ("rug", &mut |(x, _)| {
                let _ = !x.is_nan() && x.is_sign_positive();
            }),
        ],
    );
}

fn benchmark_float_is_sign_negative_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.is_sign_negative()",
        BenchmarkType::LibraryComparison,
        float_gen_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_float_complexity_bucketer("x"),
        &mut [
            ("Malachite", &mut |(_, x)| no_out!(x.is_sign_negative())),
            ("rug", &mut |(x, _)| {
                let _ = !x.is_nan() && x.is_sign_negative();
            }),
        ],
    );
}

fn benchmark_float_classify_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.classify()",
        BenchmarkType::LibraryComparison,
        float_gen_rm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &pair_2_float_complexity_bucketer("x"),
        &mut [
            ("Malachite", &mut |(_, x)| no_out!(x.classify())),
            ("rug", &mut |(x, _)| no_out!(x.classify())),
        ],
    );
}

fn benchmark_float_to_non_nan_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.to_non_nan()",
        BenchmarkType::EvaluationStrategy,
        float_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &float_complexity_bucketer("x"),
        &mut [
            ("to_non_nan", &mut |x| no_out!(x.to_non_nan())),
            ("into_non_nan", &mut |x| no_out!(x.into_non_nan())),
        ],
    );
}

fn benchmark_float_to_finite_evaluation_strategy(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Float.to_finite()",
        BenchmarkType::EvaluationStrategy,
        float_gen().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &float_complexity_bucketer("x"),
        &mut [
            ("to_finite", &mut |x| no_out!(x.to_finite())),
            ("into_finite", &mut |x| no_out!(x.into_finite())),
        ],
    );
}
