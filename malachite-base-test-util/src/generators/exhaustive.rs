use generators::common::It;
use malachite_base::bools::exhaustive::exhaustive_bools;
use malachite_base::chars::exhaustive::{exhaustive_ascii_chars, exhaustive_chars};
use malachite_base::comparison::traits::{Max, Min};
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::exhaustive::{
    exhaustive_natural_signeds, exhaustive_positive_primitive_ints, exhaustive_signeds,
    exhaustive_unsigneds,
};
use malachite_base::rounding_modes::exhaustive::exhaustive_rounding_modes;
use malachite_base::rounding_modes::RoundingMode;

// -- bool --

pub fn exhaustive_bool_gen() -> It<bool> {
    Box::new(exhaustive_bools())
}

// -- char --

pub fn exhaustive_char_gen() -> It<char> {
    Box::new(exhaustive_chars())
}

pub fn exhaustive_char_gen_var_1() -> It<char> {
    Box::new(exhaustive_ascii_chars())
}

#[allow(unstable_name_collisions)]
pub fn exhaustive_char_gen_var_2() -> It<char> {
    Box::new(char::MIN..char::MAX)
}

#[allow(unstable_name_collisions)]
pub fn exhaustive_char_gen_var_3() -> It<char> {
    Box::new('\u{1}'..=char::MAX)
}

// -- PrimitiveInt --

pub fn exhaustive_primitive_int_gen_var_1<T: PrimitiveUnsigned>() -> It<T> {
    Box::new(exhaustive_positive_primitive_ints())
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

// -- RoundingMode --

pub fn exhaustive_rounding_mode_gen() -> It<RoundingMode> {
    Box::new(exhaustive_rounding_modes())
}
