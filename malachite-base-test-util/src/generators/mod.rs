use generators::common::Generator;
use generators::exhaustive::*;
use generators::random::*;
use generators::special_random::*;
use malachite_base::iterators::bit_distributor::BitDistributorOutputType;
use malachite_base::num::arithmetic::traits::UnsignedAbs;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{
    Digits, ExactFrom, SaturatingFrom, WrappingFrom, WrappingInto,
};
use malachite_base::num::logic::traits::{BitBlockAccess, LeadingZeros};
use malachite_base::rounding_modes::RoundingMode;
use malachite_base::slices::slice_trailing_zeros;
use malachite_base::tuples::exhaustive::{exhaustive_pairs_custom_output, ExhaustivePairs};

// general

#[inline]
pub fn exhaustive_pairs_big_tiny<
    X: Clone,
    I: Iterator<Item = X>,
    Y: Clone,
    J: Iterator<Item = Y>,
>(
    xs: I,
    ys: J,
) -> ExhaustivePairs<X, I, Y, J> {
    exhaustive_pairs_custom_output(
        xs,
        ys,
        BitDistributorOutputType::normal(1),
        BitDistributorOutputType::tiny(),
    )
}

#[inline]
pub fn exhaustive_pairs_big_small<
    X: Clone,
    I: Iterator<Item = X>,
    Y: Clone,
    J: Iterator<Item = Y>,
>(
    xs: I,
    ys: J,
) -> ExhaustivePairs<X, I, Y, J> {
    exhaustive_pairs_custom_output(
        xs,
        ys,
        BitDistributorOutputType::normal(2),
        BitDistributorOutputType::normal(1),
    )
}

fn digits_valid<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(log_base: u64, digits: &[U]) -> bool {
    let digits = &digits[..digits.len() - slice_trailing_zeros(digits)];
    if digits.is_empty() {
        return true;
    }
    let significant_bits = ((u64::wrapping_from(digits.len()) - 1) * log_base)
        + digits.last().unwrap().significant_bits();
    significant_bits <= T::WIDTH
}

fn unsigned_assign_bits_valid<T: PrimitiveUnsigned>(start: u64, end: u64, bits: T) -> bool {
    let bits_width = end - start;
    let bits = bits.mod_power_of_two(bits_width);
    bits == T::ZERO || LeadingZeros::leading_zeros(bits) >= start
}

fn signed_assign_bits_valid<
    T: PrimitiveSigned + UnsignedAbs<Output = U>,
    U: BitBlockAccess<Bits = U> + PrimitiveUnsigned,
>(
    x: T,
    start: u64,
    end: u64,
    bits: U,
) -> bool {
    if x >= T::ZERO {
        unsigned_assign_bits_valid(start, end, bits) && {
            let mut abs_self = x.unsigned_abs();
            abs_self.assign_bits(start, end, &bits);
            !abs_self.get_highest_bit()
        }
    } else {
        start <= end && {
            let width = T::WIDTH;
            let bits_width = end - start;
            let bits = bits.mod_power_of_two(bits_width);
            bits_width <= width
                && if start >= width - 1 {
                    bits == U::low_mask(bits_width)
                } else {
                    end < width || bits >> (width - 1 - start) == U::low_mask(end - width + 1)
                }
        }
    }
}

// -- bool --

pub fn bool_gen() -> Generator<bool> {
    Generator::new_no_special(&exhaustive_bool_gen, &random_bool_gen)
}

// -- char --

pub fn char_gen() -> Generator<char> {
    Generator::new(
        &exhaustive_char_gen,
        &random_char_gen,
        &special_random_char_gen,
    )
}

// All `char`s except for `char::MAX`.
pub fn char_gen_var_1() -> Generator<char> {
    Generator::new(
        &exhaustive_char_gen_var_1,
        &random_char_gen_var_1,
        &special_random_char_gen_var_1,
    )
}

// All `char`s except for `char::MIN`.
pub fn char_gen_var_2() -> Generator<char> {
    Generator::new(
        &exhaustive_char_gen_var_2,
        &random_char_gen_var_2,
        &special_random_char_gen_var_2,
    )
}

// -- (char, char) --

pub fn char_pair_gen() -> Generator<(char, char)> {
    Generator::new(
        &exhaustive_char_pair_gen,
        &random_char_pair_gen,
        &special_random_char_pair_gen,
    )
}

// -- PrimitiveSigned --

pub fn signed_gen<T: PrimitiveSigned>() -> Generator<T> {
    Generator::new(
        &exhaustive_signed_gen,
        &random_primitive_int_gen,
        &special_random_signed_gen,
    )
}

// All signed `T`s which are not `T::MIN`.
pub fn signed_gen_var_1<T: PrimitiveSigned>() -> Generator<T> {
    Generator::new(
        &exhaustive_signed_gen_var_1,
        &random_signed_gen_var_1,
        &special_random_signed_gen_var_1,
    )
}

// All signed natural (non-negative) `T`s.
pub fn signed_gen_var_2<T: PrimitiveSigned>() -> Generator<T> {
    Generator::new(
        &exhaustive_signed_gen_var_2,
        &random_signed_gen_var_2,
        &special_random_signed_gen_var_2,
    )
}

// All signed `T`s that are neither 0 nor -1.
pub fn signed_gen_var_3<T: PrimitiveSigned>() -> Generator<T> {
    Generator::new(
        &exhaustive_signed_gen_var_3,
        &random_signed_gen_var_3,
        &special_random_signed_gen_var_3,
    )
}

// All negative signed `T`s.
pub fn signed_gen_var_4<T: PrimitiveSigned>() -> Generator<T> {
    Generator::new(
        &exhaustive_signed_gen_var_4,
        &random_signed_gen_var_4,
        &special_random_signed_gen_var_4,
    )
}

// All small signed `T`s.
pub fn signed_gen_var_5<T: PrimitiveSigned>() -> Generator<T> {
    Generator::new_no_special(&exhaustive_signed_gen::<T>, &random_signed_gen_var_5::<T>)
}

// All nonzero signed `T`s.
pub fn signed_gen_var_6<T: PrimitiveSigned>() -> Generator<T> {
    Generator::new(
        &exhaustive_signed_gen_var_5,
        &random_signed_gen_var_6,
        &special_random_signed_gen_var_5,
    )
}

// -- (PrimitiveSigned, PrimitiveSigned) --

pub fn signed_pair_gen<T: PrimitiveSigned>() -> Generator<(T, T)> {
    Generator::new(
        &exhaustive_signed_pair_gen,
        &random_primitive_int_pair_gen,
        &special_random_signed_pair_gen,
    )
}

// All pairs of signeds where either both values are non-negative or both are negative.
pub fn signed_pair_gen_var_1<T: PrimitiveSigned>() -> Generator<(T, T)> {
    Generator::new(
        &exhaustive_signed_pair_gen_var_1,
        &random_signed_pair_gen_var_1,
        &special_random_signed_pair_gen_var_1,
    )
}

// All pairs of signeds where the second value is small.
pub fn signed_pair_gen_var_2<T: PrimitiveSigned, U: PrimitiveSigned>() -> Generator<(T, U)> {
    Generator::new(
        &exhaustive_signed_pair_gen_var_3,
        &random_primitive_int_signed_pair_gen_var_1,
        &special_random_signed_pair_gen_var_2,
    )
}

// All pairs of signed `T` where the first is divisible by the second.
pub fn signed_pair_gen_var_3<T: PrimitiveSigned>() -> Generator<(T, T)> {
    Generator::new(
        &exhaustive_signed_pair_gen_var_4,
        &random_signed_pair_gen_var_2,
        &special_random_signed_pair_gen_var_3,
    )
}

