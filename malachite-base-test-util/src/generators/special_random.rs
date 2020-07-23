use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::random::striped::{
    striped_random_natural_signeds, striped_random_positive_unsigneds, striped_random_signeds,
    striped_random_unsigneds,
};
use malachite_base::random::EXAMPLE_SEED;

use generators::common::{GenConfig, It};

// -- PrimitiveSigned --

pub fn special_random_signed_gen<T: PrimitiveSigned>(config: &GenConfig) -> It<T> {
    Box::new(striped_random_signeds(
        EXAMPLE_SEED,
        config.get_or("mean_run_length_n", T::WIDTH >> 1),
        config.get_or("mean_run_length_d", 1),
    ))
}

pub fn special_random_signed_gen_var_1<T: PrimitiveSigned>(config: &GenConfig) -> It<T> {
    Box::new(special_random_signed_gen(config).filter(|&x| x != T::MIN))
}

pub fn special_random_signed_gen_var_2<T: PrimitiveSigned>(config: &GenConfig) -> It<T> {
    Box::new(striped_random_natural_signeds(
        EXAMPLE_SEED,
        config.get_or("mean_run_length_n", T::WIDTH >> 1),
        config.get_or("mean_run_length_d", 1),
    ))
}

// -- PrimitiveUnsigned --

pub fn special_random_unsigned_gen<T: PrimitiveUnsigned>(config: &GenConfig) -> It<T> {
    Box::new(striped_random_unsigneds(
        EXAMPLE_SEED,
        config.get_or("mean_run_length_n", T::WIDTH >> 1),
        config.get_or("mean_run_length_d", 1),
    ))
}

pub fn special_random_unsigned_gen_var_1<T: PrimitiveUnsigned>(config: &GenConfig) -> It<T> {
    Box::new(striped_random_positive_unsigneds(
        EXAMPLE_SEED,
        config.get_or("mean_run_length_n", T::WIDTH >> 1),
        config.get_or("mean_run_length_d", 1),
    ))
}
