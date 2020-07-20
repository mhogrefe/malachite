use generators::common::{GenConfig, It};
use malachite_base::bools::random::random_bools;
use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::random::{random_positive_unsigneds, random_primitive_integers};
use malachite_base::random::EXAMPLE_SEED;

// -- bool --

pub fn random_bool_gen(_config: &GenConfig) -> It<bool> {
    Box::new(random_bools(EXAMPLE_SEED))
}

// -- PrimitiveInteger --

pub fn random_primitive_integer_gen<T: PrimitiveInteger>(_config: &GenConfig) -> It<T> {
    Box::new(random_primitive_integers(EXAMPLE_SEED))
}

// -- PrimitiveSigned --

pub fn random_signed_gen_var_1<T: PrimitiveSigned>(_config: &GenConfig) -> It<T> {
    Box::new(random_primitive_integers(EXAMPLE_SEED).filter(|&x| x != T::MIN))
}

// -- PrimitiveUnsigned --

pub fn random_unsigned_gen_var_1<T: PrimitiveUnsigned>(_config: &GenConfig) -> It<T> {
    Box::new(random_positive_unsigneds(EXAMPLE_SEED))
}