// All pairs of signed `T` where the second `T` is nonzero and the pair is not `(T::MIN, -1)`.
pub fn signed_pair_gen_var_4<T: PrimitiveSigned>() -> Generator<(T, T)> {
    Generator::new(
        &exhaustive_signed_pair_gen_var_5,
        &random_signed_pair_gen_var_3,
        &special_random_signed_pair_gen_var_4,
    )
}

// All pairs of signed `T` where the second `T` is nonzero and the first is not divisible by the
// second.
pub fn signed_pair_gen_var_5<T: PrimitiveSigned>() -> Generator<(T, T)> {
    Generator::new(
        &exhaustive_signed_pair_gen_var_6,
        &random_signed_pair_gen_var_4,
        &special_random_signed_pair_gen_var_5,
    )
}

// -- (PrimitiveSigned, PrimitiveSigned, PrimitiveSigned) --

pub fn signed_triple_gen<T: PrimitiveSigned>() -> Generator<(T, T, T)> {
    Generator::new(
        &exhaustive_signed_triple_gen,
        &random_primitive_int_triple_gen,
        &special_random_signed_triple_gen,
    )
}

// All `(x, y, z): (T, T, T)` where `T` is signed and x + y * z does not overflow.
pub fn signed_triple_gen_var_1<T: PrimitiveSigned>() -> Generator<(T, T, T)> {
    Generator::new(
        &exhaustive_signed_triple_gen_var_1,
        &random_signed_triple_gen_var_1,
        &special_random_signed_triple_gen_var_1,
    )
}

// All `(x, y, z): (T, T, T)` where `T` is signed and x - y * z does not overflow.
pub fn signed_triple_gen_var_2<T: PrimitiveSigned>() -> Generator<(T, T, T)> {
    Generator::new(
        &exhaustive_signed_triple_gen_var_2,
        &random_signed_triple_gen_var_2,
        &special_random_signed_triple_gen_var_2,
    )
}

// All triple of signeds where either all values are non-negative or all are negative.
pub fn signed_triple_gen_var_3<T: PrimitiveSigned>() -> Generator<(T, T, T)> {
    Generator::new(
        &exhaustive_signed_triple_gen_var_3,
        &random_signed_triple_gen_var_3,
        &special_random_signed_triple_gen_var_3,
    )
}

// All triples of signeds (x, y, m) where x is equal to y mod m.
pub fn signed_triple_gen_var_4<
    U: PrimitiveUnsigned + WrappingInto<S> + WrappingFrom<S>,
    S: PrimitiveSigned + UnsignedAbs<Output = U> + WrappingFrom<U>,
>() -> Generator<(S, S, S)> {
    Generator::new(
        &exhaustive_signed_triple_gen_var_4,
        &random_signed_triple_gen_var_4::<U, S>,
        &special_random_signed_triple_gen_var_4::<U, S>,
    )
}

// All triples of signeds (x, y, m) where x is not equal to y mod m.
pub fn signed_triple_gen_var_5<T: PrimitiveSigned>() -> Generator<(T, T, T)> {
    Generator::new(
        &exhaustive_signed_triple_gen_var_5,
        &random_primitive_int_triple_gen_var_1,
        &special_random_signed_triple_gen_var_5,
    )
}

// -- (PrimitiveSigned, PrimitiveSigned, PrimitiveSigned, PrimitiveSigned) --

pub fn signed_quadruple_gen<T: PrimitiveSigned>() -> Generator<(T, T, T, T)> {
    Generator::new(
        &exhaustive_signed_quadruple_gen,
        &random_primitive_int_quadruple_gen,
        &special_random_signed_quadruple_gen,
    )
}

// -- (PrimitiveSigned, PrimitiveSigned, PrimitiveSigned, PrimitiveUnsigned) --

// All `(T, T, T, U)` where `T` is signed, `U` is unsigned, and the `U` is small.
pub fn signed_signed_signed_unsigned_quadruple_gen_var_1<
    T: PrimitiveSigned,
    U: PrimitiveUnsigned,
>() -> Generator<(T, T, T, U)> {
    Generator::new(
        &exhaustive_signed_signed_signed_unsigned_quadruple_gen_var_1,
        &random_primitive_int_primitive_int_primitive_int_unsigned_quadruple_gen_var_1,
        &special_random_signed_signed_signed_unsigned_quadruple_gen_var_2,
    )
}

// -- (PrimitiveSigned, PrimitiveSigned, PrimitiveUnsigned) --

// All triples `(T, T, u64)` (x, y, p) where `T` is signed and x is equal to y mod $2^p$.
pub fn signed_signed_unsigned_triple_gen_var_1<
    U: PrimitiveUnsigned + WrappingFrom<S>,
    S: PrimitiveSigned + WrappingFrom<U>,
>() -> Generator<(S, S, u64)> {
    Generator::new(
        &exhaustive_signed_signed_unsigned_triple_gen_var_1::<U, S>,
        &random_signed_signed_unsigned_triple_gen_var_1::<U, S>,
        &special_random_signed_signed_unsigned_triple_gen_var_1::<U, S>,
    )
}

// All `(T, T, U)` where `T` is signed, `U` is unsigned, and the `U` is small.
pub fn signed_signed_unsigned_triple_gen_var_2<T: PrimitiveSigned, U: PrimitiveUnsigned>(
) -> Generator<(T, T, U)> {
    Generator::new(
        &exhaustive_signed_unsigned_unsigned_triple_gen_var_4,
        &random_primitive_int_primitive_int_unsigned_triple_gen_var_2,
        &special_random_signed_signed_unsigned_triple_gen_var_2,
    )
}

// All triples `(T, T, u64)` (x, y, p) where `T` is unsigned and x is not equal to y mod $2^p$.
pub fn signed_signed_unsigned_triple_gen_var_3<T: PrimitiveSigned>() -> Generator<(T, T, u64)> {
    Generator::new(
        &exhaustive_signed_signed_unsigned_triple_gen_var_5,
        &random_primitive_int_primitive_int_unsigned_triple_gen_var_3,
        &special_random_signed_signed_unsigned_triple_gen_var_3,
    )
}

// -- (PrimitiveSigned, PrimitiveSigned, RoundingMode) --

pub fn signed_signed_rounding_mode_triple_gen_var_1<T: PrimitiveSigned>(
) -> Generator<(T, T, RoundingMode)> {
    Generator::new(
        &exhaustive_signed_signed_rounding_mode_triple_gen_var_1,
        &random_signed_signed_rounding_mode_triple_gen_var_1,
        &special_random_signed_signed_rounding_mode_triple_gen_var_1,
    )
}

// -- (PrimitiveSigned, PrimitiveUnsigned) --

// All `(T, U)`s where `T` is signed, `U` is unsigned, and the `U` is small.
pub fn signed_unsigned_pair_gen_var_1<T: PrimitiveSigned, U: PrimitiveUnsigned>(
) -> Generator<(T, U)> {
    Generator::new(
        &exhaustive_signed_unsigned_pair_gen_var_2,
        &random_primitive_int_unsigned_pair_gen_var_1,
        &special_random_signed_unsigned_pair_gen_var_1,
    )
}

// All `(T, u64)`s where `T` is signed and the `u64` is smaller than `T::WIDTH`.
pub fn signed_unsigned_pair_gen_var_2<T: PrimitiveSigned>() -> Generator<(T, u64)> {
    Generator::new(
        &exhaustive_signed_unsigned_pair_gen_var_4,
        &random_primitive_int_unsigned_pair_gen_var_2,
        &special_random_signed_unsigned_pair_gen_var_2,
    )
}

// All `(T, u64)`s where `T` is signed and the either the `T` is negative or the `u64` is less than
// `T::WIDTH`.
pub fn signed_unsigned_pair_gen_var_3<T: PrimitiveSigned>() -> Generator<(T, u64)> {
    Generator::new(
        &exhaustive_signed_unsigned_pair_gen_var_3,
        &random_signed_unsigned_pair_gen_var_1,
        &special_random_signed_unsigned_pair_gen_var_3,
    )
}

