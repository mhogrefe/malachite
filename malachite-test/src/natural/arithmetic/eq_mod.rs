use std::cmp::max;

use malachite_base::num::arithmetic::traits::{DivisibleBy, EqMod, UnsignedAbs};
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::CheckedFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_nz::integer::Integer;
use malachite_nz::natural::arithmetic::eq_mod::{
    _limbs_eq_limb_mod_naive_1, _limbs_eq_limb_mod_naive_2, _limbs_eq_mod_limb_naive_1,
    _limbs_eq_mod_limb_naive_2, _limbs_eq_mod_naive_1, _limbs_eq_mod_naive_2, limbs_eq_limb_mod,
    limbs_eq_limb_mod_ref_ref, limbs_eq_limb_mod_ref_val, limbs_eq_limb_mod_val_ref,
    limbs_eq_mod_limb_ref_ref, limbs_eq_mod_limb_ref_val, limbs_eq_mod_limb_val_ref,
    limbs_eq_mod_ref_ref_ref, limbs_eq_mod_ref_ref_val, limbs_eq_mod_ref_val_ref,
    limbs_eq_mod_ref_val_val,
};
use malachite_nz::natural::Natural;

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::{
    triples_of_unsigned_vec_unsigned_and_unsigned_vec_var_1,
    triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_8, triples_of_unsigned_vec_var_55,
};
use inputs::natural::{rm_triples_of_naturals, triples_of_naturals};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_eq_limb_mod);
    register_demo!(registry, demo_limbs_eq_limb_mod_val_ref);
    register_demo!(registry, demo_limbs_eq_limb_mod_ref_val);
    register_demo!(registry, demo_limbs_eq_limb_mod_ref_ref);
    register_demo!(registry, demo_limbs_eq_mod_limb_val_ref);
    register_demo!(registry, demo_limbs_eq_mod_limb_ref_val);
    register_demo!(registry, demo_limbs_eq_mod_limb_ref_ref);
    register_demo!(registry, demo_limbs_eq_mod_ref_val_val);
    register_demo!(registry, demo_limbs_eq_mod_ref_val_ref);
    register_demo!(registry, demo_limbs_eq_mod_ref_ref_val);
    register_demo!(registry, demo_limbs_eq_mod_ref_ref_ref);
    register_demo!(registry, demo_natural_eq_natural_mod_natural);
    register_demo!(registry, demo_natural_eq_natural_mod_natural_val_val_ref);
    register_demo!(registry, demo_natural_eq_natural_mod_natural_val_ref_val);
    register_demo!(registry, demo_natural_eq_natural_mod_natural_val_ref_ref);
    register_demo!(registry, demo_natural_eq_natural_mod_natural_ref_val_val);
    register_demo!(registry, demo_natural_eq_natural_mod_natural_ref_val_ref);
    register_demo!(registry, demo_natural_eq_natural_mod_natural_ref_ref_val);
    register_demo!(registry, demo_natural_eq_natural_mod_natural_ref_ref_ref);
    register_bench!(
        registry,
        Small,
        benchmark_limbs_eq_limb_mod_evaluation_strategy
    );
    register_bench!(registry, Small, benchmark_limbs_eq_limb_mod_algorithms);
    register_bench!(
        registry,
        Small,
        benchmark_limbs_eq_mod_limb_evaluation_strategy
    );
    register_bench!(registry, Small, benchmark_limbs_eq_mod_limb_algorithms);
    register_bench!(registry, Small, benchmark_limbs_eq_mod_evaluation_strategy);
    register_bench!(registry, Small, benchmark_limbs_eq_mod_algorithms);
    register_bench!(
        registry,
        Large,
        benchmark_natural_eq_natural_mod_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_eq_natural_mod_natural_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_eq_natural_mod_natural_algorithms
    );
}

fn demo_limbs_eq_limb_mod(gm: GenerationMode, limit: usize) {
    for (mut xs, y, mut modulus) in
        triples_of_unsigned_vec_unsigned_and_unsigned_vec_var_1(gm).take(limit)
    {
        let old_xs = xs.clone();
        let old_modulus = modulus.clone();
        println!(
            "limbs_eq_limb_mod({:?}, {}, {:?}) = {}",
            old_xs,
            y,
            old_modulus,
            limbs_eq_limb_mod(&mut xs, y, &mut modulus)
        );
    }
}

