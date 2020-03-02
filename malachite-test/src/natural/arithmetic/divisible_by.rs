use malachite_base::num::arithmetic::traits::DivisibleBy;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::slices::slice_test_zero;
use malachite_nz::natural::arithmetic::divisible_by::{
    _combined_limbs_divisible_by_limb, limbs_divisible_by, limbs_divisible_by_limb,
    limbs_divisible_by_ref_ref, limbs_divisible_by_ref_val, limbs_divisible_by_val_ref,
};
use malachite_nz::natural::arithmetic::mod_op::{limbs_mod, limbs_mod_limb};
use num::{BigUint, Integer, Zero as NumZero};

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::{
    pairs_of_limb_vec_var_14, pairs_of_unsigned_vec_and_positive_unsigned_var_1,
    pairs_of_unsigned_vec_var_13,
};
use inputs::natural::{nrm_pairs_of_naturals, pairs_of_naturals};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_divisible_by_limb);
    register_demo!(registry, demo_limbs_divisible_by);
    register_demo!(registry, demo_limbs_divisible_by_val_ref);
    register_demo!(registry, demo_limbs_divisible_by_ref_val);
    register_demo!(registry, demo_limbs_divisible_by_ref_ref);
    register_demo!(registry, demo_natural_divisible_by);
    register_demo!(registry, demo_natural_divisible_by_val_ref);
    register_demo!(registry, demo_natural_divisible_by_ref_val);
    register_demo!(registry, demo_natural_divisible_by_ref_ref);
    register_bench!(
        registry,
        Small,
        benchmark_limbs_divisible_by_limb_algorithms
    );
    register_bench!(registry, Small, benchmark_limbs_divisible_by_algorithms);
    register_bench!(
        registry,
        Small,
        benchmark_limbs_divisible_by_evaluation_strategy
    );
    register_bench!(registry, Large, benchmark_natural_divisible_by_algorithms);
    register_bench!(
        registry,
        Large,
        benchmark_natural_divisible_by_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_divisible_by_library_comparison
    );
}

pub fn num_divisible_by(x: BigUint, y: BigUint) -> bool {
    x == BigUint::zero() || y != BigUint::zero() && x.is_multiple_of(&y)
}

fn demo_limbs_divisible_by_limb(gm: GenerationMode, limit: usize) {
    for (limbs, limb) in pairs_of_unsigned_vec_and_positive_unsigned_var_1(gm).take(limit) {
        println!(
            "limbs_divisible_by_limb({:?}, {}) = {}",
            limbs,
            limb,
            limbs_divisible_by_limb(&limbs, limb)
        );
    }
}

fn demo_limbs_divisible_by(gm: GenerationMode, limit: usize) {
    for (ns, ds) in pairs_of_unsigned_vec_var_13(gm).take(limit) {
        let mut mut_ns = ns.to_vec();
        let mut mut_ds = ds.to_vec();
        println!(
            "limbs_divisible_by({:?}, {:?}) = {}",
            ns,
            ds,
            limbs_divisible_by(&mut mut_ns, &mut mut_ds)
        );
    }
}

fn demo_limbs_divisible_by_val_ref(gm: GenerationMode, limit: usize) {
    for (ns, ds) in pairs_of_unsigned_vec_var_13(gm).take(limit) {
        let mut mut_ns = ns.to_vec();
        println!(
            "limbs_divisible_by_val_ref({:?}, {:?}) = {}",
            ns,
            ds,
            limbs_divisible_by_val_ref(&mut mut_ns, &ds)
        );
    }
}

fn demo_limbs_divisible_by_ref_val(gm: GenerationMode, limit: usize) {
    for (ns, ds) in pairs_of_unsigned_vec_var_13(gm).take(limit) {
        let mut mut_ds = ds.to_vec();
        println!(
            "limbs_divisible_by_ref_val({:?}, {:?}) = {}",
            ns,
            ds,
            limbs_divisible_by_ref_val(&ns, &mut mut_ds)
        );
    }
}

fn demo_limbs_divisible_by_ref_ref(gm: GenerationMode, limit: usize) {
    for (ns, ds) in pairs_of_unsigned_vec_var_13(gm).take(limit) {
        println!(
            "limbs_divisible_by_ref_ref({:?}, {:?}) = {}",
            ns,
            ds,
            limbs_divisible_by_ref_ref(&ns, &ds)
        );
    }
}

fn demo_natural_divisible_by(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_naturals(gm).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        if x.divisible_by(y) {
            println!("{} is divisible by {}", x_old, y_old);
        } else {
            println!("{} is not divisible by {}", x_old, y_old);
        }
    }
}

fn demo_natural_divisible_by_val_ref(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_naturals(gm).take(limit) {
        let x_old = x.clone();
        if x.divisible_by(&y) {
            println!("{} is divisible by {}", x_old, y);
        } else {
            println!("{} is not divisible by {}", x_old, y);
        }
    }
}

fn demo_natural_divisible_by_ref_val(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_naturals(gm).take(limit) {
        let y_old = y.clone();
        if (&x).divisible_by(y) {
            println!("{} is divisible by {}", x, y_old);
        } else {
            println!("{} is not divisible by {}", x, y_old);
        }
    }
}

