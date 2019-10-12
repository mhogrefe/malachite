use malachite_base::num::arithmetic::traits::{DivMod, Mod, ModAssign, NegMod, NegModAssign};
use malachite_base::num::conversion::traits::CheckedFrom;
use malachite_base::num::logic::traits::SignificantBits;
use num::Integer;
use rug::ops::RemRounding;

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::natural::{
    nrm_pairs_of_natural_and_positive_natural, pairs_of_natural_and_positive_natural,
    rm_pairs_of_natural_and_positive_natural,
};

// For `Natural`s, `mod` is equivalent to `rem`.

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_natural_mod_assign);
    register_demo!(registry, demo_natural_mod_assign_ref);
    register_demo!(registry, demo_natural_mod);
    register_demo!(registry, demo_natural_mod_val_ref);
    register_demo!(registry, demo_natural_mod_ref_val);
    register_demo!(registry, demo_natural_mod_ref_ref);
    register_demo!(registry, demo_natural_rem_assign);
    register_demo!(registry, demo_natural_rem_assign_ref);
    register_demo!(registry, demo_natural_rem);
    register_demo!(registry, demo_natural_rem_val_ref);
    register_demo!(registry, demo_natural_rem_ref_val);
    register_demo!(registry, demo_natural_rem_ref_ref);
    register_demo!(registry, demo_natural_neg_mod_assign);
    register_demo!(registry, demo_natural_neg_mod_assign_ref);
    register_demo!(registry, demo_natural_neg_mod);
    register_demo!(registry, demo_natural_neg_mod_val_ref);
    register_demo!(registry, demo_natural_neg_mod_ref_val);
    register_demo!(registry, demo_natural_neg_mod_ref_ref);
    register_bench!(
        registry,
        Large,
        benchmark_natural_mod_assign_evaluation_strategy
    );
    register_bench!(registry, Large, benchmark_natural_mod_library_comparison);
    register_bench!(registry, Large, benchmark_natural_mod_algorithms);
    register_bench!(registry, Large, benchmark_natural_mod_evaluation_strategy);
    register_bench!(
        registry,
        Large,
        benchmark_natural_rem_assign_evaluation_strategy
    );
    register_bench!(registry, Large, benchmark_natural_rem_library_comparison);
    register_bench!(registry, Large, benchmark_natural_rem_evaluation_strategy);
    register_bench!(
        registry,
        Large,
        benchmark_natural_neg_mod_assign_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_neg_mod_library_comparison
    );
    //TODO
    /*
    register_bench!(
        registry,
        Large,
        benchmark_natural_ceiling_div_neg_mod_limb_algorithms
    );*/
    register_bench!(
        registry,
        Large,
        benchmark_natural_neg_mod_evaluation_strategy
    );
}

pub fn rug_neg_mod(x: rug::Integer, y: rug::Integer) -> rug::Integer {
    -x.rem_ceil(y)
}

fn demo_natural_mod_assign(gm: GenerationMode, limit: usize) {
    for (mut x, y) in pairs_of_natural_and_positive_natural(gm).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        x.mod_assign(y);
        println!("x := {}; x.mod_assign({}); x = {}", x_old, y_old, x,);
    }
}

fn demo_natural_mod_assign_ref(gm: GenerationMode, limit: usize) {
    for (mut x, y) in pairs_of_natural_and_positive_natural(gm).take(limit) {
        let x_old = x.clone();
        x.mod_assign(&y);
        println!("x := {}; x.mod_assign(&{}); x = {}", x_old, y, x,);
    }
}

fn demo_natural_mod(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_natural_and_positive_natural(gm).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!("{}.mod_op({}) = {}", x_old, y_old, x.mod_op(y));
    }
}

fn demo_natural_mod_val_ref(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_natural_and_positive_natural(gm).take(limit) {
        let x_old = x.clone();
        println!("{}.mod_op(&{}) = {}", x_old, y, x.mod_op(&y));
    }
}

fn demo_natural_mod_ref_val(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_natural_and_positive_natural(gm).take(limit) {
        let y_old = y.clone();
        println!("(&{}).mod_op({}) = {:?}", x, y_old, (&x).mod_op(y));
    }
}