fn demo_limbs_eq_limb_mod_val_ref(gm: GenerationMode, limit: usize) {
    for (mut xs, y, modulus) in
        triples_of_unsigned_vec_unsigned_and_unsigned_vec_var_1(gm).take(limit)
    {
        let old_xs = xs.clone();
        println!(
            "limbs_eq_limb_mod_val_ref({:?}, {}, {:?}) = {}",
            old_xs,
            y,
            modulus,
            limbs_eq_limb_mod_val_ref(&mut xs, y, &modulus)
        );
    }
}

fn demo_limbs_eq_limb_mod_ref_val(gm: GenerationMode, limit: usize) {
    for (xs, y, mut modulus) in
        triples_of_unsigned_vec_unsigned_and_unsigned_vec_var_1(gm).take(limit)
    {
        let old_modulus = modulus.clone();
        println!(
            "limbs_eq_limb_mod_ref_val({:?}, {}, {:?}) = {}",
            xs,
            y,
            old_modulus,
            limbs_eq_limb_mod_ref_val(&xs, y, &mut modulus)
        );
    }
}

fn demo_limbs_eq_limb_mod_ref_ref(gm: GenerationMode, limit: usize) {
    for (xs, y, modulus) in triples_of_unsigned_vec_unsigned_and_unsigned_vec_var_1(gm).take(limit)
    {
        println!(
            "limbs_eq_limb_mod_ref_ref({:?}, {}, {:?}) = {}",
            xs,
            y,
            modulus,
            limbs_eq_limb_mod_ref_ref(&xs, y, &modulus)
        );
    }
}

fn demo_limbs_eq_mod_limb_val_ref(gm: GenerationMode, limit: usize) {
    for (mut xs, ys, modulus) in
        triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_8(gm).take(limit)
    {
        let old_xs = xs.clone();
        println!(
            "limbs_eq_mod_limb_val_ref({:?}, {:?}, {}) = {}",
            old_xs,
            ys,
            modulus,
            limbs_eq_mod_limb_val_ref(&mut xs, &ys, modulus)
        );
    }
}

fn demo_limbs_eq_mod_limb_ref_val(gm: GenerationMode, limit: usize) {
    for (xs, mut ys, modulus) in
        triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_8(gm).take(limit)
    {
        let old_ys = ys.clone();
        println!(
            "limbs_eq_mod_limb_ref_val({:?}, {:?}, {}) = {}",
            xs,
            old_ys,
            modulus,
            limbs_eq_mod_limb_ref_val(&xs, &mut ys, modulus)
        );
    }
}

fn demo_limbs_eq_mod_limb_ref_ref(gm: GenerationMode, limit: usize) {
    for (xs, ys, modulus) in triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_8(gm).take(limit)
    {
        println!(
            "limbs_eq_mod_limb_ref_ref({:?}, {:?}, {}) = {}",
            xs,
            ys,
            modulus,
            limbs_eq_mod_limb_ref_ref(&xs, &ys, modulus)
        );
    }
}

fn demo_limbs_eq_mod_ref_val_val(gm: GenerationMode, limit: usize) {
    for (xs, mut ys, mut modulus) in triples_of_unsigned_vec_var_55(gm).take(limit) {
        let old_ys = ys.clone();
        let old_modulus = modulus.clone();
        println!(
            "limbs_eq_mod_ref_val_val({:?}, {:?}, {:?}) = {}",
            xs,
            old_ys,
            old_modulus,
            limbs_eq_mod_ref_val_val(&xs, &mut ys, &mut modulus)
        );
    }
}

fn demo_limbs_eq_mod_ref_val_ref(gm: GenerationMode, limit: usize) {
    for (xs, mut ys, modulus) in triples_of_unsigned_vec_var_55(gm).take(limit) {
        let old_ys = ys.clone();
        println!(
            "limbs_eq_mod_ref_val_ref({:?}, {:?}, {:?}) = {}",
            xs,
            old_ys,
            modulus,
            limbs_eq_mod_ref_val_ref(&xs, &mut ys, &modulus)
        );
    }
}

fn demo_limbs_eq_mod_ref_ref_val(gm: GenerationMode, limit: usize) {
    for (xs, ys, mut modulus) in triples_of_unsigned_vec_var_55(gm).take(limit) {
        let old_modulus = modulus.clone();
        println!(
            "limbs_eq_mod_ref_ref_val({:?}, {:?}, {:?}) = {}",
            xs,
            ys,
            old_modulus,
            limbs_eq_mod_ref_ref_val(&xs, &ys, &mut modulus)
        );
    }
}

