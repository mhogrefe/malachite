use common::{gmp_natural_to_native, gmp_natural_to_num_biguint, gmp_natural_to_rugint_integer};
use malachite_base::round::RoundingMode;
use malachite_base::traits::{ShrRound, ShrRoundAssign};
use malachite_gmp::natural as gmp;
use malachite_native::natural as native;
use num;
use rugint;
use rust_wheels::benchmarks::{BenchmarkOptions2, BenchmarkOptions3, BenchmarkOptions4,
                              benchmark_2, benchmark_3, benchmark_4};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::integers_geometric::natural_u32s_geometric;
use rust_wheels::iterators::naturals::{exhaustive_naturals, random_naturals};
use rust_wheels::iterators::primitive_ints::exhaustive_u;
use rust_wheels::iterators::rounding_modes::{exhaustive_rounding_modes, random_rounding_modes};
use rust_wheels::iterators::tuples::{lex_pairs, log_pairs, random_pairs, random_triples};

pub fn demo_exhaustive_natural_shr_assign_u32(limit: usize) {
    for (mut n, u) in log_pairs(exhaustive_naturals(), exhaustive_u::<u32>()).take(limit) {
        let n_old = n.clone();
        n >>= u;
        println!("x := {}; x >>= {}; x = {}", n_old, u, n);
    }
}

pub fn demo_random_natural_shr_assign_u32(limit: usize) {
    for (mut n, u) in random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_naturals(seed, 32)),
        &(|seed| natural_u32s_geometric(seed, 32)),
    ).take(limit)
    {
        let n_old = n.clone();
        n >>= u;
        println!("x := {}; x >>= {}; x = {}", n_old, u, n);
    }
}

pub fn demo_exhaustive_natural_shr_u32(limit: usize) {
    for (n, u) in log_pairs(exhaustive_naturals(), exhaustive_u::<u32>()).take(limit) {
        let n_old = n.clone();
        println!("{} >> {} = {}", n_old, u, n >> u);
    }
}

pub fn demo_random_natural_shr_u32(limit: usize) {
    for (n, u) in random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_naturals(seed, 32)),
        &(|seed| natural_u32s_geometric(seed, 32)),
    ).take(limit)
    {
        let n_old = n.clone();
        println!("{} >> {} = {}", n_old, u, n >> u);
    }
}

pub fn demo_exhaustive_natural_shr_u32_ref(limit: usize) {
    for (n, u) in log_pairs(exhaustive_naturals(), exhaustive_u::<u32>()).take(limit) {
        println!("&{} >> {} = {}", n, u, &n >> u);
    }
}

pub fn demo_random_natural_shr_u32_ref(limit: usize) {
    for (n, u) in random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_naturals(seed, 32)),
        &(|seed| natural_u32s_geometric(seed, 32)),
    ).take(limit)
    {
        println!("&{} >> {} = {}", n, u, &n >> u);
    }
}

pub fn demo_exhaustive_natural_shr_round_assign_u32(limit: usize) {
    for ((mut n, u), rm) in lex_pairs(
        log_pairs(exhaustive_naturals(), exhaustive_u::<u32>()),
        exhaustive_rounding_modes(),
    ).filter(|&((ref n, u), rm)| {
        rm != RoundingMode::Exact || n.divisible_by_power_of_2(u.into())
    })
        .take(limit)
    {
        let n_old = n.clone();
        n.shr_round_assign(u, rm);
        println!(
            "x := {}; x.shr_round_assign({}, {:?}); x = {}",
            n_old,
            u,
            rm,
            n
        );
    }
}

pub fn demo_random_natural_shr_round_assign_u32(limit: usize) {
    for (mut n, u, rm) in random_triples(
        &EXAMPLE_SEED,
        &(|seed| random_naturals(seed, 32)),
        &(|seed| natural_u32s_geometric(seed, 32)),
        &(|seed| random_rounding_modes(seed)),
    ).filter(|&(ref n, u, rm)| {
        rm != RoundingMode::Exact || n.divisible_by_power_of_2(u.into())
    })
        .take(limit)
    {
        let n_old = n.clone();
        n.shr_round_assign(u, rm);
        println!(
            "x := {}; x.shr_round_assign({}, {:?}); x = {}",
            n_old,
            u,
            rm,
            n
        );
    }
}