fn demo_natural_mod_ref_ref(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_natural_and_positive_natural(gm).take(limit) {
        println!("(&{}).mod_op(&{}) = {:?}", x, y, (&x).mod_op(&y));
    }
}

fn demo_natural_rem_assign(gm: GenerationMode, limit: usize) {
    for (mut x, y) in pairs_of_natural_and_positive_natural(gm).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        x %= y;
        println!("x := {}; x %= {}; x = {}", x_old, y_old, x);
    }
}

fn demo_natural_rem_assign_ref(gm: GenerationMode, limit: usize) {
    for (mut x, y) in pairs_of_natural_and_positive_natural(gm).take(limit) {
        let x_old = x.clone();
        x %= &y;
        println!("x := {}; x %= &{}; x = {}", x_old, y, x);
    }
}

fn demo_natural_rem(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_natural_and_positive_natural(gm).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!("{} % {} = {:?}", x_old, y_old, x % y);
    }
}

fn demo_natural_rem_val_ref(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_natural_and_positive_natural(gm).take(limit) {
        let x_old = x.clone();
        println!("{} % &{} = {:?}", x_old, y, x % &y);
    }
}

fn demo_natural_rem_ref_val(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_natural_and_positive_natural(gm).take(limit) {
        let y_old = y.clone();
        println!("&{} % {} = {:?}", x, y_old, &x % y);
    }
}

fn demo_natural_rem_ref_ref(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_natural_and_positive_natural(gm).take(limit) {
        println!("&{} % &{} = {:?}", x, y, &x % &y);
    }
}

fn demo_natural_neg_mod_assign(gm: GenerationMode, limit: usize) {
    for (mut x, y) in pairs_of_natural_and_positive_natural(gm).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        x.neg_mod_assign(y);
        println!("x := {}; x.neg_mod_assign({}); x = {}", x_old, y_old, x);
    }
}

fn demo_natural_neg_mod_assign_ref(gm: GenerationMode, limit: usize) {
    for (mut x, y) in pairs_of_natural_and_positive_natural(gm).take(limit) {
        let x_old = x.clone();
        x.neg_mod_assign(&y);
        println!("x := {}; x.neg_mod_assign(&{}); x = {}", x_old, y, x);
    }
}

fn demo_natural_neg_mod(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_natural_and_positive_natural(gm).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!("{}.neg_mod({}) = {}", x_old, y_old, x.neg_mod(y));
    }
}

fn demo_natural_neg_mod_val_ref(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_natural_and_positive_natural(gm).take(limit) {
        let x_old = x.clone();
        println!("{}.neg_mod(&{}) = {}", x_old, y, x.neg_mod(&y));
    }
}

fn demo_natural_neg_mod_ref_val(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_natural_and_positive_natural(gm).take(limit) {
        let y_old = y.clone();
        println!("(&{}).neg_mod({}) = {}", x, y_old, (&x).neg_mod(y));
    }
}

fn demo_natural_neg_mod_ref_ref(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_natural_and_positive_natural(gm).take(limit) {
        println!("(&{}).neg_mod(&{}) = {}", x, y, (&x).neg_mod(&y));
    }
}

fn benchmark_natural_mod_assign_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.mod_assign(Natural)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_natural_and_positive_natural(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            (
                "Natural.mod_assign(Natural)",
                &mut (|(mut x, y)| no_out!(x.mod_assign(y))),
            ),
            (
                "Natural.mod_assign(&Natural)",
                &mut (|(mut x, y)| no_out!(x.mod_assign(&y))),
            ),
        ],
    );
}

fn benchmark_natural_mod_library_comparison(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural.mod_op(Natural)",
        BenchmarkType::LibraryComparison,
        nrm_pairs_of_natural_and_positive_natural(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, (ref n, _))| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, _, (x, y))| no_out!(x.mod_op(y)))),
            ("num", &mut (|((x, y), _, _)| no_out!(x.mod_floor(&y)))),
            ("rug", &mut (|(_, (x, y), _)| no_out!(x.rem_floor(y)))),
        ],
    );
}

fn benchmark_natural_mod_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural.mod_op(Natural)",
        BenchmarkType::Algorithms,
        pairs_of_natural_and_positive_natural(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            ("standard", &mut (|(x, y)| no_out!(x.mod_op(y)))),
            ("using div_mod", &mut (|(x, y)| no_out!(x.div_mod(y).1))),
        ],
    );
}