fn demo_limbs_eq_mod_ref_ref_ref(gm: GenerationMode, limit: usize) {
    for (xs, ys, modulus) in triples_of_unsigned_vec_var_55(gm).take(limit) {
        println!(
            "limbs_eq_mod_ref_ref_ref({:?}, {:?}, {:?}) = {}",
            xs,
            ys,
            modulus,
            limbs_eq_mod_ref_ref_ref(&xs, &ys, &modulus)
        );
    }
}

fn demo_natural_eq_natural_mod_natural(gm: GenerationMode, limit: usize) {
    for (x, y, m) in triples_of_naturals(gm).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        let m_old = m.clone();
        if x.eq_mod(y, m) {
            println!("{} is equal to {} mod {}", x_old, y_old, m_old);
        } else {
            println!("{} is not equal to {} mod {}", x_old, y_old, m_old);
        }
    }
}

fn demo_natural_eq_natural_mod_natural_val_val_ref(gm: GenerationMode, limit: usize) {
    for (x, y, m) in triples_of_naturals(gm).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        if x.eq_mod(y, &m) {
            println!("{} is equal to {} mod &{}", x_old, y_old, m);
        } else {
            println!("{} is not equal to {} mod &{}", x_old, y_old, m);
        }
    }
}

fn demo_natural_eq_natural_mod_natural_val_ref_val(gm: GenerationMode, limit: usize) {
    for (x, y, m) in triples_of_naturals(gm).take(limit) {
        let x_old = x.clone();
        let m_old = m.clone();
        if x.eq_mod(&y, m) {
            println!("{} is equal to &{} mod {}", x_old, y, m_old);
        } else {
            println!("{} is not equal to &{} mod {}", x_old, y, m_old);
        }
    }
}

fn demo_natural_eq_natural_mod_natural_val_ref_ref(gm: GenerationMode, limit: usize) {
    for (x, y, m) in triples_of_naturals(gm).take(limit) {
        let x_old = x.clone();
        if x.eq_mod(&y, &m) {
            println!("{} is equal to &{} mod &{}", x_old, y, m);
        } else {
            println!("{} is not equal to &{} mod &{}", x_old, y, m);
        }
    }
}

fn demo_natural_eq_natural_mod_natural_ref_val_val(gm: GenerationMode, limit: usize) {
    for (x, y, m) in triples_of_naturals(gm).take(limit) {
        let y_old = y.clone();
        let m_old = m.clone();
        if (&x).eq_mod(y, m) {
            println!("&{} is equal to {} mod {}", x, y_old, m_old);
        } else {
            println!("&{} is not equal to {} mod {}", x, y_old, m_old);
        }
    }
}

fn demo_natural_eq_natural_mod_natural_ref_val_ref(gm: GenerationMode, limit: usize) {
    for (x, y, m) in triples_of_naturals(gm).take(limit) {
        let y_old = y.clone();
        if (&x).eq_mod(y, &m) {
            println!("&{} is equal to {} mod &{}", x, y_old, m);
        } else {
            println!("&{} is not equal to {} mod &{}", x, y_old, m);
        }
    }
}

fn demo_natural_eq_natural_mod_natural_ref_ref_val(gm: GenerationMode, limit: usize) {
    for (x, y, m) in triples_of_naturals(gm).take(limit) {
        let m_old = m.clone();
        if (&x).eq_mod(&y, m) {
            println!("&{} is equal to &{} mod {}", x, y, m_old);
        } else {
            println!("&{} is not equal to &{} mod {}", x, y, m_old);
        }
    }
}

fn demo_natural_eq_natural_mod_natural_ref_ref_ref(gm: GenerationMode, limit: usize) {
    for (x, y, m) in triples_of_naturals(gm).take(limit) {
        if (&x).eq_mod(&y, &m) {
            println!("&{} is equal to &{} mod &{}", x, y, m);
        } else {
            println!("&{} is not equal to &{} mod &{}", x, y, m);
        }
    }
}

fn benchmark_limbs_eq_limb_mod_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "limbs_eq_limb_mod(&[Limb], Limb, &[Limb])",
        BenchmarkType::Algorithms,
        triples_of_unsigned_vec_unsigned_and_unsigned_vec_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref xs, _, _)| xs.len()),
        "xs.len()",
        &mut [
            (
                "limbs_eq_limb_mod",
                &mut (|(ref mut xs, y, ref mut modulus)| {
                    no_out!(limbs_eq_limb_mod(xs, y, modulus))
                }),
            ),
            (
                "limbs_eq_limb_mod_val_ref",
                &mut (|(ref mut xs, y, ref modulus)| {
                    no_out!(limbs_eq_limb_mod_val_ref(xs, y, modulus))
                }),
            ),
            (
                "limbs_eq_limb_mod_ref_val",
                &mut (|(ref xs, y, ref mut modulus)| {
                    no_out!(limbs_eq_limb_mod_ref_val(xs, y, modulus))
                }),
            ),
            (
                "limbs_eq_limb_mod_ref_ref",
                &mut (|(ref xs, y, ref modulus)| {
                    no_out!(limbs_eq_limb_mod_ref_ref(xs, y, modulus))
                }),
            ),
        ],
    );
}