pub fn demo_exhaustive_natural_shr_round_u32(limit: usize) {
    for ((n, u), rm) in lex_pairs(
        log_pairs(exhaustive_naturals(), exhaustive_u::<u32>()),
        exhaustive_rounding_modes(),
    ).filter(|&((ref n, u), rm)| {
        rm != RoundingMode::Exact || n.divisible_by_power_of_2(u.into())
    })
        .take(limit)
    {
        let n_old = n.clone();
        println!(
            "{}.shr_round({}, {:?}) = {}",
            n_old,
            u,
            rm,
            n.shr_round(u, rm)
        );
    }
}

pub fn demo_random_natural_shr_round_u32(limit: usize) {
    for (n, u, rm) in random_triples(
        &EXAMPLE_SEED,
        &(|seed| random_naturals(seed, 32)),
        &(|seed| natural_u32s_geometric(seed, 32)),
        &(|seed| random_rounding_modes(seed)),
    ).filter(|&(ref n, u, rm)| {
        rm != RoundingMode::Exact || n.divisible_by_power_of_2(u.into())
    })
        .take(limit)
    {
        let n_old = n.clone();
        println!(
            "{}.shr_round({}, {:?}) = {}",
            n_old,
            u,
            rm,
            n.shr_round(u, rm)
        );
    }
}

pub fn demo_exhaustive_natural_shr_round_u32_ref(limit: usize) {
    for ((n, u), rm) in lex_pairs(
        log_pairs(exhaustive_naturals(), exhaustive_u::<u32>()),
        exhaustive_rounding_modes(),
    ).filter(|&((ref n, u), rm)| {
        rm != RoundingMode::Exact || n.divisible_by_power_of_2(u.into())
    })
        .take(limit)
    {
        println!(
            "(&{}).shr_round({}, {:?}) = {}",
            n,
            u,
            rm,
            (&n).shr_round(u, rm)
        );
    }
}

pub fn demo_random_natural_shr_round_u32_ref(limit: usize) {
    for (n, u, rm) in random_triples(
        &EXAMPLE_SEED,
        &(|seed| random_naturals(seed, 32)),
        &(|seed| natural_u32s_geometric(seed, 32)),
        &(|seed| random_rounding_modes(seed)),
    ).filter(|&(ref n, u, rm)| {
        rm != RoundingMode::Exact || n.divisible_by_power_of_2(u.into())
    })
        .take(limit)
    {
        println!(
            "(&{}).shr_round({}, {:?}) = {}",
            n,
            u,
            rm,
            (&n).shr_round(u, rm)
        );
    }
}

