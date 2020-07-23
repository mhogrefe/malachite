use malachite_base::bools::exhaustive::exhaustive_bools;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::exhaustive::{
    exhaustive_natural_signeds, exhaustive_positive_primitives, exhaustive_signeds,
    exhaustive_unsigneds,
};

use generators::common::It;

// -- bool --

pub fn exhaustive_bool_gen() -> It<bool> {
    Box::new(exhaustive_bools())
}

// -- PrimitiveInteger --

pub fn exhaustive_primitive_integer_gen_var_1<T: PrimitiveUnsigned>() -> It<T> {
    Box::new(exhaustive_positive_primitives())
}

// -- PrimitiveSigned --

pub fn exhaustive_signed_gen<T: PrimitiveSigned>() -> It<T> {
    Box::new(exhaustive_signeds())
}

pub fn exhaustive_signed_gen_var_1<T: PrimitiveSigned>() -> It<T> {
    Box::new(exhaustive_signeds().filter(|&x| x != T::MIN))
}

pub fn exhaustive_signed_gen_var_2<T: PrimitiveSigned>() -> It<T> {
    Box::new(exhaustive_natural_signeds())
}

// -- PrimitiveUnsigned --

pub fn exhaustive_unsigned_gen<T: PrimitiveUnsigned>() -> It<T> {
    Box::new(exhaustive_unsigneds())
}