fn benchmark_limbs_eq_limb_mod_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_eq_limb_mod_ref_ref(&[Limb], Limb, &[Limb])",
        BenchmarkType::Algorithms,
        triples_of_unsigned_vec_unsigned_and_unsigned_vec_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref xs, _, _)| xs.len()),
        "xs.len()",
        &mut [
            (
                "standard",
                &mut (|(ref xs, y, ref modulus)| {
                    no_out!(limbs_eq_limb_mod_ref_ref(xs, y, modulus))
                }),
            ),
            (
                "naive 1",
                &mut (|(ref xs, y, ref modulus)| {
                    no_out!(_limbs_eq_limb_mod_naive_1(xs, y, modulus))
                }),
            ),
            (
                "naive 2",
                &mut (|(ref xs, y, ref modulus)| {
                    no_out!(_limbs_eq_limb_mod_naive_2(xs, y, modulus))
                }),
            ),
        ],
    );
}

fn benchmark_limbs_eq_mod_limb_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "limbs_eq_mod_limb_val_ref(&mut [Limb], &[Limb], Limb)",
        BenchmarkType::EvaluationStrategy,
        triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_8(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref xs, ref ys, _)| max(xs.len(), ys.len())),
        "max(xs.len(), ys.len())",
        &mut [
            (
                "limbs_eq_mod_limb_val_ref",
                &mut (|(ref mut xs, ref ys, modulus)| {
                    no_out!(limbs_eq_mod_limb_val_ref(xs, ys, modulus))
                }),
            ),
            (
                "limbs_eq_mod_limb_ref_val",
                &mut (|(ref xs, ref mut ys, modulus)| {
                    no_out!(limbs_eq_mod_limb_ref_val(xs, ys, modulus))
                }),
            ),
            (
                "limbs_eq_mod_limb_ref_ref",
                &mut (|(ref xs, ref ys, modulus)| {
                    no_out!(limbs_eq_mod_limb_ref_ref(xs, ys, modulus))
                }),
            ),
        ],
    );
}

fn benchmark_limbs_eq_mod_limb_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_eq_mod_limb_ref_ref(&[Limb], &[Limb], Limb)",
        BenchmarkType::Algorithms,
        triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_8(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref xs, ref ys, _)| max(xs.len(), ys.len())),
        "max(xs.len(), ys.len())",
        &mut [
            (
                "standard",
                &mut (|(ref xs, ref ys, modulus)| {
                    no_out!(limbs_eq_mod_limb_ref_ref(xs, ys, modulus))
                }),
            ),
            (
                "naive 1",
                &mut (|(ref xs, ref ys, modulus)| {
                    no_out!(_limbs_eq_mod_limb_naive_1(xs, ys, modulus))
                }),
            ),
            (
                "naive 2",
                &mut (|(ref xs, ref ys, modulus)| {
                    no_out!(_limbs_eq_mod_limb_naive_2(xs, ys, modulus))
                }),
            ),
        ],
    );
}

fn benchmark_limbs_eq_mod_evaluation_strategy(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_eq_mod_ref_ref_ref(&[Limb], &[Limb], &[Limb])",
        BenchmarkType::EvaluationStrategy,
        triples_of_unsigned_vec_var_55(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref xs, ref ys, _)| max(xs.len(), ys.len())),
        "max(xs.len(), ys.len())",
        &mut [
            (
                "limbs_eq_mod_ref_val_val",
                &mut (|(ref xs, ref mut ys, ref mut modulus)| {
                    no_out!(limbs_eq_mod_ref_val_val(xs, ys, modulus))
                }),
            ),
            (
                "limbs_eq_mod_ref_val_ref",
                &mut (|(ref xs, ref mut ys, ref modulus)| {
                    no_out!(limbs_eq_mod_ref_val_ref(xs, ys, modulus))
                }),
            ),
            (
                "limbs_eq_mod_ref_ref_val",
                &mut (|(ref xs, ref ys, ref mut modulus)| {
                    no_out!(limbs_eq_mod_ref_ref_val(xs, ys, modulus))
                }),
            ),
            (
                "limbs_eq_mod_ref_ref_ref",
                &mut (|(ref xs, ref ys, ref modulus)| {
                    no_out!(limbs_eq_mod_ref_ref_ref(xs, ys, modulus))
                }),
            ),
        ],
    );
}

