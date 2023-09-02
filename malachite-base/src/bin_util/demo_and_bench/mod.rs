use malachite_base::test_util::runner::Runner;

macro_rules! unsigned_single_arg_demo {
    ($name: ident, $f: ident) => {
        fn $name<T: PrimitiveUnsigned>(gm: GenMode, config: &GenConfig, limit: usize) {
            for u in unsigned_gen::<T>().get(gm, config).take(limit) {
                println!(concat!("{}.", stringify!($f), "() = {}"), u, u.$f());
            }
        }
    };
}

macro_rules! signed_single_arg_demo {
    ($name: ident, $f: ident) => {
        fn $name<T: PrimitiveSigned>(gm: GenMode, config: &GenConfig, limit: usize) {
            for i in signed_gen::<T>().get(gm, config).take(limit) {
                println!(concat!("({}).", stringify!($f), "() = {}"), i, i.$f());
            }
        }
    };
}

macro_rules! unsigned_single_arg_bench {
    ($name: ident, $f: ident) => {
        fn $name<T: PrimitiveUnsigned>(
            gm: GenMode,
            config: &GenConfig,
            limit: usize,
            file_name: &str,
        ) {
            run_benchmark(
                &format!(concat!("{}.", stringify!($f), "()"), T::NAME),
                BenchmarkType::Single,
                unsigned_gen::<T>().get(gm, config),
                gm.name(),
                limit,
                file_name,
                &unsigned_bit_bucketer(),
                &mut [("Malachite", &mut |u| no_out!(u.$f()))],
            );
        }
    };
}

macro_rules! signed_single_arg_bench {
    ($name: ident, $f: ident) => {
        fn $name<T: PrimitiveSigned>(
            gm: GenMode,
            config: &GenConfig,
            limit: usize,
            file_name: &str,
        ) {
            run_benchmark(
                &format!(concat!("{}.", stringify!($f), "()"), T::NAME),
                BenchmarkType::Single,
                signed_gen::<T>().get(gm, config),
                gm.name(),
                limit,
                file_name,
                &signed_bit_bucketer(),
                &mut [("Malachite", &mut |i| no_out!(i.$f()))],
            );
        }
    };
}

pub(crate) fn register(runner: &mut Runner) {
    bools::register(runner);
    chars::register(runner);
    comparison::register(runner);
    num::register(runner);
    rational_sequences::register(runner);
    rounding_modes::register(runner);
    slices::register(runner);
    strings::register(runner);
    vecs::register(runner);
}

mod bools;
mod chars;
mod comparison;
mod num;
mod rational_sequences;
mod rounding_modes;
mod slices;
mod strings;
mod vecs;