// All `(T, u64)`s where `T` is signed and the either the `T` is non-negative or the `u64` is less
// than `T::WIDTH`.
pub fn signed_unsigned_pair_gen_var_4<T: PrimitiveSigned>() -> Generator<(T, u64)> {
    Generator::new(
        &exhaustive_signed_unsigned_pair_gen_var_5,
        &random_signed_unsigned_pair_gen_var_2,
        &special_random_signed_unsigned_pair_gen_var_4,
    )
}

// All `(T, U)`s where `T` is signed, `U` is unsigned, and the `U` is greater than 1 and no greater
// than 36.
pub fn signed_unsigned_pair_gen_var_5<T: PrimitiveSigned, U: ExactFrom<u8> + PrimitiveUnsigned>(
) -> Generator<(T, U)> {
    Generator::new(
        &exhaustive_signed_unsigned_pair_gen_var_6,
        &random_primitive_int_unsigned_pair_gen_var_5,
        &special_random_signed_unsigned_pair_gen_var_5,
    )
}

// All `(T, U)`s where `T` is signed, `U` is unsigned, the `T` is non-negative, and the `U` is
// small.
pub fn signed_unsigned_pair_gen_var_6<T: PrimitiveSigned, U: PrimitiveUnsigned>(
) -> Generator<(T, U)> {
    Generator::new(
        &exhaustive_signed_unsigned_pair_gen_var_7,
        &random_signed_unsigned_pair_gen_var_3,
        &special_random_signed_unsigned_pair_gen_var_6,
    )
}

// All `(T, U)`s where `T` is signed, `U` is unsigned, the `T` is non-negative, and the `U` is
// greater than 1 and no greater than 36.
pub fn signed_unsigned_pair_gen_var_7<T: PrimitiveSigned, U: ExactFrom<u8> + PrimitiveUnsigned>(
) -> Generator<(T, U)> {
    Generator::new(
        &exhaustive_signed_unsigned_pair_gen_var_8,
        &random_signed_unsigned_pair_gen_var_4,
        &special_random_signed_unsigned_pair_gen_var_7,
    )
}

// All `(T, U)`s where `T` is signed, `U` is unsigned, the `U` is small, and the `T` is not
// divisible by 2 to the power of the `U`.
pub fn signed_unsigned_pair_gen_var_8<T: PrimitiveSigned, U: PrimitiveUnsigned>(
) -> Generator<(T, U)> {
    Generator::new(
        &exhaustive_signed_unsigned_pair_gen_var_9,
        &random_primitive_int_unsigned_pair_gen_var_6,
        &special_random_signed_unsigned_pair_gen_var_8,
    )
}

// All `(T, U)`s where `T` is signed, `U` is unsigned, the `U` is small, and the `T` is divisible by
// 2 to the power of the `U`.
pub fn signed_unsigned_pair_gen_var_9<T: PrimitiveSigned, U: PrimitiveUnsigned>(
) -> Generator<(T, u64)> {
    Generator::new(
        &exhaustive_signed_unsigned_pair_gen_var_10,
        &random_primitive_int_unsigned_pair_gen_var_7,
        &special_random_signed_unsigned_pair_gen_var_9,
    )
}

// -- (PrimitiveSigned, PrimitiveUnsigned, bool) --

// All (`T`, `u64`, and `bool`) where `T` is signed and either the `u64` is smaller than `T::WIDTH`
// or the `bool` is equal to whether the `T` is negative.
pub fn signed_unsigned_bool_triple_gen_var_1<T: PrimitiveSigned>() -> Generator<(T, u64, bool)> {
    Generator::new(
        &exhaustive_signed_unsigned_bool_triple_gen_var_1,
        &random_primitive_int_unsigned_bool_triple_gen_var_2,
        &random_signed_unsigned_bool_triple_gen_var_1,
    )
}

// -- (PrimitiveSigned, PrimitiveUnsigned, PrimitiveUnsigned) --

// All `(T, U, V)` where `T` is signed, `U` and `V` are unsigned, and the `V` is small.
pub fn signed_unsigned_unsigned_triple_gen_var_1<
    T: PrimitiveSigned,
    U: PrimitiveUnsigned,
    V: PrimitiveUnsigned,
>() -> Generator<(T, U, V)> {
    Generator::new(
        &exhaustive_signed_unsigned_unsigned_triple_gen_var_1,
        &random_primitive_int_primitive_int_unsigned_triple_gen_var_2,
        &special_random_signed_unsigned_unsigned_triple_gen_var_1,
    )
}

// All `(T, U, U)` where `T` is signed, `U` is unsigned, both `U`s are small, the first `U` is less
// than or equal to the second, and if the `T` is negative, the difference between the two `U`s is
// no greater than the width of `T`.
pub fn signed_unsigned_unsigned_triple_gen_var_2<T: PrimitiveSigned, U: PrimitiveUnsigned>(
) -> Generator<(T, U, U)> {
    Generator::new(
        &exhaustive_signed_unsigned_unsigned_triple_gen_var_2,
        &random_signed_unsigned_unsigned_triple_gen_var_1,
        &special_random_signed_unsigned_unsigned_triple_gen_var_2,
    )
}

// All `(T, U, V)`s where `T` is signed, `U` and `V` are unsigned, the `U` is greater than 1 and no
// greater than 36, and the `V` is small.
pub fn signed_unsigned_unsigned_triple_gen_var_3<
    T: PrimitiveSigned,
    U: ExactFrom<u8> + PrimitiveUnsigned,
    V: PrimitiveUnsigned,
>() -> Generator<(T, U, V)> {
    Generator::new(
        &exhaustive_signed_unsigned_unsigned_triple_gen_var_3,
        &random_primitive_int_unsigned_unsigned_triple_gen_var_3,
        &special_random_signed_unsigned_unsigned_triple_gen_var_3,
    )
}

// -- (PrimitiveSigned, PrimitiveUnsigned, PrimitiveUnsigned, PrimitiveUnsigned) --

// All `(T, u64, u64, U)` where `T` is signed, `U` is unsigned, both `u64`s are small, and the four
// values are valid arguments to `assign_bits`.
pub fn signed_unsigned_unsigned_unsigned_quadruple_gen_var_1<
    T: PrimitiveSigned + UnsignedAbs<Output = U>,
    U: BitBlockAccess<Bits = U> + PrimitiveUnsigned,
>() -> Generator<(T, u64, u64, U)> {
    Generator::new(
        &exhaustive_signed_unsigned_unsigned_unsigned_quadruple_gen_var_1,
        &random_signed_unsigned_unsigned_unsigned_quadruple_gen_var_1,
        &special_random_signed_unsigned_unsigned_unsigned_quadruple_gen_var_1,
    )
}

// -- (PrimitiveSigned, RoundingMode) --

pub fn signed_rounding_mode_pair_gen<T: PrimitiveSigned>() -> Generator<(T, RoundingMode)> {
    Generator::new(
        &exhaustive_signed_rounding_mode_pair_gen,
        &random_primitive_int_rounding_mode_pair_gen,
        &special_random_signed_rounding_mode_pair_gen,
    )
}

// All `(T, RoundingMode)`s where `T` is signed and the `T` is nonzero.
pub fn signed_rounding_mode_pair_gen_var_1<T: PrimitiveSigned>() -> Generator<(T, RoundingMode)> {
    Generator::new(
        &exhaustive_signed_rounding_mode_pair_gen_var_1,
        &random_signed_rounding_mode_pair_gen_var_1,
        &special_random_signed_rounding_mode_pair_gen_var_1,
    )
}

// All `(T, RoundingMode)`s where `T` is signed and the `T` is not `T::MIN`.
pub fn signed_rounding_mode_pair_gen_var_2<T: PrimitiveSigned>() -> Generator<(T, RoundingMode)> {
    Generator::new(
        &exhaustive_signed_rounding_mode_pair_gen_var_2,
        &random_signed_rounding_mode_pair_gen_var_2,
        &special_random_signed_rounding_mode_pair_gen_var_2,
    )
}

