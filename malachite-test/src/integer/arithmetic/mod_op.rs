use malachite_base::num::arithmetic::traits::{
    CeilingDivMod, CeilingMod, CeilingModAssign, DivMod, Mod, ModAssign,
};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base_test_util::bench::{run_benchmark_old, BenchmarkType};
use num::Integer;
use rug::ops::RemRounding;

use malachite_test::common::{DemoBenchRegistry, GenerationMode, ScaleType};
use malachite_test::inputs::integer::{
    nrm_pairs_of_integer_and_nonzero_integer, pairs_of_integer_and_nonzero_integer,
    rm_pairs_of_integer_and_nonzero_integer,
};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_integer_mod_assign);
    register_demo!(registry, demo_integer_mod_assign_ref);
    register_demo!(registry, demo_integer_mod);
    register_demo!(registry, demo_integer_mod_val_ref);
    register_demo!(registry, demo_integer_mod_ref_val);
    register_demo!(registry, demo_integer_mod_ref_ref);
    register_demo!(registry, demo_integer_rem_assign);
    register_demo!(registry, demo_integer_rem_assign_ref);
    register_demo!(registry, demo_integer_rem);
    register_demo!(registry, demo_integer_rem_val_ref);
    register_demo!(registry, demo_integer_rem_ref_val);
    register_demo!(registry, demo_integer_rem_ref_ref);
    register_demo!(registry, demo_integer_ceiling_mod_assign);
    register_demo!(registry, demo_integer_ceiling_mod_assign_ref);
    register_demo!(registry, demo_integer_ceiling_mod);
    register_demo!(registry, demo_integer_ceiling_mod_val_ref);
    register_demo!(registry, demo_integer_ceiling_mod_ref_val);
    register_demo!(registry, demo_integer_ceiling_mod_ref_ref);
    register_bench!(
        registry,
        Large,
        benchmark_integer_mod_assign_evaluation_strategy
    );
    register_bench!(registry, Large, benchmark_integer_mod_library_comparison);
    register_bench!(registry, Large, benchmark_integer_mod_algorithms);
    register_bench!(registry, Large, benchmark_integer_mod_evaluation_strategy);
    register_bench!(
        registry,
        Large,
        benchmark_integer_rem_assign_evaluation_strategy
    );
    register_bench!(registry, Large, benchmark_integer_rem_library_comparison);
    register_bench!(registry, Large, benchmark_integer_rem_evaluation_strategy);
    register_bench!(
        registry,
        Large,
        benchmark_integer_ceiling_mod_assign_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_integer_ceiling_mod_library_comparison
    );
    register_bench!(registry, Large, benchmark_integer_ceiling_mod_algorithms);
    register_bench!(
        registry,
        Large,
        benchmark_integer_ceiling_mod_evaluation_strategy
    );
}

fn demo_integer_mod_assign(gm: GenerationMode, limit: usize) {
    for (mut x, y) in pairs_of_integer_and_nonzero_integer(gm).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        x.mod_assign(y);
        println!("x := {}; x.mod_assign({}); x = {}", x_old, y_old, x);
    }
}

fn demo_integer_mod_assign_ref(gm: GenerationMode, limit: usize) {
    for (mut x, y) in pairs_of_integer_and_nonzero_integer(gm).take(limit) {
        let x_old = x.clone();
        x.mod_assign(&y);
        println!("x := {}; x.mod_assign(&{}); x = {}", x_old, y, x);
    }
}

fn demo_integer_mod(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_integer_and_nonzero_integer(gm).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!("{}.mod_op({}) = {}", x_old, y_old, x.mod_op(y));
    }
}

fn demo_integer_mod_val_ref(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_integer_and_nonzero_integer(gm).take(limit) {
        let x_old = x.clone();
        println!("{}.mod_op(&{}) = {}", x_old, y, x.mod_op(&y));
    }
}

fn demo_integer_mod_ref_val(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_integer_and_nonzero_integer(gm).take(limit) {
        let y_old = y.clone();
        println!("(&{}).mod_op({}) = {:?}", x, y_old, (&x).mod_op(y));
    }
}

fn demo_integer_mod_ref_ref(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_integer_and_nonzero_integer(gm).take(limit) {
        println!("(&{}).mod_op(&{}) = {:?}", x, y, (&x).mod_op(&y));
    }
}