pub fn benchmark_exhaustive_natural_shr_assign_u32(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Natural >>= u32");
    benchmark_3(BenchmarkOptions3 {
        xs: log_pairs(exhaustive_naturals(), exhaustive_u::<u32>()),
        function_f: &(|(mut n, u)| n >>= u),
        function_g: &(|(mut n, u): (native::Natural, u32)| n >>= u),
        function_h: &(|(mut n, u): (rugint::Integer, u32)| n >>= u),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, index)| (gmp_natural_to_native(n), index)),
        z_cons: &(|&(ref n, index)| (gmp_natural_to_rugint_integer(n), index)),
        x_param: &(|&(_, index)| index as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        h_name: "rugint",
        title: "Natural >>= u32",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_natural_shr_assign_u32(limit: usize, scale: u32, file_name: &str) {
    println!("benchmarking random Natural >>= u32");
    benchmark_3(BenchmarkOptions3 {
        xs: random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_naturals(seed, scale)),
            &(|seed| natural_u32s_geometric(seed, scale)),
        ),
        function_f: &(|(mut n, u)| n >>= u),
        function_g: &(|(mut n, u): (native::Natural, u32)| n >>= u),
        function_h: &(|(mut n, u): (rugint::Integer, u32)| n >>= u),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, index)| (gmp_natural_to_native(n), index)),
        z_cons: &(|&(ref n, index)| (gmp_natural_to_rugint_integer(n), index)),
        x_param: &(|&(_, index)| index as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        h_name: "rugint",
        title: "Natural >>= u32",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_exhaustive_natural_shr_u32(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Natural >> u32");
    benchmark_4(BenchmarkOptions4 {
        xs: log_pairs(exhaustive_naturals(), exhaustive_u::<u32>()),
        function_f: &(|(n, u)| n >> u),
        function_g: &(|(n, u): (native::Natural, u32)| n >> u),
        function_h: &(|(n, u): (num::BigUint, u32)| n >> u as usize),
        function_i: &(|(n, u): (rugint::Integer, u32)| n >> u),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, index)| (gmp_natural_to_native(n), index)),
        z_cons: &(|&(ref n, index)| (gmp_natural_to_num_biguint(n), index)),
        w_cons: &(|&(ref n, index)| (gmp_natural_to_rugint_integer(n), index)),
        x_param: &(|&(_, index)| index as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        h_name: "num",
        i_name: "rugint",
        title: "Natural >> u32",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_natural_shr_u32(limit: usize, scale: u32, file_name: &str) {
    println!("benchmarking random Natural >>= u32");
    benchmark_4(BenchmarkOptions4 {
        xs: random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_naturals(seed, scale)),
            &(|seed| natural_u32s_geometric(seed, scale)),
        ),
        function_f: &(|(n, u)| n >> u),
        function_g: &(|(n, u): (native::Natural, u32)| n >> u),
        function_h: &(|(n, u): (num::BigUint, u32)| n >> u as usize),
        function_i: &(|(n, u): (rugint::Integer, u32)| n >> u),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, index)| (gmp_natural_to_native(n), index)),
        z_cons: &(|&(ref n, index)| (gmp_natural_to_num_biguint(n), index)),
        w_cons: &(|&(ref n, index)| (gmp_natural_to_rugint_integer(n), index)),
        x_param: &(|&(_, index)| index as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        h_name: "num",
        i_name: "rugint",
        title: "Natural >> u32",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_exhaustive_natural_shr_u32_ref(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive &Natural >> u32");
    benchmark_3(BenchmarkOptions3 {
        xs: log_pairs(exhaustive_naturals(), exhaustive_u::<u32>()),
        function_f: &(|(n, u)| &n >> u),
        function_g: &(|(n, u): (native::Natural, u32)| &n >> u),
        function_h: &(|(n, u): (num::BigUint, u32)| &n >> u as usize),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, index)| (gmp_natural_to_native(n), index)),
        z_cons: &(|&(ref n, index)| (gmp_natural_to_num_biguint(n), index)),
        x_param: &(|&(_, index)| index as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        h_name: "num",
        title: "\\\\&Natural >> u32",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_natural_shr_u32_ref(limit: usize, scale: u32, file_name: &str) {
    println!("benchmarking random &Natural >>= u32");
    benchmark_3(BenchmarkOptions3 {
        xs: random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_naturals(seed, scale)),
            &(|seed| natural_u32s_geometric(seed, scale)),
        ),
        function_f: &(|(n, u)| &n >> u),
        function_g: &(|(n, u): (native::Natural, u32)| &n >> u),
        function_h: &(|(n, u): (num::BigUint, u32)| &n >> u as usize),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, index)| (gmp_natural_to_native(n), index)),
        z_cons: &(|&(ref n, index)| (gmp_natural_to_num_biguint(n), index)),
        x_param: &(|&(_, index)| index as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        h_name: "num",
        title: "\\\\&Natural >> u32",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_exhaustive_natural_shr_round_assign_u32(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Natural.shr_round_assign(u32, RoundingMode)");
    benchmark_2(BenchmarkOptions2 {
        xs: lex_pairs(
            log_pairs(exhaustive_naturals(), exhaustive_u::<u32>()),
            exhaustive_rounding_modes(),
        ).filter(|&((ref n, u), rm)| {
            rm != RoundingMode::Exact || n.divisible_by_power_of_2(u.into())
        }),
        function_f: &(|((mut n, u), rm): ((gmp::Natural, u32), RoundingMode)| {
                          n.shr_round_assign(u, rm)
                      }),
        function_g: &(|((mut n, u), rm): ((native::Natural, u32), RoundingMode)| {
                          n.shr_round_assign(u, rm)
                      }),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&((ref n, index), rm)| ((gmp_natural_to_native(n), index), rm)),
        x_param: &(|&((_, index), _)| index as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "Natural.shr\\\\_round\\\\_assign(u32, RoundingMode)",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_natural_shr_round_assign_u32(limit: usize, scale: u32, file_name: &str) {
    println!("benchmarking random Natural.shr_round_assign(u32, RoundingMode)");
    benchmark_2(BenchmarkOptions2 {
        xs: random_triples(
            &EXAMPLE_SEED,
            &(|seed| random_naturals(seed, scale)),
            &(|seed| natural_u32s_geometric(seed, scale)),
            &(|seed| random_rounding_modes(seed)),
        ).filter(|&(ref n, u, rm)| {
            rm != RoundingMode::Exact || n.divisible_by_power_of_2(u.into())
        }),
        function_f: &(|(mut n, u, rm): (gmp::Natural, u32, RoundingMode)| {
            n.shr_round_assign(u, rm)
        }),
        function_g: &(|(mut n, u, rm): (native::Natural, u32, RoundingMode)| {
                          n.shr_round_assign(u, rm)
                      }),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, index, rm)| (gmp_natural_to_native(n), index, rm)),
        x_param: &(|&(_, index, _)| index as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "Natural.shr\\\\_round\\\\_assign(u32, RoundingMode)",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_exhaustive_natural_shr_round_u32(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Natural.shr_round(u32, RoundingMode)");
    benchmark_2(BenchmarkOptions2 {
        xs: lex_pairs(
            log_pairs(exhaustive_naturals(), exhaustive_u::<u32>()),
            exhaustive_rounding_modes(),
        ).filter(|&((ref n, u), rm)| {
            rm != RoundingMode::Exact || n.divisible_by_power_of_2(u.into())
        }),
        function_f: &(|((n, u), rm): ((gmp::Natural, u32), RoundingMode)| n.shr_round(u, rm)),
        function_g: &(|((n, u), rm): ((native::Natural, u32), RoundingMode)| n.shr_round(u, rm)),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&((ref n, index), rm)| ((gmp_natural_to_native(n), index), rm)),
        x_param: &(|&((_, index), _)| index as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "Natural.shr\\\\_round(u32, RoundingMode)",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_natural_shr_round_u32(limit: usize, scale: u32, file_name: &str) {
    println!("benchmarking random Natural.shr_round(u32, RoundingMode)");
    benchmark_2(BenchmarkOptions2 {
        xs: random_triples(
            &EXAMPLE_SEED,
            &(|seed| random_naturals(seed, scale)),
            &(|seed| natural_u32s_geometric(seed, scale)),
            &(|seed| random_rounding_modes(seed)),
        ).filter(|&(ref n, u, rm)| {
            rm != RoundingMode::Exact || n.divisible_by_power_of_2(u.into())
        }),
        function_f: &(|(n, u, rm): (gmp::Natural, u32, RoundingMode)| n.shr_round(u, rm)),
        function_g: &(|(n, u, rm): (native::Natural, u32, RoundingMode)| n.shr_round(u, rm)),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, index, rm)| (gmp_natural_to_native(n), index, rm)),
        x_param: &(|&(_, index, _)| index as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "Natural.shr\\\\_round(u32, RoundingMode)",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_exhaustive_natural_shr_round_u32_ref(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive (&Natural).shr_round(u32, RoundingMode)");
    benchmark_2(BenchmarkOptions2 {
        xs: lex_pairs(
            log_pairs(exhaustive_naturals(), exhaustive_u::<u32>()),
            exhaustive_rounding_modes(),
        ).filter(|&((ref n, u), rm)| {
            rm != RoundingMode::Exact || n.divisible_by_power_of_2(u.into())
        }),
        function_f: &(|((n, u), rm): ((gmp::Natural, u32), RoundingMode)| (&n).shr_round(u, rm)),
        function_g: &(|((n, u), rm): ((native::Natural, u32), RoundingMode)| (&n).shr_round(u, rm)),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&((ref n, index), rm)| ((gmp_natural_to_native(n), index), rm)),
        x_param: &(|&((_, index), _)| index as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "(\\\\&Natural).shr\\\\_round(u32, RoundingMode)",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_natural_shr_round_u32_ref(limit: usize, scale: u32, file_name: &str) {
    println!("benchmarking random (&Natural).shr_round(u32, RoundingMode)");
    benchmark_2(BenchmarkOptions2 {
        xs: random_triples(
            &EXAMPLE_SEED,
            &(|seed| random_naturals(seed, scale)),
            &(|seed| natural_u32s_geometric(seed, scale)),
            &(|seed| random_rounding_modes(seed)),
        ).filter(|&(ref n, u, rm)| {
            rm != RoundingMode::Exact || n.divisible_by_power_of_2(u.into())
        }),
        function_f: &(|(n, u, rm): (gmp::Natural, u32, RoundingMode)| (&n).shr_round(u, rm)),
        function_g: &(|(n, u, rm): (native::Natural, u32, RoundingMode)| (&n).shr_round(u, rm)),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, index, rm)| (gmp_natural_to_native(n), index, rm)),
        x_param: &(|&(_, index, _)| index as usize),
        limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "(\\\\&Natural).shr\\\\_round(u32, RoundingMode)",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