// All `(T, RoundingMode)`s where `T` is signed and the `T` is nonzero and not `T::MIN`.
pub fn signed_rounding_mode_pair_gen_var_3<T: PrimitiveSigned>() -> Generator<(T, RoundingMode)> {
    Generator::new(
        &exhaustive_signed_rounding_mode_pair_gen_var_3,
        &random_signed_rounding_mode_pair_gen_var_3,
        &special_random_signed_rounding_mode_pair_gen_var_3,
    )
}

// -- (PrimitiveSigned, Vec<bool>) --

// All `(T, Vec<bool>)` where `T` is signed and the `Vec` has as many elements as
// `u64::exact_from(n.to_bits_asc().len())` (which is not necessarily the same as the number of
// significant bits).
pub fn signed_bool_vec_pair_gen_var_1<T: PrimitiveSigned>() -> Generator<(T, Vec<bool>)> {
    Generator::new(
        &exhaustive_signed_bool_vec_pair_gen_var_1,
        &random_signed_bool_vec_pair_gen_var_1,
        &special_random_signed_bool_vec_pair_gen_var_1,
    )
}

// -- PrimitiveUnsigned --

pub fn unsigned_gen<T: PrimitiveUnsigned>() -> Generator<T> {
    Generator::new(
        &exhaustive_unsigned_gen,
        &random_primitive_int_gen,
        &special_random_unsigned_gen,
    )
}

// All unsigned positive `T`s.
pub fn unsigned_gen_var_1<T: PrimitiveUnsigned>() -> Generator<T> {
    Generator::new(
        &exhaustive_primitive_int_gen_var_1,
        &random_unsigned_gen_var_1,
        &special_random_unsigned_gen_var_1,
    )
}

// All `u32`s smaller than `NUMBER_OF_CHARS`.
pub fn unsigned_gen_var_2() -> Generator<u32> {
    Generator::new_no_special(&exhaustive_unsigned_gen_var_1, &random_unsigned_gen_var_2)
}

// All `u64`s between 1 and `T::WIDTH`, inclusive, where `T` is a primitive integer.
pub fn unsigned_gen_var_3<T: PrimitiveInt>() -> Generator<u64> {
    Generator::new_no_special(
        &exhaustive_unsigned_gen_var_2::<T>,
        &random_unsigned_gen_var_3::<T>,
    )
}

// All `U`s greater than 1 and no greater than `T::MAX`.
pub fn unsigned_gen_var_4<T: PrimitiveUnsigned, U: PrimitiveUnsigned + SaturatingFrom<T>>(
) -> Generator<U> {
    Generator::new_no_special(
        &exhaustive_unsigned_gen_var_4::<T, U>,
        &random_unsigned_gen_var_4::<T, U>,
    )
}

// All small unsigned `T`s.
pub fn unsigned_gen_var_5<T: PrimitiveUnsigned>() -> Generator<T> {
    Generator::new_no_special(
        &exhaustive_unsigned_gen::<T>,
        &random_unsigned_gen_var_5::<T>,
    )
}

// All unsigned `T`s greater than or equal to 2.
pub fn unsigned_gen_var_6<T: PrimitiveUnsigned>() -> Generator<T> {
    Generator::new_no_special(
        &exhaustive_primitive_int_gen_var_2::<T>,
        &random_unsigned_gen_var_6::<T>,
    )
}

// All unsigned `T`s less than 36.
pub fn unsigned_gen_var_7<T: PrimitiveUnsigned>() -> Generator<T> {
    Generator::new_no_special(
        &exhaustive_primitive_int_gen_var_3,
        &random_unsigned_gen_var_7,
    )
}

// All unsigned `T`s greater than or equal to 2 and less than or equal to 36.
pub fn unsigned_gen_var_8<T: PrimitiveUnsigned>() -> Generator<T> {
    Generator::new_no_special(
        &exhaustive_primitive_int_gen_var_4,
        &random_unsigned_gen_var_8,
    )
}

// All `u64`s between 0 and `T::WIDTH`, inclusive, where `T` is a primitive integer.
pub fn unsigned_gen_var_9<T: PrimitiveInt>() -> Generator<u64> {
    Generator::new_no_special(
        &exhaustive_unsigned_gen_var_5::<T>,
        &random_unsigned_gen_var_9::<T>,
    )
}

// -- (PrimitiveUnsigned, PrimitiveSigned) --

// All `(T, U)`s where `T` is unsigned, `U` is signed, and the `U` is small.
pub fn unsigned_signed_pair_gen_var_1<T: PrimitiveUnsigned, U: PrimitiveSigned>(
) -> Generator<(T, U)> {
    Generator::new(
        &exhaustive_unsigned_signed_pair_gen_var_3,
        &random_primitive_int_signed_pair_gen_var_1,
        &special_random_unsigned_signed_pair_gen_var_1,
    )
}

// -- (PrimitiveUnsigned, PrimitiveUnsigned) --

pub fn unsigned_pair_gen<T: PrimitiveUnsigned>() -> Generator<(T, T)> {
    Generator::new(
        &exhaustive_unsigned_pair_gen,
        &random_primitive_int_pair_gen,
        &special_random_unsigned_pair_gen,
    )
}

// All `(u32, u32)`s where each `u32` is smaller than `NUMBER_OF_CHARS`.
pub fn unsigned_pair_gen_var_1() -> Generator<(u32, u32)> {
    Generator::new_no_special(
        &exhaustive_unsigned_pair_gen_var_1,
        &random_unsigned_pair_gen_var_1,
    )
}

// All `(T, U)`s where `T` and `U` are unsigned and the `U` is small.
pub fn unsigned_pair_gen_var_2<T: PrimitiveUnsigned, U: PrimitiveUnsigned>() -> Generator<(T, U)> {
    Generator::new(
        &exhaustive_unsigned_pair_gen_var_2,
        &random_primitive_int_unsigned_pair_gen_var_1,
        &special_random_unsigned_pair_gen_var_1,
    )
}

// All `(T, u64)`s where `T` is unsigned and the `u64` is smaller than `T::WIDTH`.
pub fn unsigned_pair_gen_var_3<T: PrimitiveUnsigned>() -> Generator<(T, u64)> {
    Generator::new(
        &exhaustive_unsigned_pair_gen_var_3,
        &random_primitive_int_unsigned_pair_gen_var_2,
        &special_random_unsigned_pair_gen_var_2,
    )
}

// All `(T, u64)`s where `T` is unsigned and the `u64` is between 1 and `U::WIDTH`, inclusive.
pub fn unsigned_pair_gen_var_4<T: PrimitiveUnsigned, U: PrimitiveUnsigned>() -> Generator<(T, u64)>
{
    Generator::new(
        &exhaustive_unsigned_pair_gen_var_4::<T, U>,
        &random_primitive_int_unsigned_pair_gen_var_3::<T, U>,
        &special_random_unsigned_pair_gen_var_3::<T, U>,
    )
}

// All `(T, u64)`s where `T` is unsigned, the `T` is small, and the `u64` is between 1 and
// `U::WIDTH`, inclusive.
pub fn unsigned_pair_gen_var_5<T: PrimitiveUnsigned, U: PrimitiveInt>() -> Generator<(T, u64)> {
    Generator::new_no_special(
        &exhaustive_unsigned_pair_gen_var_4::<T, U>,
        &random_unsigned_pair_gen_var_2::<T, U>,
    )
}

// All `(T, U)`s where `T` and `U` are unsigned and the `U` is greater than 1 and no greater than
// `T::MAX`.
pub fn unsigned_pair_gen_var_6<T: PrimitiveUnsigned, U: PrimitiveUnsigned + SaturatingFrom<T>>(
) -> Generator<(T, U)> {
    Generator::new(
        &exhaustive_unsigned_pair_gen_var_5::<T, U>,
        &random_primitive_int_unsigned_pair_gen_var_4::<T, U>,
        &special_random_unsigned_pair_gen_var_4::<T, U>,
    )
}

