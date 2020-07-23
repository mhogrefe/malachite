use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;

use generators::common::Generator;
use generators::exhaustive::{
    exhaustive_bool_gen, exhaustive_primitive_integer_gen_var_1, exhaustive_signed_gen,
    exhaustive_signed_gen_var_1, exhaustive_signed_gen_var_2, exhaustive_unsigned_gen,
};
use generators::random::{
    random_bool_gen, random_primitive_integer_gen, random_signed_gen_var_1,
    random_signed_gen_var_2, random_unsigned_gen_var_1,
};
use generators::special_random::{
    special_random_signed_gen, special_random_signed_gen_var_1, special_random_signed_gen_var_2,
    special_random_unsigned_gen, special_random_unsigned_gen_var_1,
};

// -- bool --

pub fn bool_gen() -> Generator<bool> {
    Generator::new_no_special(&exhaustive_bool_gen, &random_bool_gen)
}

// -- PrimitiveSigned --

pub fn signed_gen<T: PrimitiveSigned>() -> Generator<T> {
    Generator::new(
        &exhaustive_signed_gen,
        &random_primitive_integer_gen,
        &special_random_signed_gen,
    )
}

/// All `T`s where `T` is signed and the `T` is not `T::MIN`.
pub fn signed_gen_var_1<T: PrimitiveSigned>() -> Generator<T> {
    Generator::new(
        &exhaustive_signed_gen_var_1,
        &random_signed_gen_var_1,
        &special_random_signed_gen_var_1,
    )
}

/// All signed natural (non-negative) `T`s.
pub fn signed_gen_var_2<T: PrimitiveSigned>() -> Generator<T> {
    Generator::new(
        &exhaustive_signed_gen_var_2,
        &random_signed_gen_var_2,
        &special_random_signed_gen_var_2,
    )
}

// -- PrimitiveUnsigned --

pub fn unsigned_gen<T: PrimitiveUnsigned>() -> Generator<T> {
    Generator::new(
        &exhaustive_unsigned_gen,
        &random_primitive_integer_gen,
        &special_random_unsigned_gen,
    )
}

/// All `T` where `T` is unsigned and the `T` is positive.
pub fn unsigned_gen_var_1<T: PrimitiveUnsigned>() -> Generator<T> {
    Generator::new(
        &exhaustive_primitive_integer_gen_var_1,
        &random_unsigned_gen_var_1,
        &special_random_unsigned_gen_var_1,
    )
}

pub mod common;
pub mod exhaustive;
pub mod random;
pub mod special_random;
