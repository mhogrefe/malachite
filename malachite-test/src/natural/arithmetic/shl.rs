use malachite_base::named::Named;
use malachite_base::num::arithmetic::traits::UnsignedAbs;
use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_nz::natural::arithmetic::shl::{
    limbs_shl, limbs_shl_to_out, limbs_shl_with_complement_to_out, limbs_slice_shl_in_place,
    limbs_vec_shl_in_place,
};
use malachite_nz::natural::logic::not::limbs_not_in_place;
use malachite_nz::platform::Limb;

use malachite_test::common::{DemoBenchRegistry, GenerationMode, ScaleType};
use malachite_test::inputs::base::{
    pairs_of_unsigned_vec_and_small_unsigned, pairs_of_unsigned_vec_and_u64_var_1,
    triples_of_unsigned_vec_unsigned_vec_and_u64_var_5,
    triples_of_unsigned_vec_unsigned_vec_and_u64_var_6,
};
use malachite_test::inputs::natural::{
    pairs_of_natural_and_small_signed, pairs_of_natural_and_small_unsigned,
    rm_pairs_of_natural_and_small_signed, rm_pairs_of_natural_and_small_unsigned,
};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_shl);
    register_demo!(registry, demo_limbs_shl_to_out);
    register_demo!(registry, demo_limbs_slice_shl_in_place);
    register_demo!(registry, demo_limbs_vec_shl_in_place);
    register_demo!(registry, demo_limbs_shl_with_complement_to_out);

    register_demo!(registry, demo_natural_shl_assign_u8);
    register_demo!(registry, demo_natural_shl_assign_u16);
    register_demo!(registry, demo_natural_shl_assign_u32);
    register_demo!(registry, demo_natural_shl_assign_u64);
    register_demo!(registry, demo_natural_shl_assign_usize);
    register_demo!(registry, demo_natural_shl_assign_i8);
    register_demo!(registry, demo_natural_shl_assign_i16);
    register_demo!(registry, demo_natural_shl_assign_i32);
    register_demo!(registry, demo_natural_shl_assign_i64);
    register_demo!(registry, demo_natural_shl_assign_isize);

    register_demo!(registry, demo_natural_shl_u8);
    register_demo!(registry, demo_natural_shl_u16);
    register_demo!(registry, demo_natural_shl_u32);
    register_demo!(registry, demo_natural_shl_u64);
    register_demo!(registry, demo_natural_shl_usize);
    register_demo!(registry, demo_natural_shl_i8);
    register_demo!(registry, demo_natural_shl_i16);
    register_demo!(registry, demo_natural_shl_i32);
    register_demo!(registry, demo_natural_shl_i64);
    register_demo!(registry, demo_natural_shl_isize);

    register_demo!(registry, demo_natural_shl_u8_ref);
    register_demo!(registry, demo_natural_shl_u16_ref);
    register_demo!(registry, demo_natural_shl_u32_ref);
    register_demo!(registry, demo_natural_shl_u64_ref);
    register_demo!(registry, demo_natural_shl_usize_ref);
    register_demo!(registry, demo_natural_shl_i8_ref);
    register_demo!(registry, demo_natural_shl_i16_ref);
    register_demo!(registry, demo_natural_shl_i32_ref);
    register_demo!(registry, demo_natural_shl_i64_ref);
    register_demo!(registry, demo_natural_shl_isize_ref);

    register_bench!(registry, Small, benchmark_limbs_shl);
    register_bench!(registry, Small, benchmark_limbs_shl_to_out);
    register_bench!(registry, Small, benchmark_limbs_slice_shl_in_place);
    register_bench!(registry, Small, benchmark_limbs_vec_shl_in_place);
    register_bench!(
        registry,
        Small,
        benchmark_limbs_shl_with_complement_to_out_algorithms
    );

    register_bench!(
        registry,
        Large,
        benchmark_natural_shl_assign_u32_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_shl_u32_library_comparison
    );

    register_bench!(
        registry,
        Large,
        benchmark_natural_shl_u8_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_shl_u16_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_shl_u32_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_shl_u64_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_shl_usize_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_shl_i8_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_shl_i16_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_shl_i32_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_shl_i64_evaluation_strategy
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_shl_isize_evaluation_strategy
    );

    register_bench!(
        registry,
        Large,
        benchmark_natural_shl_assign_i32_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_shl_i32_library_comparison
    );
}