// All `(T, T)` where `T` is unsigned and the first element is less than or equal to the second.
pub fn unsigned_pair_gen_var_7<T: PrimitiveUnsigned>() -> Generator<(T, T)> {
    Generator::new(
        &exhaustive_unsigned_pair_gen_var_6,
        &random_primitive_int_pair_gen_var_1,
        &special_random_unsigned_pair_gen_var_5,
    )
}

// All `(T, U)`s where `T` and `U` are unsigned and the `U` is greater than 1 and no greater than
// 36.
pub fn unsigned_pair_gen_var_8<T: PrimitiveUnsigned, U: ExactFrom<u8> + PrimitiveUnsigned>(
) -> Generator<(T, U)> {
    Generator::new(
        &exhaustive_unsigned_pair_gen_var_7,
        &random_primitive_int_unsigned_pair_gen_var_5,
        &special_random_unsigned_pair_gen_var_6,
    )
}

// All `(T, U)`s where `T` and `U` are unsigned, the `T` is small, and the `U` is greater than 1 and
// no greater than 36.
pub fn unsigned_pair_gen_var_9<T: PrimitiveUnsigned, U: ExactFrom<u8> + PrimitiveUnsigned>(
) -> Generator<(T, U)> {
    Generator::new_no_special(
        &exhaustive_unsigned_pair_gen_var_7,
        &random_unsigned_pair_gen_var_3,
    )
}

// All `(T, V)`s where `T` is unsigned, the `T` is between 2 and `max(T::MAX, U::MAX)`, inclusive,
// and the `V` is small.
pub fn unsigned_pair_gen_var_10<
    T: PrimitiveUnsigned + SaturatingFrom<U>,
    U: PrimitiveUnsigned,
    V: PrimitiveUnsigned,
>() -> Generator<(T, V)> {
    Generator::new_no_special(
        &exhaustive_unsigned_pair_gen_var_8::<T, U, V>,
        &random_unsigned_pair_gen_var_4::<T, U, V>,
    )
}

// All pairs of unsigned `T` where the first is divisible by the second.
pub fn unsigned_pair_gen_var_11<T: PrimitiveUnsigned>() -> Generator<(T, T)> {
    Generator::new(
        &exhaustive_unsigned_pair_gen_var_9,
        &random_unsigned_pair_gen_var_5,
        &special_random_unsigned_pair_gen_var_7,
    )
}

// All pairs of unsigned `T` where the second `T` is positive.
pub fn unsigned_pair_gen_var_12<T: PrimitiveUnsigned>() -> Generator<(T, T)> {
    Generator::new(
        &exhaustive_unsigned_pair_gen_var_10,
        &random_unsigned_pair_gen_var_6,
        &special_random_unsigned_pair_gen_var_8,
    )
}

// All pairs of unsigned `T` where the second `T` is positive and the first is not divisible by the
// second.
pub fn unsigned_pair_gen_var_13<T: PrimitiveUnsigned>() -> Generator<(T, T)> {
    Generator::new(
        &exhaustive_unsigned_pair_gen_var_11,
        &random_unsigned_pair_gen_var_7,
        &special_random_unsigned_pair_gen_var_9,
    )
}

// All `(T, U)`s where `T` and `U` are unsigned, the `U` is small, and the `T` is not divisible by 2
// to the power of the `U`.
pub fn unsigned_pair_gen_var_14<T: PrimitiveUnsigned, U: PrimitiveUnsigned>() -> Generator<(T, U)> {
    Generator::new(
        &exhaustive_unsigned_pair_gen_var_12,
        &random_primitive_int_unsigned_pair_gen_var_6,
        &special_random_unsigned_pair_gen_var_10,
    )
}

// All `(T, U)`s where `T` and `U` are unsigned, the `U` is small, and the `T` not divisible by 2 to
// the power of the `U`.
pub fn unsigned_pair_gen_var_15<T: PrimitiveUnsigned>() -> Generator<(T, u64)> {
    Generator::new(
        &exhaustive_unsigned_pair_gen_var_13,
        &random_primitive_int_unsigned_pair_gen_var_7,
        &special_random_unsigned_pair_gen_var_11,
    )
}

// -- (PrimitiveUnsigned, PrimitiveUnsigned, bool) --

// All (`T`, `u64`, `bool`) where `T` is unsigned and either the `bool` is false or the `u64` is
// smaller than `T::WIDTH`.
pub fn unsigned_unsigned_bool_triple_gen_var_1<T: PrimitiveUnsigned>() -> Generator<(T, u64, bool)>
{
    Generator::new(
        &exhaustive_unsigned_unsigned_bool_triple_gen_var_1,
        &random_primitive_int_unsigned_bool_triple_gen_var_1,
        &special_random_unsigned_unsigned_bool_triple_gen_var_1,
    )
}

// -- (PrimitiveUnsigned, PrimitiveUnsigned, PrimitiveUnsigned) --

pub fn unsigned_triple_gen<T: PrimitiveUnsigned>() -> Generator<(T, T, T)> {
    Generator::new(
        &exhaustive_unsigned_triple_gen,
        &random_primitive_int_triple_gen,
        &special_random_unsigned_triple_gen,
    )
}

// All `(x, y, z): (T, T, T)` where `T` is unsigned and x + y * z does not overflow.
pub fn unsigned_triple_gen_var_1<T: PrimitiveUnsigned>() -> Generator<(T, T, T)> {
    Generator::new(
        &exhaustive_unsigned_triple_gen_var_1,
        &random_unsigned_triple_gen_var_1,
        &special_random_unsigned_triple_gen_var_1,
    )
}

// All `(x, y, z): (T, T, T)` where `T` is unsigned and x - y * z does not overflow.
pub fn unsigned_triple_gen_var_2<T: PrimitiveUnsigned>() -> Generator<(T, T, T)> {
    Generator::new(
        &exhaustive_unsigned_triple_gen_var_2,
        &random_unsigned_triple_gen_var_2,
        &special_random_unsigned_triple_gen_var_2,
    )
}

// All `(T, u64, V)` where `T` is unsigned, the `u64` is between 1 and `U::WIDTH`, inclusive, and
// `V` is unsigned and the `V` is small.
pub fn unsigned_triple_gen_var_3<T: PrimitiveUnsigned, U: PrimitiveInt, V: PrimitiveUnsigned>(
) -> Generator<(T, u64, V)> {
    Generator::new(
        &exhaustive_unsigned_primitive_int_unsigned_triple_gen_var_3::<T, U, V>,
        &random_primitive_int_unsigned_unsigned_triple_gen_var_1::<T, U, V>,
        &special_random_unsigned_triple_gen_var_3::<T, U, V>,
    )
}

// All `(T, T, U)` where `T` and `U` are unsigned and the `U` is small.
pub fn unsigned_triple_gen_var_4<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
) -> Generator<(T, T, U)> {
    Generator::new(
        &exhaustive_unsigned_triple_gen_var_3,
        &random_primitive_int_primitive_int_unsigned_triple_gen_var_1,
        &special_random_unsigned_triple_gen_var_4,
    )
}

// All `(T, U, U)` where `T` and `U` are unsigned, both `U`s are small, and the first `U` is less
// than or equal to the second.
pub fn unsigned_triple_gen_var_5<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
) -> Generator<(T, U, U)> {
    Generator::new(
        &exhaustive_unsigned_triple_gen_var_4,
        &random_primitive_int_unsigned_unsigned_triple_gen_var_2,
        &special_random_unsigned_triple_gen_var_5,
    )
}