fn benchmark_natural_mod_evaluation_strategy(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural.mod_op(Natural)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_natural_and_positive_natural(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            (
                "Natural.mod_op(Natural)",
                &mut (|(x, y)| no_out!(x.mod_op(y))),
            ),
            (
                "Natural.mod_op(&Natural)",
                &mut (|(x, y)| no_out!(x.mod_op(&y))),
            ),
            (
                "(&Natural).mod_op(Natural)",
                &mut (|(x, y)| no_out!((&x).mod_op(y))),
            ),
            (
                "(&Natural).mod_op(&Natural)",
                &mut (|(x, y)| no_out!((&x).mod_op(&y))),
            ),
        ],
    );
}

fn benchmark_natural_rem_assign_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.div_assign_rem(Natural)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_natural_and_positive_natural(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            ("Natural %= Natural", &mut (|(mut x, y)| x %= y)),
            ("Natural %= &Natural", &mut (|(mut x, y)| x %= &y)),
        ],
    );
}

fn benchmark_natural_rem_library_comparison(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural % Natural",
        BenchmarkType::LibraryComparison,
        nrm_pairs_of_natural_and_positive_natural(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, (ref n, _))| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, _, (x, y))| no_out!(x % y))),
            ("num", &mut (|((x, y), _, _)| no_out!(x % y))),
            ("rug", &mut (|(_, (x, y), _)| no_out!(x % y))),
        ],
    );
}

fn benchmark_natural_rem_evaluation_strategy(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural % Natural",
        BenchmarkType::EvaluationStrategy,
        pairs_of_natural_and_positive_natural(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            ("Natural % Natural", &mut (|(x, y)| no_out!(x % y))),
            ("Natural % &Natural", &mut (|(x, y)| no_out!(x % &y))),
            ("&Natural % Natural", &mut (|(x, y)| no_out!(&x % y))),
            ("&Natural % &Natural", &mut (|(x, y)| no_out!(&x % &y))),
        ],
    );
}

fn benchmark_natural_neg_mod_assign_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.neg_mod_assign(Natural)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_natural_and_positive_natural(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            (
                "Natural.neg_mod_assign(Natural)",
                &mut (|(mut x, y)| no_out!(x.neg_mod_assign(y))),
            ),
            (
                "Natural.neg_mod_assign(&Natural)",
                &mut (|(mut x, y)| no_out!(x.neg_mod_assign(&y))),
            ),
        ],
    );
}

fn benchmark_natural_neg_mod_library_comparison(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural.neg_mod(Natural)",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_natural_and_positive_natural(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (ref n, _))| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, (x, y))| no_out!(x.neg_mod(y)))),
            ("rug", &mut (|((x, y), _)| no_out!(rug_neg_mod(x, y)))),
        ],
    );
}

//TODO
/*
fn benchmark_natural_ceiling_div_neg_mod_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.ceiling_div_neg_mod(Natural)",
        BenchmarkType::Algorithms,
        pairs_of_natural_and_positive_natural(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            (
                "standard",
                &mut (|(x, y)| no_out!(x.ceiling_div_neg_mod(y))),
            ),
            (
                "using div_round and %",
                &mut (|(x, y)| {
                    let remainder = (&x).neg_mod(y);
                    (x.div_round(y, RoundingMode::Ceiling), remainder);
                }),
            ),
        ],
    );
}*/

fn benchmark_natural_neg_mod_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.neg_mod(Natural)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_natural_and_positive_natural(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| usize::checked_from(n.significant_bits()).unwrap()),
        "n.significant_bits()",
        &mut [
            (
                "Natural.neg_mod(Natural)",
                &mut (|(x, y)| no_out!(x.neg_mod(y))),
            ),
            (
                "Natural.neg_mod(&Natural)",
                &mut (|(x, y)| no_out!(x.neg_mod(&y))),
            ),
            (
                "(&Natural).neg_mod(Natural)",
                &mut (|(x, y)| no_out!((&x).neg_mod(y))),
            ),
            (
                "(&Natural).neg_mod(&Natural)",
                &mut (|(x, y)| no_out!((&x).neg_mod(&y))),
            ),
        ],
    );
}
