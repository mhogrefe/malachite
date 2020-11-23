use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base_test_util::bench::bucketers::{
    pair_max_bit_bucketer, signed_bit_bucketer, triple_max_bit_bucketer, unsigned_bit_bucketer,
};
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::generators::{
    signed_gen, signed_pair_gen, signed_triple_gen, unsigned_gen, unsigned_pair_gen,
    unsigned_triple_gen,
};
use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_unsigned_max_1);
    register_unsigned_demos!(runner, demo_unsigned_max_2);
    register_unsigned_demos!(runner, demo_unsigned_max_3);
    register_unsigned_demos!(runner, demo_unsigned_min_1);
    register_unsigned_demos!(runner, demo_unsigned_min_2);
    register_unsigned_demos!(runner, demo_unsigned_min_3);
    register_signed_demos!(runner, demo_signed_max_1);
    register_signed_demos!(runner, demo_signed_max_2);
    register_signed_demos!(runner, demo_signed_max_3);
    register_signed_demos!(runner, demo_signed_min_1);
    register_signed_demos!(runner, demo_signed_min_2);
    register_signed_demos!(runner, demo_signed_min_3);
    register_unsigned_benches!(runner, benchmark_unsigned_max_1);
    register_unsigned_benches!(runner, benchmark_unsigned_max_2);
    register_unsigned_benches!(runner, benchmark_unsigned_max_3);
    register_unsigned_benches!(runner, benchmark_unsigned_min_1);
    register_unsigned_benches!(runner, benchmark_unsigned_min_2);
    register_unsigned_benches!(runner, benchmark_unsigned_min_3);
    register_signed_benches!(runner, benchmark_signed_max_1);
    register_signed_benches!(runner, benchmark_signed_max_2);
    register_signed_benches!(runner, benchmark_signed_max_3);
    register_signed_benches!(runner, benchmark_signed_min_1);
    register_signed_benches!(runner, benchmark_signed_min_2);
    register_signed_benches!(runner, benchmark_signed_min_3);
}

fn demo_unsigned_max_1<T: PrimitiveUnsigned>(gm: GenMode, config: GenConfig, limit: usize) {
    for x in unsigned_gen::<T>().get(gm, &config).take(limit) {
        println!("max!({}) = {}", x, max!(x));
    }
}

fn demo_unsigned_max_2<T: PrimitiveUnsigned>(gm: GenMode, config: GenConfig, limit: usize) {
    for (x, y) in unsigned_pair_gen::<T>().get(gm, &config).take(limit) {
        println!("max!({}, {}) = {}", x, y, max!(x, y));
    }
}

fn demo_unsigned_max_3<T: PrimitiveUnsigned>(gm: GenMode, config: GenConfig, limit: usize) {
    for (x, y, z) in unsigned_triple_gen::<T>().get(gm, &config).take(limit) {
        println!("max!({}, {}, {}) = {}", x, y, z, max!(x, y, z));
    }
}

fn demo_unsigned_min_1<T: PrimitiveUnsigned>(gm: GenMode, config: GenConfig, limit: usize) {
    for x in unsigned_gen::<T>().get(gm, &config).take(limit) {
        println!("min!({}) = {}", x, min!(x));
    }
}

fn demo_unsigned_min_2<T: PrimitiveUnsigned>(gm: GenMode, config: GenConfig, limit: usize) {
    for (x, y) in unsigned_pair_gen::<T>().get(gm, &config).take(limit) {
        println!("min!({}, {}) = {}", x, y, min!(x, y));
    }
}

fn demo_unsigned_min_3<T: PrimitiveUnsigned>(gm: GenMode, config: GenConfig, limit: usize) {
    for (x, y, z) in unsigned_triple_gen::<T>().get(gm, &config).take(limit) {
        println!("min!({}, {}, {}) = {}", x, y, z, min!(x, y, z));
    }
}

fn demo_signed_max_1<T: PrimitiveSigned>(gm: GenMode, config: GenConfig, limit: usize) {
    for x in signed_gen::<T>().get(gm, &config).take(limit) {
        println!("max!({}) = {}", x, max!(x));
    }
}

fn demo_signed_max_2<T: PrimitiveSigned>(gm: GenMode, config: GenConfig, limit: usize) {
    for (x, y) in signed_pair_gen::<T>().get(gm, &config).take(limit) {
        println!("max!({}, {}) = {}", x, y, max!(x, y));
    }
}

fn demo_signed_max_3<T: PrimitiveSigned>(gm: GenMode, config: GenConfig, limit: usize) {
    for (x, y, z) in signed_triple_gen::<T>().get(gm, &config).take(limit) {
        println!("max!({}, {}, {}) = {}", x, y, z, max!(x, y, z));
    }
}