// All `(T, U, V)`s where `T`, `U`, and `V` are unsigned, the `U` is greater than 1 and no greater
// than 36, and the `V` is small.
pub fn unsigned_triple_gen_var_6<
    T: PrimitiveUnsigned,
    U: ExactFrom<u8> + PrimitiveUnsigned,
    V: PrimitiveUnsigned,
>() -> Generator<(T, U, V)> {
    Generator::new(
        &exhaustive_unsigned_triple_gen_var_5,
        &random_primitive_int_unsigned_unsigned_triple_gen_var_3,
        &special_random_unsigned_triple_gen_var_6,
    )
}

// All triples of unsigneds (x, y, m) where x is equal to y mod m.
pub fn unsigned_triple_gen_var_7<T: PrimitiveUnsigned>() -> Generator<(T, T, T)> {
    Generator::new(
        &exhaustive_unsigned_triple_gen_var_6,
        &random_unsigned_triple_gen_var_3,
        &special_random_unsigned_triple_gen_var_7,
    )
}

// All triples of signeds (x, y, m) where x is not equal to y mod m.
pub fn unsigned_triple_gen_var_8<T: PrimitiveUnsigned>() -> Generator<(T, T, T)> {
    Generator::new(
        &exhaustive_unsigned_triple_gen_var_7,
        &random_primitive_int_triple_gen_var_1,
        &special_random_unsigned_triple_gen_var_8,
    )
}

// All triples `(T, T, u64)` (x, y, p) where `T` is unsigned and x is equal to y mod $2^p$.
pub fn unsigned_triple_gen_var_9<T: PrimitiveUnsigned>() -> Generator<(T, T, u64)> {
    Generator::new(
        &exhaustive_unsigned_triple_gen_var_8,
        &random_unsigned_triple_gen_var_4,
        &special_random_unsigned_triple_gen_var_9,
    )
}

// All triples `(T, T, u64)` (x, y, p) where `T` is unsigned and x is not equal to y mod $2^p$.
pub fn unsigned_triple_gen_var_10<T: PrimitiveUnsigned>() -> Generator<(T, T, u64)> {
    Generator::new(
        &exhaustive_unsigned_triple_gen_var_9,
        &random_primitive_int_primitive_int_unsigned_triple_gen_var_3,
        &special_random_unsigned_triple_gen_var_10,
    )
}

// -- (PrimitiveUnsigned, PrimitiveUnsigned, PrimitiveUnsigned, PrimitiveUnsigned) --

pub fn unsigned_quadruple_gen<T: PrimitiveUnsigned>() -> Generator<(T, T, T, T)> {
    Generator::new(
        &exhaustive_unsigned_quadruple_gen,
        &random_primitive_int_quadruple_gen,
        &special_random_unsigned_quadruple_gen,
    )
}

// All `(T, u64, u64, U)` where `T` and `U` are unsigned, both `u64`s are small, and the four values
// are valid arguments to `assign_bits`.
pub fn unsigned_quadruple_gen_var_1<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
) -> Generator<(T, u64, u64, U)> {
    Generator::new(
        &exhaustive_unsigned_quadruple_gen_var_1,
        &random_primitive_int_unsigned_unsigned_unsigned_quadruple_gen_var_1,
        &special_random_unsigned_quadruple_gen_var_1,
    )
}

// All `(T, T, T, U)` where `T` and `U` are unsigned and the `U` is small.
pub fn unsigned_quadruple_gen_var_2<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
) -> Generator<(T, T, T, U)> {
    Generator::new(
        &exhaustive_unsigned_quadruple_gen_var_2,
        &random_primitive_int_primitive_int_primitive_int_unsigned_quadruple_gen_var_1,
        &special_random_unsigned_quadruple_gen_var_2,
    )
}

// -- (PrimitiveUnsigned, PrimitiveUnsigned, RoundingMode) --

pub fn unsigned_unsigned_rounding_mode_triple_gen_var_1<T: PrimitiveUnsigned>(
) -> Generator<(T, T, RoundingMode)> {
    Generator::new(
        &exhaustive_unsigned_unsigned_rounding_mode_triple_gen_var_1,
        &random_unsigned_unsigned_rounding_mode_triple_gen_var_1,
        &special_random_unsigned_unsigned_rounding_mode_triple_gen_var_1,
    )
}

// -- (PrimitiveUnsigned, PrimitiveUnsigned, Vec<bool>) --

// All `(T, u64, Vec<bool>)` where `T` is unsigned, the `u64` is between 1 and `U::WIDTH`,
// inclusive, and the `Vec` has as many elements as the `T` has digits in base $2^\ell$, where
// $\ell$ is the `u64`.
pub fn unsigned_unsigned_bool_vec_triple_gen_var_1<T: PrimitiveUnsigned, U: PrimitiveInt>(
) -> Generator<(T, u64, Vec<bool>)> {
    Generator::new(
        &exhaustive_unsigned_unsigned_bool_vec_triple_gen_var_1::<T, U>,
        &random_primitive_int_unsigned_bool_vec_triple_gen_var_1::<T, U>,
        &special_random_unsigned_unsigned_bool_vec_triple_gen_var_1::<T, U>,
    )
}

// -- (PrimitiveUnsigned, RoundingMode) --

pub fn unsigned_rounding_mode_pair_gen<T: PrimitiveUnsigned>() -> Generator<(T, RoundingMode)> {
    Generator::new(
        &exhaustive_unsigned_rounding_mode_pair_gen,
        &random_primitive_int_rounding_mode_pair_gen,
        &special_random_unsigned_rounding_mode_pair_gen,
    )
}

// All `(T, RoundingMode)`s where `T` is unsigned and the `T` is positive.
pub fn unsigned_rounding_mode_pair_gen_var_1<T: PrimitiveUnsigned>() -> Generator<(T, RoundingMode)>
{
    Generator::new(
        &exhaustive_primitive_int_rounding_mode_pair_gen_var_1,
        &random_unsigned_rounding_mode_pair_gen_var_1,
        &special_random_unsigned_rounding_mode_pair_gen_var_1,
    )
}

// -- (PrimitiveUnsigned, Vec<bool>) --

// All `(T, Vec<bool>)` where `T` is unsigned and the `Vec` has as many elements as the `T` has
// significant bits.
pub fn unsigned_bool_vec_pair_gen_var_1<T: PrimitiveUnsigned>() -> Generator<(T, Vec<bool>)> {
    Generator::new(
        &exhaustive_unsigned_bool_vec_pair_gen_var_1,
        &random_unsigned_bool_vec_pair_gen_var_1,
        &special_random_unsigned_bool_vec_pair_gen_var_1,
    )
}

// -- (PrimitiveUnsigned, Vec<bPrimitiveUnsigned>) --

// All `(T, Vec<T>)` that are valid inputs to `U::from_digits_desc`, where the `Vec` is no longer
// than the number of digits of `U::MAX` in the base of the `T`.
pub fn unsigned_unsigned_vec_pair_gen_var_1<
    T: PrimitiveUnsigned + SaturatingFrom<U>,
    U: Digits<T> + PrimitiveUnsigned,
>() -> Generator<(T, Vec<T>)> {
    Generator::new_no_special(
        &exhaustive_unsigned_unsigned_vec_pair_gen_var_1::<T, U>,
        &random_unsigned_unsigned_vec_pair_gen_var_1::<T, U>,
    )
}

// All `(T, Vec<T>)` that are valid inputs to `U::from_digits_asc`, where the `Vec` is no longer
// than the number of digits of `U::MAX` in the base of the `T`.
pub fn unsigned_unsigned_vec_pair_gen_var_2<
    T: PrimitiveUnsigned + SaturatingFrom<U>,
    U: Digits<T> + PrimitiveUnsigned,
>() -> Generator<(T, Vec<T>)> {
    Generator::new_no_special(
        &exhaustive_unsigned_unsigned_vec_pair_gen_var_2::<T, U>,
        &random_unsigned_unsigned_vec_pair_gen_var_2::<T, U>,
    )
}