fn demo_limbs_shl(gm: GenerationMode, limit: usize) {
    for (limbs, bits) in pairs_of_unsigned_vec_and_small_unsigned(gm).take(limit) {
        println!(
            "limbs_shl({:?}, {}) = {:?}",
            limbs,
            bits,
            limbs_shl(&limbs, bits)
        );
    }
}

fn demo_limbs_shl_to_out(gm: GenerationMode, limit: usize) {
    for (out, in_limbs, bits) in triples_of_unsigned_vec_unsigned_vec_and_u64_var_5(gm).take(limit)
    {
        let mut out = out.to_vec();
        let out_old = out.clone();
        let carry = limbs_shl_to_out(&mut out, &in_limbs, bits);
        println!(
            "out := {:?}; limbs_shl_to_out(&mut out, {:?}, {}) = {}; out = {:?}",
            out_old, in_limbs, bits, carry, out
        );
    }
}

fn demo_limbs_slice_shl_in_place(gm: GenerationMode, limit: usize) {
    for (limbs, bits) in pairs_of_unsigned_vec_and_u64_var_1(gm).take(limit) {
        let mut limbs = limbs.to_vec();
        let limbs_old = limbs.clone();
        let carry = limbs_slice_shl_in_place(&mut limbs, bits);
        println!(
            "limbs := {:?}; limbs_slice_shl_in_place(&mut limbs, {}) = {}; limbs = {:?}",
            limbs_old, bits, carry, limbs
        );
    }
}

fn demo_limbs_vec_shl_in_place(gm: GenerationMode, limit: usize) {
    for (limbs, bits) in pairs_of_unsigned_vec_and_small_unsigned(gm).take(limit) {
        let mut limbs = limbs.to_vec();
        let limbs_old = limbs.clone();
        limbs_vec_shl_in_place(&mut limbs, bits);
        println!(
            "limbs := {:?}; limbs_vec_shl_in_place(&mut limbs, {}); limbs = {:?}",
            limbs_old, bits, limbs
        );
    }
}

fn demo_limbs_shl_with_complement_to_out(gm: GenerationMode, limit: usize) {
    for (out, in_limbs, bits) in triples_of_unsigned_vec_unsigned_vec_and_u64_var_6(gm).take(limit)
    {
        let mut out = out.to_vec();
        let out_old = out.clone();
        let carry = limbs_shl_with_complement_to_out(&mut out, &in_limbs, bits);
        println!(
            "out := {:?}; limbs_shl_with_complement_to_out(&mut out, {:?}, {}) = {}; out = {:?}",
            out_old, in_limbs, bits, carry, out
        );
    }
}

macro_rules! demos_and_benches_unsigned {
    (
        $t:ident,
        $demo_natural_shl_assign_u:ident,
        $demo_natural_shl_u:ident,
        $demo_natural_shl_u_ref:ident,
        $benchmark_natural_shl_u_evaluation_strategy:ident
    ) => {
        fn $demo_natural_shl_assign_u(gm: GenerationMode, limit: usize) {
            for (mut n, u) in pairs_of_natural_and_small_unsigned::<$t>(gm).take(limit) {
                let n_old = n.clone();
                n <<= u;
                println!("x := {}; x <<= {}; x = {}", n_old, u, n);
            }
        }

        fn $demo_natural_shl_u(gm: GenerationMode, limit: usize) {
            for (n, u) in pairs_of_natural_and_small_unsigned::<$t>(gm).take(limit) {
                let n_old = n.clone();
                println!("{} << {} = {}", n_old, u, n << u);
            }
        }

        fn $demo_natural_shl_u_ref(gm: GenerationMode, limit: usize) {
            for (n, u) in pairs_of_natural_and_small_unsigned::<$t>(gm).take(limit) {
                println!("&{} << {} = {}", n, u, &n << u);
            }
        }

        fn $benchmark_natural_shl_u_evaluation_strategy(
            gm: GenerationMode,
            limit: usize,
            file_name: &str,
        ) {
            run_benchmark(
                &format!("Natural << {}", $t::NAME),
                BenchmarkType::EvaluationStrategy,
                pairs_of_natural_and_small_unsigned::<$t>(gm),
                gm.name(),
                limit,
                file_name,
                &(|&(_, other)| usize::exact_from(other)),
                "other",
                &mut [
                    (
                        &format!("Natural << {}", $t::NAME),
                        &mut (|(x, y)| no_out!(x << y)),
                    ),
                    (
                        &format!("&Natural << {}", $t::NAME),
                        &mut (|(x, y)| no_out!(&x << y)),
                    ),
                ],
            );
        }
    };
}
demos_and_benches_unsigned!(
    u8,
    demo_natural_shl_assign_u8,
    demo_natural_shl_u8,
    demo_natural_shl_u8_ref,
    benchmark_natural_shl_u8_evaluation_strategy
);
demos_and_benches_unsigned!(
    u16,
    demo_natural_shl_assign_u16,
    demo_natural_shl_u16,
    demo_natural_shl_u16_ref,
    benchmark_natural_shl_u16_evaluation_strategy
);
demos_and_benches_unsigned!(
    u32,
    demo_natural_shl_assign_u32,
    demo_natural_shl_u32,
    demo_natural_shl_u32_ref,
    benchmark_natural_shl_u32_evaluation_strategy
);
demos_and_benches_unsigned!(
    u64,
    demo_natural_shl_assign_u64,
    demo_natural_shl_u64,
    demo_natural_shl_u64_ref,
    benchmark_natural_shl_u64_evaluation_strategy
);
demos_and_benches_unsigned!(
    usize,
    demo_natural_shl_assign_usize,
    demo_natural_shl_usize,
    demo_natural_shl_usize_ref,
    benchmark_natural_shl_usize_evaluation_strategy
);