fn demo_integer_rem_assign(gm: GenerationMode, limit: usize) {
    for (mut x, y) in pairs_of_integer_and_nonzero_integer(gm).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        x %= y;
        println!("x := {}; x %= {}; x = {}", x_old, y_old, x);
    }
}

fn demo_integer_rem_assign_ref(gm: GenerationMode, limit: usize) {
    for (mut x, y) in pairs_of_integer_and_nonzero_integer(gm).take(limit) {
        let x_old = x.clone();
        x %= &y;
        println!("x := {}; x %= &{}; x = {}", x_old, y, x);
    }
}

fn demo_integer_rem(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_integer_and_nonzero_integer(gm).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!("{} % {} = {:?}", x_old, y_old, x % y);
    }
}

fn demo_integer_rem_val_ref(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_integer_and_nonzero_integer(gm).take(limit) {
        let x_old = x.clone();
        println!("{} % &{} = {:?}", x_old, y, x % &y);
    }
}

fn demo_integer_rem_ref_val(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_integer_and_nonzero_integer(gm).take(limit) {
        let y_old = y.clone();
        println!("&{} % {} = {:?}", x, y_old, &x % y);
    }
}

fn demo_integer_rem_ref_ref(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_integer_and_nonzero_integer(gm).take(limit) {
        println!("&{} % &{} = {:?}", x, y, &x % &y);
    }
}

fn demo_integer_ceiling_mod_assign(gm: GenerationMode, limit: usize) {
    for (mut x, y) in pairs_of_integer_and_nonzero_integer(gm).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        x.ceiling_mod_assign(y);
        println!("x := {}; x.ceiling_mod_assign({}); x = {}", x_old, y_old, x);
    }
}

fn demo_integer_ceiling_mod_assign_ref(gm: GenerationMode, limit: usize) {
    for (mut x, y) in pairs_of_integer_and_nonzero_integer(gm).take(limit) {
        let x_old = x.clone();
        x.ceiling_mod_assign(&y);
        println!("x := {}; x.ceiling_mod_assign(&{}); x = {}", x_old, y, x);
    }
}

fn demo_integer_ceiling_mod(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_integer_and_nonzero_integer(gm).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!("{}.ceiling_mod({}) = {}", x_old, y_old, x.ceiling_mod(y));
    }
}

fn demo_integer_ceiling_mod_val_ref(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_integer_and_nonzero_integer(gm).take(limit) {
        let x_old = x.clone();
        println!("{}.ceiling_mod(&{}) = {}", x_old, y, x.ceiling_mod(&y));
    }
}

fn demo_integer_ceiling_mod_ref_val(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_integer_and_nonzero_integer(gm).take(limit) {
        let y_old = y.clone();
        println!("(&{}).ceiling_mod({}) = {}", x, y_old, (&x).ceiling_mod(y));
    }
}

fn demo_integer_ceiling_mod_ref_ref(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_integer_and_nonzero_integer(gm).take(limit) {
        println!("(&{}).ceiling_mod(&{}) = {}", x, y, (&x).ceiling_mod(&y));
    }
}

fn benchmark_integer_mod_assign_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "Integer.mod_assign(Integer)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_integer_and_nonzero_integer(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            (
                "Integer.mod_assign(Integer)",
                &mut (|(mut x, y)| no_out!(x.mod_assign(y))),
            ),
            (
                "Integer.mod_assign(&Integer)",
                &mut (|(mut x, y)| no_out!(x.mod_assign(&y))),
            ),
        ],
    );
}

fn benchmark_integer_mod_library_comparison(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "Integer.mod_op(Integer)",
        BenchmarkType::LibraryComparison,
        nrm_pairs_of_integer_and_nonzero_integer(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, (ref n, _))| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            ("Malachite", &mut (|(_, _, (x, y))| no_out!(x.mod_op(y)))),
            ("num", &mut (|((x, y), _, _)| no_out!(x.mod_floor(&y)))),
            ("rug", &mut (|(_, (x, y), _)| no_out!(x.rem_floor(y)))),
        ],
    );
}

fn benchmark_integer_mod_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "Integer.mod_op(Integer)",
        BenchmarkType::Algorithms,
        pairs_of_integer_and_nonzero_integer(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            ("standard", &mut (|(x, y)| no_out!(x.mod_op(y)))),
            ("using div_mod", &mut (|(x, y)| no_out!(x.div_mod(y).1))),
        ],
    );
}