fn demo_signed_min_1<T: PrimitiveSigned>(gm: GenMode, config: GenConfig, limit: usize) {
    for x in signed_gen::<T>().get(gm, &config).take(limit) {
        println!("min!({}) = {}", x, min!(x));
    }
}

fn demo_signed_min_2<T: PrimitiveSigned>(gm: GenMode, config: GenConfig, limit: usize) {
    for (x, y) in signed_pair_gen::<T>().get(gm, &config).take(limit) {
        println!("min!({}, {}) = {}", x, y, min!(x, y));
    }
}

fn demo_signed_min_3<T: PrimitiveSigned>(gm: GenMode, config: GenConfig, limit: usize) {
    for (x, y, z) in signed_triple_gen::<T>().get(gm, &config).take(limit) {
        println!("min!({}, {}, {}) = {}", x, y, z, min!(x, y, z));
    }
}

fn benchmark_unsigned_max_1<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("max!({})", T::NAME),
        BenchmarkType::Single,
        unsigned_gen::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &unsigned_bit_bucketer(),
        &mut [("Malachite", &mut (|x| no_out!(max!(x))))],
    );
}

fn benchmark_unsigned_max_2<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("max!({}, {})", T::NAME, T::NAME),
        BenchmarkType::Single,
        unsigned_pair_gen::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_max_bit_bucketer("x", "y"),
        &mut [("Malachite", &mut (|(x, y)| no_out!(max!(x, y))))],
    );
}

fn benchmark_unsigned_max_3<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("max!({}, {}, {})", T::NAME, T::NAME, T::NAME),
        BenchmarkType::Single,
        unsigned_triple_gen::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &triple_max_bit_bucketer("x", "y", "z"),
        &mut [("Malachite", &mut (|(x, y, z)| no_out!(max!(x, y, z))))],
    );
}

fn benchmark_unsigned_min_1<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("min!({})", T::NAME),
        BenchmarkType::Single,
        unsigned_gen::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &unsigned_bit_bucketer(),
        &mut [("Malachite", &mut (|x| no_out!(min!(x))))],
    );
}

fn benchmark_unsigned_min_2<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("min!({}, {})", T::NAME, T::NAME),
        BenchmarkType::Single,
        unsigned_pair_gen::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_max_bit_bucketer("x", "y"),
        &mut [("Malachite", &mut (|(x, y)| no_out!(min!(x, y))))],
    );
}

fn benchmark_unsigned_min_3<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("min!({}, {}, {})", T::NAME, T::NAME, T::NAME),
        BenchmarkType::Single,
        unsigned_triple_gen::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &triple_max_bit_bucketer("x", "y", "z"),
        &mut [("Malachite", &mut (|(x, y, z)| no_out!(min!(x, y, z))))],
    );
}

fn benchmark_signed_max_1<T: PrimitiveSigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("max!({})", T::NAME),
        BenchmarkType::Single,
        signed_gen::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &signed_bit_bucketer(),
        &mut [("Malachite", &mut (|x| no_out!(max!(x))))],
    );
}

fn benchmark_signed_max_2<T: PrimitiveSigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("max!({}, {})", T::NAME, T::NAME),
        BenchmarkType::Single,
        signed_pair_gen::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_max_bit_bucketer("x", "y"),
        &mut [("Malachite", &mut (|(x, y)| no_out!(max!(x, y))))],
    );
}

fn benchmark_signed_max_3<T: PrimitiveSigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("max!({}, {}, {})", T::NAME, T::NAME, T::NAME),
        BenchmarkType::Single,
        signed_triple_gen::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &triple_max_bit_bucketer("x", "y", "z"),
        &mut [("Malachite", &mut (|(x, y, z)| no_out!(max!(x, y, z))))],
    );
}

fn benchmark_signed_min_1<T: PrimitiveSigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("min!({})", T::NAME),
        BenchmarkType::Single,
        signed_gen::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &signed_bit_bucketer(),
        &mut [("Malachite", &mut (|x| no_out!(min!(x))))],
    );
}

fn benchmark_signed_min_2<T: PrimitiveSigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("min!({}, {})", T::NAME, T::NAME),
        BenchmarkType::Single,
        signed_pair_gen::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_max_bit_bucketer("x", "y"),
        &mut [("Malachite", &mut (|(x, y)| no_out!(min!(x, y))))],
    );
}

fn benchmark_signed_min_3<T: PrimitiveSigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("min!({}, {}, {})", T::NAME, T::NAME, T::NAME),
        BenchmarkType::Single,
        signed_triple_gen::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &triple_max_bit_bucketer("x", "y", "z"),
        &mut [("Malachite", &mut (|(x, y, z)| no_out!(min!(x, y, z))))],
    );
}