macro_rules! demos_and_benches_signed {
    (
        $t:ident,
        $demo_natural_shl_assign_i:ident,
        $demo_natural_shl_i:ident,
        $demo_natural_shl_i_ref:ident,
        $benchmark_natural_shl_i_evaluation_strategy:ident
    ) => {
        fn $demo_natural_shl_assign_i(gm: GenerationMode, limit: usize) {
            for (mut n, i) in pairs_of_natural_and_small_signed::<$t>(gm).take(limit) {
                let n_old = n.clone();
                n <<= i;
                println!("x := {}; x <<= {}; x = {}", n_old, i, n);
            }
        }

        fn $demo_natural_shl_i(gm: GenerationMode, limit: usize) {
            for (n, i) in pairs_of_natural_and_small_signed::<$t>(gm).take(limit) {
                let n_old = n.clone();
                println!("{} << {} = {}", n_old, i, n << i);
            }
        }

        fn $demo_natural_shl_i_ref(gm: GenerationMode, limit: usize) {
            for (n, i) in pairs_of_natural_and_small_signed::<$t>(gm).take(limit) {
                println!("&{} << {} = {}", n, i, &n << i);
            }
        }

        fn $benchmark_natural_shl_i_evaluation_strategy(
            gm: GenerationMode,
            limit: usize,
            file_name: &str,
        ) {
            run_benchmark(
                &format!("Natural << {}", $t::NAME),
                BenchmarkType::EvaluationStrategy,
                pairs_of_natural_and_small_signed::<$t>(gm),
                gm.name(),
                limit,
                file_name,
                &(|&(_, other)| usize::exact_from(other.unsigned_abs())),
                "|other|",
                &mut [
                    (
                        &format!("Natural << {}", $t::NAME),
                        &mut (|(x, y)| no_out!(x << y)),
                    ),
                    (
                        &format!("&Natural << {}", $t::NAME),
                        &mut (|(x, y)| no_out!(&x << y)),
                    ),
                ],
            );
        }
    };
}
demos_and_benches_signed!(
    i8,
    demo_natural_shl_assign_i8,
    demo_natural_shl_i8,
    demo_natural_shl_i8_ref,
    benchmark_natural_shl_i8_evaluation_strategy
);
demos_and_benches_signed!(
    i16,
    demo_natural_shl_assign_i16,
    demo_natural_shl_i16,
    demo_natural_shl_i16_ref,
    benchmark_natural_shl_i16_evaluation_strategy
);
demos_and_benches_signed!(
    i32,
    demo_natural_shl_assign_i32,
    demo_natural_shl_i32,
    demo_natural_shl_i32_ref,
    benchmark_natural_shl_i32_evaluation_strategy
);
demos_and_benches_signed!(
    i64,
    demo_natural_shl_assign_i64,
    demo_natural_shl_i64,
    demo_natural_shl_i64_ref,
    benchmark_natural_shl_i64_evaluation_strategy
);
demos_and_benches_signed!(
    isize,
    demo_natural_shl_assign_isize,
    demo_natural_shl_isize,
    demo_natural_shl_isize_ref,
    benchmark_natural_shl_isize_evaluation_strategy
);