// -- RoundingMode --

pub fn rounding_mode_gen() -> Generator<RoundingMode> {
    Generator::new_no_special(&exhaustive_rounding_mode_gen, &random_rounding_mode_gen)
}

// -- (RoundingMode, RoundingMode) --

pub fn rounding_mode_pair_gen() -> Generator<(RoundingMode, RoundingMode)> {
    Generator::new_no_special(
        &exhaustive_rounding_mode_pair_gen,
        &random_rounding_mode_pair_gen,
    )
}

// -- (RoundingMode, RoundingMode, RoundingMode) --

pub fn rounding_mode_triple_gen() -> Generator<(RoundingMode, RoundingMode, RoundingMode)> {
    Generator::new_no_special(
        &exhaustive_rounding_mode_triple_gen,
        &random_rounding_mode_triple_gen,
    )
}

// -- String --

pub fn string_gen() -> Generator<String> {
    Generator::new(
        &exhaustive_string_gen,
        &random_string_gen,
        &special_random_string_gen,
    )
}

// All ASCII `String`s.
pub fn string_gen_var_1() -> Generator<String> {
    Generator::new(
        &exhaustive_string_gen_var_1,
        &random_string_gen_var_1,
        &special_random_string_gen_var_1,
    )
}

// All `String`s containing only characters that appear in the `String` representations of
// `RoundingMode`s.
pub fn string_gen_var_2() -> Generator<String> {
    Generator::new_no_special(&exhaustive_string_gen_var_2, &random_string_gen_var_2)
}

// -- (String, String) --

pub fn string_pair_gen() -> Generator<(String, String)> {
    Generator::new(
        &exhaustive_string_pair_gen,
        &random_string_pair_gen,
        &special_random_string_pair_gen,
    )
}

// All pairs of ASCII `String`s.
pub fn string_pair_gen_var_1() -> Generator<(String, String)> {
    Generator::new(
        &exhaustive_string_pair_gen_var_1,
        &random_string_pair_gen_var_1,
        &special_random_string_pair_gen_var_1,
    )
}

// -- Vec<bool> --

// All `Vec<bool>`s that could be the bits, in ascending order, of an unsigned value of type `T`.
// The `Vec`s may be arbitrarily long.
pub fn bool_vec_gen_var_1<T: PrimitiveUnsigned>() -> Generator<Vec<bool>> {
    Generator::new(
        &exhaustive_bool_vec_gen_var_1::<T>,
        &random_bool_vec_gen_var_1::<T>,
        &special_random_bool_vec_gen_var_1::<T>,
    )
}

// All `Vec<bool>`s that could be the bits, in ascending order, of a signed value of type `T`. The
// `Vec`s may be arbitrarily long.
pub fn bool_vec_gen_var_2<T: PrimitiveSigned>() -> Generator<Vec<bool>> {
    Generator::new(
        &exhaustive_bool_vec_gen_var_2::<T>,
        &random_bool_vec_gen_var_2::<T>,
        &special_random_bool_vec_gen_var_2::<T>,
    )
}

// All `Vec<bool>`s that could be the bits, in descending order, of an unsigned value of type `T`.
// The `Vec`s may be arbitrarily long.
pub fn bool_vec_gen_var_3<T: PrimitiveUnsigned>() -> Generator<Vec<bool>> {
    Generator::new(
        &exhaustive_bool_vec_gen_var_3::<T>,
        &random_bool_vec_gen_var_3::<T>,
        &special_random_bool_vec_gen_var_3::<T>,
    )
}

// All `Vec<bool>`s that could be the bits, in descending order, of a signed value of type `T`. The
// `Vec`s may be arbitrarily long.
pub fn bool_vec_gen_var_4<T: PrimitiveSigned>() -> Generator<Vec<bool>> {
    Generator::new(
        &exhaustive_bool_vec_gen_var_4::<T>,
        &random_bool_vec_gen_var_4::<T>,
        &special_random_bool_vec_gen_var_4::<T>,
    )
}

// -- Vec<PrimitiveUnsigned> --

pub fn unsigned_vec_gen<T: PrimitiveUnsigned>() -> Generator<Vec<T>> {
    Generator::new(
        &exhaustive_unsigned_vec_gen,
        &random_primitive_int_vec_gen,
        &special_random_unsigned_vec_gen,
    )
}

// -- (Vec<PrimitiveUnsigned>, PrimitiveUnsigned) --

pub fn unsigned_vec_unsigned_pair_gen<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
) -> Generator<(Vec<T>, U)> {
    Generator::new(
        &exhaustive_unsigned_vec_unsigned_pair_gen,
        &random_primitive_int_vec_primitive_int_pair_gen,
        &special_random_unsigned_vec_unsigned_pair_gen,
    )
}

// All `(Vec<T>, usize)` where `T` is unsigned and the `usize` is less than or equal to the length
// of the `Vec`.
pub fn unsigned_vec_unsigned_pair_gen_var_1<T: PrimitiveUnsigned>() -> Generator<(Vec<T>, usize)> {
    Generator::new(
        &exhaustive_unsigned_vec_unsigned_pair_gen_var_1,
        &random_primitive_int_vec_unsigned_pair_gen_var_1,
        &special_random_unsigned_vec_unsigned_pair_gen_var_3,
    )
}

// All `(Vec<U>, u64)` such that the flipped `(u64, Vec<U>)` is a valid input to
// `from_power_of_two_digits_asc<T, U>`, where the `Vec` is no longer than the number of digits of
// `T::MAX` in the base 2 to the power of the `u64`.
pub fn unsigned_vec_unsigned_pair_gen_var_2<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
) -> Generator<(Vec<U>, u64)> {
    Generator::new(
        &exhaustive_unsigned_vec_unsigned_pair_gen_var_2::<T, U>,
        &random_unsigned_vec_unsigned_pair_gen_var_1::<T, U>,
        &special_random_unsigned_vec_unsigned_pair_gen_var_1::<T, U>,
    )
}

// All `(Vec<U>, u64)` such that the flipped `(u64, Vec<U>)` is a valid input to
// `from_power_of_two_digits_desc<T, U>`, where the `Vec` is no longer than the number of digits of
// `T::MAX` in the base 2 to the power of the `u64`.
pub fn unsigned_vec_unsigned_pair_gen_var_3<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
) -> Generator<(Vec<U>, u64)> {
    Generator::new(
        &exhaustive_unsigned_vec_unsigned_pair_gen_var_3::<T, U>,
        &random_unsigned_vec_unsigned_pair_gen_var_2::<T, U>,
        &special_random_unsigned_vec_unsigned_pair_gen_var_2::<T, U>,
    )
}

// var 4 is in malachite-nz

// All `(Vec<T>, U)` that are valid inputs to _from_digits_desc_basecase in malachite-nz.
pub fn unsigned_vec_unsigned_pair_gen_var_5<
    T: ExactFrom<U> + PrimitiveUnsigned + WrappingFrom<U>,
    U: PrimitiveUnsigned + SaturatingFrom<T>,
>() -> Generator<(Vec<T>, U)> {
    Generator::new_no_special(
        &exhaustive_unsigned_vec_unsigned_pair_gen_var_5::<T, U>,
        &random_unsigned_vec_unsigned_pair_gen_var_3::<T, U>,
    )
}

// -- (Vec<PrimitiveUnsigned>, PrimitiveUnsigned, PrimitiveUnsigned) --

type T1<T> = Generator<(Vec<T>, T, T)>;

pub fn unsigned_vec_unsigned_unsigned_triple_gen<T: PrimitiveUnsigned>() -> T1<T> {
    Generator::new(
        &exhaustive_unsigned_vec_unsigned_unsigned_triple_gen,
        &random_primitive_int_vec_primitive_int_primitive_int_triple_gen,
        &special_random_unsigned_vec_unsigned_unsigned_triple_gen,
    )
}