fn demo_natural_divisible_by_ref_ref(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_naturals(gm).take(limit) {
        if (&x).divisible_by(&y) {
            println!("{} is divisible by {}", x, y);
        } else {
            println!("{} is not divisible by {}", x, y);
        }
    }
}

fn benchmark_limbs_divisible_by_limb_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_divisible_by_limb(&[Limb], Limb)",
        BenchmarkType::Algorithms,
        pairs_of_unsigned_vec_and_positive_unsigned_var_1(gm.with_scale(512)),
        gm.name(),
        limit,
        file_name,
        &(|&(ref limbs, _)| limbs.len()),
        "limbs.len()",
        &mut [
            (
                "limbs_divisible_by_limb",
                &mut (|(ref limbs, limb)| no_out!(limbs_divisible_by_limb(limbs, limb))),
            ),
            (
                "divisibility using limbs_mod_limb",
                &mut (|(ref limbs, limb)| no_out!(limbs_mod_limb(limbs, limb) == 0)),
            ),
            (
                "_combined_limbs_divisible_by_limb",
                &mut (|(ref limbs, limb)| no_out!(_combined_limbs_divisible_by_limb(limbs, limb))),
            ),
        ],
    );
}

fn benchmark_limbs_divisible_by_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_divisible_by(&[Limb], &[Limb])",
        BenchmarkType::Algorithms,
        pairs_of_limb_vec_var_14(gm.with_scale(512)),
        gm.name(),
        limit,
        file_name,
        &(|&(ref ns, _)| ns.len()),
        "limbs.len()",
        &mut [
            (
                "limbs_divisible_by",
                &mut (|(ref mut ns, ref mut ds)| no_out!(limbs_divisible_by(ns, ds))),
            ),
            (
                "divisibility using limbs_mod",
                &mut (|(ref ns, ref ds)| no_out!(slice_test_zero(&limbs_mod(ns, ds)))),
            ),
        ],
    );
}

fn benchmark_limbs_divisible_by_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "limbs_divisible_by(&[Limb], &[Limb])",
        BenchmarkType::EvaluationStrategy,
        pairs_of_unsigned_vec_var_13(gm.with_scale(512)),
        gm.name(),
        limit,
        file_name,
        &(|&(ref ns, _)| ns.len()),
        "limbs.len()",
        &mut [
            (
                "limbs_divisible_by(&mut [Limb], &mut [Limb])",
                &mut (|(ref mut ns, ref mut ds)| no_out!(limbs_divisible_by(ns, ds))),
            ),
            (
                "limbs_divisible_by_val_ref(&mut [Limb], &mut [Limb])",
                &mut (|(ref mut ns, ref ds)| no_out!(limbs_divisible_by_val_ref(ns, ds))),
            ),
            (
                "limbs_divisible_by_ref_val(&mut [Limb], &mut [Limb])",
                &mut (|(ref ns, ref mut ds)| no_out!(limbs_divisible_by_ref_val(ns, ds))),
            ),
            (
                "limbs_divisible_by_ref_ref(&mut [Limb], &mut [Limb])",
                &mut (|(ref ns, ref ds)| no_out!(limbs_divisible_by_ref_ref(ns, ds))),
            ),
        ],
    );
}

fn benchmark_natural_divisible_by_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural.divisible_by(Natural)",
        BenchmarkType::Algorithms,
        pairs_of_naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref x, _)| usize::exact_from(x.significant_bits())),
        "x.significant_bits()",
        &mut [
            ("standard", &mut (|(x, y)| no_out!(x.divisible_by(y)))),
            (
                "using %",
                &mut (|(x, y)| no_out!(x == 0 || y != 0 && x % y == 0)),
            ),
        ],
    );
}

fn benchmark_natural_divisible_by_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.divisible_by(Natural)",
        BenchmarkType::EvaluationStrategy,
        pairs_of_naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref x, _)| usize::exact_from(x.significant_bits())),
        "x.significant_bits()",
        &mut [
            (
                "Natural.divisible_by(Natural)",
                &mut (|(x, y)| no_out!(x.divisible_by(y))),
            ),
            (
                "Natural.divisible_by(&Natural)",
                &mut (|(x, y)| no_out!(x.divisible_by(&y))),
            ),
            (
                "(&Natural).divisible_by(Natural)",
                &mut (|(x, y)| no_out!((&x).divisible_by(y))),
            ),
            (
                "(&Natural).divisible_by(&Natural)",
                &mut (|(x, y)| no_out!((&x).divisible_by(&y))),
            ),
        ],
    );
}

fn benchmark_natural_divisible_by_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.divisible_by(Natural)",
        BenchmarkType::LibraryComparison,
        nrm_pairs_of_naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, (ref x, _))| usize::exact_from(x.significant_bits())),
        "y.significant_bits()",
        &mut [
            (
                "malachite",
                &mut (|(_, _, (x, y))| no_out!(x.divisible_by(y))),
            ),
            (
                "num",
                &mut (|((x, y), _, _)| no_out!(num_divisible_by(x, y))),
            ),
            ("rug", &mut (|(_, (x, y), _)| no_out!(x.is_divisible(&y)))),
        ],
    );
}