fn benchmark_limbs_eq_mod_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_eq_mod_ref_ref_ref(&[Limb], &[Limb], &[Limb])",
        BenchmarkType::Algorithms,
        triples_of_unsigned_vec_var_55(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref xs, ref ys, _)| max(xs.len(), ys.len())),
        "max(xs.len(), ys.len())",
        &mut [
            (
                "standard",
                &mut (|(ref xs, ref ys, ref modulus)| {
                    no_out!(limbs_eq_mod_ref_ref_ref(xs, ys, modulus))
                }),
            ),
            (
                "naive 1",
                &mut (|(ref xs, ref ys, ref modulus)| {
                    no_out!(_limbs_eq_mod_naive_1(xs, ys, modulus))
                }),
            ),
            (
                "naive 2",
                &mut (|(ref xs, ref ys, ref modulus)| {
                    no_out!(_limbs_eq_mod_naive_2(xs, ys, modulus))
                }),
            ),
        ],
    );
}

fn benchmark_natural_eq_natural_mod_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.eq_mod(Natural, Natural)",
        BenchmarkType::EvaluationStrategy,
        triples_of_naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref x, ref y, _)| {
            usize::checked_from(max(x.significant_bits(), y.significant_bits())).unwrap()
        }),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [
            (
                "Natural.eq_mod(Natural, Natural)",
                &mut (|(x, y, m)| no_out!(x.eq_mod(y, m))),
            ),
            (
                "Natural.eq_mod(Natural, &Natural)",
                &mut (|(x, y, m)| no_out!(x.eq_mod(y, &m))),
            ),
            (
                "Natural.eq_mod(&Natural, Natural)",
                &mut (|(x, y, m)| no_out!(x.eq_mod(&y, m))),
            ),
            (
                "Natural.eq_mod(&Natural, &Natural)",
                &mut (|(x, y, m)| no_out!(x.eq_mod(&y, &m))),
            ),
            (
                "(&Natural).eq_mod(Natural, Natural)",
                &mut (|(x, y, m)| no_out!((&x).eq_mod(y, m))),
            ),
            (
                "(&Natural).eq_mod(Natural, &Natural)",
                &mut (|(x, y, m)| no_out!((&x).eq_mod(y, &m))),
            ),
            (
                "(&Natural).eq_mod(&Natural, Natural)",
                &mut (|(x, y, m)| no_out!((&x).eq_mod(&y, m))),
            ),
            (
                "(&Natural).eq_mod(&Natural, &Natural)",
                &mut (|(x, y, m)| no_out!((&x).eq_mod(&y, &m))),
            ),
        ],
    );
}

fn benchmark_natural_eq_natural_mod_natural_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.eq_mod(Natural, Natural)",
        BenchmarkType::LibraryComparison,
        rm_triples_of_naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (ref x, ref y, _))| {
            usize::checked_from(max(x.significant_bits(), y.significant_bits())).unwrap()
        }),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [
            ("malachite", &mut (|(_, (x, y, m))| no_out!(x.eq_mod(y, m)))),
            (
                "rug",
                &mut (|((x, y, m), _)| no_out!(x.is_congruent(&y, &m))),
            ),
        ],
    );
}

fn benchmark_natural_eq_natural_mod_natural_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.eq_mod(Natural, Natural)",
        BenchmarkType::Algorithms,
        triples_of_naturals(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref x, ref y, _)| {
            usize::checked_from(max(x.significant_bits(), y.significant_bits())).unwrap()
        }),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [
            (
                "Natural.eq_mod(Natural, Natural)",
                &mut (|(x, y, m)| no_out!(x.eq_mod(y, m))),
            ),
            (
                "Natural == Natural || Natural != 0 && Natural % Natural == Natural % Natural",
                &mut (|(x, y, m)| no_out!(x == y || m != Natural::ZERO && x % &m == y % m)),
            ),
            (
                "|Natural - Natural|.divisible_by(Natural)",
                &mut (|(x, y, m)| {
                    no_out!((Integer::from(x) - Integer::from(y))
                        .unsigned_abs()
                        .divisible_by(m))
                }),
            ),
        ],
    );
}