fn benchmark_limbs_shl(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_shl(&[Limb], u64)",
        BenchmarkType::Single,
        pairs_of_unsigned_vec_and_small_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref limbs, bits)| limbs.len() + usize::exact_from(bits >> Limb::LOG_WIDTH)),
        "limbs.len() + bits / Limb::WIDTH",
        &mut [(
            "malachite",
            &mut (|(limbs, bits)| no_out!(limbs_shl(&limbs, bits))),
        )],
    );
}

fn benchmark_limbs_shl_to_out(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_shl_to_out(&mut [Limb], &[Limb], u64)",
        BenchmarkType::Single,
        triples_of_unsigned_vec_unsigned_vec_and_u64_var_5(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref in_limbs, _)| in_limbs.len()),
        "in_limbs.len()",
        &mut [(
            "malachite",
            &mut (|(mut out, in_limbs, bits)| no_out!(limbs_shl_to_out(&mut out, &in_limbs, bits))),
        )],
    );
}

fn benchmark_limbs_slice_shl_in_place(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_slice_shl_in_place(&mut [Limb], u64)",
        BenchmarkType::Single,
        pairs_of_unsigned_vec_and_u64_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref limbs, _)| limbs.len()),
        "limbs.len()",
        &mut [(
            "malachite",
            &mut (|(mut limbs, bits)| no_out!(limbs_slice_shl_in_place(&mut limbs, bits))),
        )],
    );
}

fn benchmark_limbs_vec_shl_in_place(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark(
        "limbs_vec_shl_in_place(&mut Vec<Limb>, u64)",
        BenchmarkType::Single,
        pairs_of_unsigned_vec_and_small_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref limbs, bits)| limbs.len() + usize::exact_from(bits >> Limb::LOG_WIDTH)),
        "limbs.len()",
        &mut [(
            "malachite",
            &mut (|(mut limbs, bits)| limbs_vec_shl_in_place(&mut limbs, bits)),
        )],
    );
}

fn benchmark_natural_shl_assign_u32_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural <<= u32",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_natural_and_small_unsigned::<u32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (_, other))| usize::exact_from(other)),
        "other",
        &mut [
            ("malachite", &mut (|(_, (mut x, y))| x <<= y)),
            ("rug", &mut (|((mut x, y), _)| x <<= y)),
        ],
    );
}

fn benchmark_natural_shl_u32_library_comparison(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark(
        "Natural << u32",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_natural_and_small_unsigned::<u32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (_, other))| usize::exact_from(other)),
        "other",
        &mut [
            ("malachite", &mut (|(_, (x, y))| no_out!(x << y))),
            ("rug", &mut (|((x, y), _)| no_out!(x << y))),
        ],
    );
}

fn benchmark_limbs_shl_with_complement_to_out_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "limbs_shl_with_complement_to_out(&mut [u32], &[u32], u32)",
        BenchmarkType::Algorithms,
        triples_of_unsigned_vec_unsigned_vec_and_u64_var_6(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref in_limbs, _)| in_limbs.len()),
        "in_limbs.len()",
        &mut [
            (
                "standard",
                &mut (|(mut out, in_limbs, bits)| {
                    no_out!(limbs_shl_with_complement_to_out(&mut out, &in_limbs, bits))
                }),
            ),
            (
                "limbs_shl_to_out and limbs_not_in_place",
                &mut (|(mut out, in_limbs, bits)| {
                    limbs_shl_to_out(&mut out, &in_limbs, bits);
                    limbs_not_in_place(&mut out);
                }),
            ),
        ],
    );
}

fn benchmark_natural_shl_assign_i32_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural <<= i32",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_natural_and_small_signed::<i32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (_, other))| usize::exact_from(other.unsigned_abs())),
        "|other|",
        &mut [
            ("malachite", &mut (|(_, (mut x, y))| x <<= y)),
            ("rug", &mut (|((mut x, y), _)| x <<= y)),
        ],
    );
}

fn benchmark_natural_shl_i32_library_comparison(gm: GenerationMode, limit: usize, file_name: &str) {
    run_benchmark(
        "Natural << i32",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_natural_and_small_signed::<i32>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (_, other))| usize::exact_from(other.unsigned_abs())),
        "|other|",
        &mut [
            ("malachite", &mut (|(_, (x, y))| no_out!(x << y))),
            ("rug", &mut (|((x, y), _)| no_out!(x << y))),
        ],
    );
}