fn benchmark_integer_mod_evaluation_strategy(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "Integer.mod_op(Integer)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_integer_and_nonzero_integer(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            (
                "Integer.mod_op(Integer)",
                &mut (|(x, y)| no_out!(x.mod_op(y))),
            ),
            (
                "Integer.mod_op(&Integer)",
                &mut (|(x, y)| no_out!(x.mod_op(&y))),
            ),
            (
                "(&Integer).mod_op(Integer)",
                &mut (|(x, y)| no_out!((&x).mod_op(y))),
            ),
            (
                "(&Integer).mod_op(&Integer)",
                &mut (|(x, y)| no_out!((&x).mod_op(&y))),
            ),
        ],
    );
}

fn benchmark_integer_rem_assign_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "Integer.div_assign_rem(Integer)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_integer_and_nonzero_integer(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            ("Integer %= Integer", &mut (|(mut x, y)| x %= y)),
            ("Integer %= &Integer", &mut (|(mut x, y)| x %= &y)),
        ],
    );
}

fn benchmark_integer_rem_library_comparison(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "Integer % Integer",
        BenchmarkType::LibraryComparison,
        nrm_pairs_of_integer_and_nonzero_integer(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, (ref n, _))| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            ("Malachite", &mut (|(_, _, (x, y))| no_out!(x % y))),
            ("num", &mut (|((x, y), _, _)| no_out!(x % y))),
            ("rug", &mut (|(_, (x, y), _)| no_out!(x % y))),
        ],
    );
}

fn benchmark_integer_rem_evaluation_strategy(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "Integer % Integer",
        BenchmarkType::EvaluationStrategy,
        pairs_of_integer_and_nonzero_integer(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            ("Integer % Integer", &mut (|(x, y)| no_out!(x % y))),
            ("Integer % &Integer", &mut (|(x, y)| no_out!(x % &y))),
            ("&Integer % Integer", &mut (|(x, y)| no_out!(&x % y))),
            ("&Integer % &Integer", &mut (|(x, y)| no_out!(&x % &y))),
        ],
    );
}

fn benchmark_integer_ceiling_mod_assign_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "Integer.ceiling_mod_assign(Integer)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_integer_and_nonzero_integer(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            (
                "Integer.ceiling_mod_assign(Integer)",
                &mut (|(mut x, y)| no_out!(x.ceiling_mod_assign(y))),
            ),
            (
                "Integer.ceiling_mod_assign(&Integer)",
                &mut (|(mut x, y)| no_out!(x.ceiling_mod_assign(&y))),
            ),
        ],
    );
}

fn benchmark_integer_ceiling_mod_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "Integer.ceiling_mod(Integer)",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_integer_and_nonzero_integer(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (ref n, _))| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            ("Malachite", &mut (|(_, (x, y))| no_out!(x.ceiling_mod(y)))),
            ("rug", &mut (|((x, y), _)| no_out!(x.rem_ceil(y)))),
        ],
    );
}

fn benchmark_integer_ceiling_mod_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark_old(
        "Integer.ceiling_mod(Integer)",
        BenchmarkType::Algorithms,
        pairs_of_integer_and_nonzero_integer(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            ("standard", &mut (|(x, y)| no_out!(x.ceiling_mod(y)))),
            (
                "using ceiling_div_mod",
                &mut (|(x, y)| no_out!(x.ceiling_div_mod(y).1)),
            ),
        ],
    );
}

fn benchmark_integer_ceiling_mod_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark_old(
        "Integer.ceiling_mod(Integer)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_integer_and_nonzero_integer(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| usize::exact_from(n.significant_bits())),
        "n.significant_bits()",
        &mut [
            (
                "Integer.ceiling_mod(Integer)",
                &mut (|(x, y)| no_out!(x.ceiling_mod(y))),
            ),
            (
                "Integer.ceiling_mod(&Integer)",
                &mut (|(x, y)| no_out!(x.ceiling_mod(&y))),
            ),
            (
                "(&Integer).ceiling_mod(Integer)",
                &mut (|(x, y)| no_out!((&x).ceiling_mod(y))),
            ),
            (
                "(&Integer).ceiling_mod(&Integer)",
                &mut (|(x, y)| no_out!((&x).ceiling_mod(&y))),
            ),
        ],
    );
}