// All `(Vec<T>, U, V)` where `T`, `U`, and `V` are unsigned and the `U` is small.
pub fn unsigned_vec_unsigned_unsigned_triple_gen_var_1<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned,
    V: PrimitiveUnsigned,
>() -> Generator<(Vec<T>, U, V)> {
    Generator::new(
        &exhaustive_unsigned_vec_unsigned_unsigned_triple_gen_var_1,
        &random_primitive_int_vec_unsigned_primitive_int_triple_gen_var_1,
        &special_random_unsigned_vec_unsigned_unsigned_triple_gen_var_1,
    )
}

// All `(Vec<T>, usize, usize)` where `T` is unsigned and the length of the `Vec` is at least the
// product of the `usize`s.
pub fn unsigned_vec_unsigned_unsigned_triple_gen_var_2<T: PrimitiveUnsigned>(
) -> Generator<(Vec<T>, usize, usize)> {
    Generator::new(
        &exhaustive_unsigned_vec_unsigned_unsigned_triple_gen_var_2,
        &random_primitive_int_vec_unsigned_unsigned_triple_gen_var_1,
        &special_random_primitive_int_vec_unsigned_unsigned_triple_gen_var_1,
    )
}

// -- (Vec<PrimitiveUnsigned>, Vec<PrimitiveUnsigned>) --

// All `(Vec<T>, Vec<T>)` where `T` is unsigned, both `Vec`s are nonempty, and the first `Vec` is at
// least as large as the second,
pub fn unsigned_vec_pair_gen_var_1<T: PrimitiveUnsigned>() -> Generator<(Vec<T>, Vec<T>)> {
    Generator::new(
        &exhaustive_unsigned_vec_pair_gen_var_2,
        &random_primitive_int_vec_pair_gen_var_2,
        &special_random_unsigned_vec_pair_gen_var_2,
    )
}

// All `(Vec<T>, Vec<T>)` where `T` is unsigned and both `Vec`s are nonempty.
pub fn unsigned_vec_pair_gen_var_2<T: PrimitiveUnsigned>() -> Generator<(Vec<T>, Vec<T>)> {
    Generator::new(
        &exhaustive_unsigned_vec_pair_gen_var_3,
        &random_primitive_int_vec_pair_gen_var_3,
        &special_random_unsigned_vec_pair_gen_var_3,
    )
}

// -- (Vec<PrimitiveUnsigned>, Vec<PrimitiveUnsigned>, PrimitiveUnsigned) --

// All `(Vec<T>, Vec<T>, T)` where `T` is unsigned and the first `Vec` is at least as long as the
// second.
pub fn unsigned_vec_unsigned_vec_unsigned_triple_gen_var_1<T: PrimitiveUnsigned>(
) -> Generator<(Vec<T>, Vec<T>, T)> {
    Generator::new(
        &exhaustive_unsigned_vec_unsigned_vec_unsigned_triple_gen_var_1,
        &random_primitive_int_vec_primitive_int_vec_primitive_int_triple_gen_var_1,
        &special_random_unsigned_vec_unsigned_vec_unsigned_triple_gen_var_1,
    )
}

// var 2 is in malachite-nz

// -- (Vec<PrimitiveUnsigned>, Vec<PrimitiveUnsigned>, Vec<PrimitiveUnsigned>) --

// All `(Vec<T>, Vec<T>, Vec<T>)` where `T` is unsigned, no `Vec` is empty, the second and third
// `Vec`s have equal length, and the first is at least twice as long as the second.
pub fn unsigned_vec_triple_gen_var_1<T: PrimitiveUnsigned>() -> Generator<(Vec<T>, Vec<T>, Vec<T>)>
{
    Generator::new(
        &exhaustive_unsigned_vec_triple_gen_var_1,
        &random_primitive_int_vec_triple_gen_var_1,
        &special_random_unsigned_vec_triple_gen_var_1,
    )
}

// All `(Vec<T>, Vec<T>, Vec<T>)` where `T` is unsigned, no `Vec` is empty, the second is at least
// as long as the third, and the length of the first is at least the sum of the lengths of the
// second and the third.
pub fn unsigned_vec_triple_gen_var_2<T: PrimitiveUnsigned>() -> Generator<(Vec<T>, Vec<T>, Vec<T>)>
{
    Generator::new(
        &exhaustive_unsigned_vec_triple_gen_var_2,
        &random_primitive_int_vec_triple_gen_var_2,
        &special_random_unsigned_vec_triple_gen_var_2,
    )
}

// All `(Vec<T>, Vec<T>, Vec<T>)` where `T` is unsigned, no `Vec` is empty, and the length of the
// first is at least the sum of the lengths of the second and the third.
pub fn unsigned_vec_triple_gen_var_3<T: PrimitiveUnsigned>() -> Generator<(Vec<T>, Vec<T>, Vec<T>)>
{
    Generator::new(
        &exhaustive_unsigned_vec_triple_gen_var_3,
        &random_primitive_int_vec_triple_gen_var_3,
        &special_random_unsigned_vec_triple_gen_var_3,
    )
}

// vars 4 through 23 are in malachite-nz

// All `(Vec<T>, Vec<T>, Vec<T>)` where `T` is unsigned, no `Vec` is empty, the second and third
// `Vec`s have equal length, and the first is at least as long as the second.
pub fn unsigned_vec_triple_gen_var_24<T: PrimitiveUnsigned>() -> Generator<(Vec<T>, Vec<T>, Vec<T>)>
{
    Generator::new(
        &exhaustive_unsigned_vec_triple_gen_var_24,
        &random_primitive_int_vec_triple_gen_var_24,
        &special_random_unsigned_vec_triple_gen_var_24,
    )
}

// All `(Vec<T>, Vec<T>, Vec<T>)` where `T` is unsigned, all `Vec`s have length at least 2, the
// second and third `Vec`s have equal length, and the first is at least twice as long as the second.
pub fn unsigned_vec_triple_gen_var_25<T: PrimitiveUnsigned>() -> Generator<(Vec<T>, Vec<T>, Vec<T>)>
{
    Generator::new(
        &exhaustive_unsigned_vec_triple_gen_var_25,
        &random_primitive_int_vec_triple_gen_var_25,
        &special_random_unsigned_vec_triple_gen_var_25,
    )
}

// All `(Vec<T>, Vec<T>, Vec<T>)` where `T` is unsigned, all `Vec`s have length at least 2, the
// second and third `Vec`s have equal length, and the first is at least as long as the second.
pub fn unsigned_vec_triple_gen_var_26<T: PrimitiveUnsigned>() -> Generator<(Vec<T>, Vec<T>, Vec<T>)>
{
    Generator::new(
        &exhaustive_unsigned_vec_triple_gen_var_26,
        &random_primitive_int_vec_triple_gen_var_26,
        &special_random_unsigned_vec_triple_gen_var_26,
    )
}

// All `(Vec<T>, Vec<T>, Vec<T>)` where `T` is unsigned and all three `Vec`s have the same length,
// which is at least 2.
pub fn unsigned_vec_triple_gen_var_27<T: PrimitiveUnsigned>() -> Generator<(Vec<T>, Vec<T>, Vec<T>)>
{
    Generator::new(
        &exhaustive_unsigned_vec_triple_gen_var_27,
        &random_primitive_int_vec_triple_gen_var_27,
        &special_random_unsigned_vec_triple_gen_var_27,
    )
}

// -- large types --

pub fn large_type_gen_var_1<T: PrimitiveUnsigned>() -> Generator<(Vec<T>, Vec<T>, T, T)> {
    Generator::new(
        &exhaustive_large_type_gen_var_1,
        &random_large_type_gen_var_1,
        &special_random_large_type_gen_var_1,
    )
}

pub mod common;
pub mod exhaustive;
pub mod random;
pub mod special_random;
