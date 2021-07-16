use generators::common::{
    permute_1_3_2, permute_2_1, reshape_1_2_to_3, reshape_2_1_to_3, reshape_2_2_to_4,
    reshape_3_1_to_4, It,
};
use generators::{
    digits_valid, exhaustive_pairs_big_small, exhaustive_pairs_big_tiny, signed_assign_bits_valid,
    unsigned_assign_bits_valid,
};
use itertools::{repeat_n, Itertools};
use malachite_base::bools::exhaustive::{exhaustive_bools, ExhaustiveBools};
use malachite_base::chars::constants::NUMBER_OF_CHARS;
use malachite_base::chars::exhaustive::{exhaustive_ascii_chars, exhaustive_chars};
use malachite_base::comparison::traits::Min;
use malachite_base::iterators::bit_distributor::BitDistributorOutputType;
use malachite_base::num::arithmetic::traits::{
    ArithmeticCheckedShl, CheckedNeg, DivRound, PowerOf2, UnsignedAbs,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{
    CheckedFrom, ConvertibleFrom, Digits, ExactFrom, HasHalf, JoinHalves, SaturatingFrom,
    SplitInHalf, WrappingFrom,
};
use malachite_base::num::exhaustive::{
    exhaustive_finite_primitive_floats, exhaustive_natural_signeds, exhaustive_negative_signeds,
    exhaustive_nonzero_finite_primitive_floats, exhaustive_nonzero_signeds,
    exhaustive_positive_finite_primitive_floats, exhaustive_positive_primitive_ints,
    exhaustive_primitive_float_range, exhaustive_primitive_floats,
    exhaustive_signed_inclusive_range, exhaustive_signed_range, exhaustive_signeds,
    exhaustive_unsigneds, primitive_int_increasing_inclusive_range, primitive_int_increasing_range,
    PrimitiveIntIncreasingRange, PrimitiveIntUpDown,
};
use malachite_base::num::float::PrimitiveFloat;
use malachite_base::num::iterators::{bit_distributor_sequence, ruler_sequence};
use malachite_base::num::logic::traits::{BitBlockAccess, LeadingZeros};
use malachite_base::rounding_modes::exhaustive::exhaustive_rounding_modes;
use malachite_base::rounding_modes::RoundingMode;
use malachite_base::strings::exhaustive::{exhaustive_strings, exhaustive_strings_using_chars};
use malachite_base::strings::{strings_from_char_vecs, StringsFromCharVecs};
use malachite_base::tuples::exhaustive::{
    exhaustive_dependent_pairs, exhaustive_pairs, exhaustive_pairs_from_single,
    exhaustive_quadruples_from_single, exhaustive_quadruples_xxxy_custom_output,
    exhaustive_quadruples_xxyx, exhaustive_quadruples_xyyx,
    exhaustive_quadruples_xyyz_custom_output, exhaustive_triples, exhaustive_triples_custom_output,
    exhaustive_triples_from_single, exhaustive_triples_xxy, exhaustive_triples_xxy_custom_output,
    exhaustive_triples_xyx, exhaustive_triples_xyx_custom_output, exhaustive_triples_xyy,
    exhaustive_triples_xyy_custom_output, lex_pairs, lex_pairs_from_single,
    lex_triples_from_single, ExhaustiveDependentPairsYsGenerator, ExhaustivePairs,
    ExhaustivePairs1Input, ExhaustiveTriples, ExhaustiveTriples1Input, ExhaustiveTriplesXXY,
    ExhaustiveTriplesXYY,
};
use malachite_base::unions::exhaustive::lex_union3s;
use malachite_base::unions::Union3;
use malachite_base::vecs::exhaustive::{
    exhaustive_fixed_length_vecs_from_single, exhaustive_vecs,
    exhaustive_vecs_length_inclusive_range, exhaustive_vecs_min_length,
    lex_fixed_length_vecs_from_single, shortlex_vecs_length_inclusive_range,
    ExhaustiveFixedLengthVecs1Input, ExhaustiveVecs, LexFixedLengthVecsFromSingle, ShortlexVecs,
};
use num::arithmetic::mod_mul::limbs_invert_limb_naive;
use num::float::PRIMITIVE_FLOAT_CHARS;
use rounding_modes::ROUNDING_MODE_CHARS;
use std::cmp::{max, min};
use std::iter::{once, Chain, Once};
use std::marker::PhantomData;
use std::vec::IntoIter;

// general

fn add_mul_inputs_valid<T: PrimitiveInt>(x: T, y: T, z: T) -> bool {
    x.checked_add_mul(y, z).is_some()
}

fn sub_mul_inputs_valid<T: PrimitiveInt>(x: T, y: T, z: T) -> bool {
    x.checked_sub_mul(y, z).is_some()
}

// -- bool --

pub fn exhaustive_bool_gen() -> It<bool> {
    Box::new(exhaustive_bools())
}

// -- char --

pub fn exhaustive_char_gen() -> It<char> {
    Box::new(exhaustive_chars())
}

#[allow(unstable_name_collisions)]
pub fn exhaustive_char_gen_var_1() -> It<char> {
    Box::new(char::MIN..char::MAX)
}

#[allow(unstable_name_collisions)]
pub fn exhaustive_char_gen_var_2() -> It<char> {
    Box::new('\u{1}'..=char::MAX)
}

// -- (char, char) --

pub fn exhaustive_char_pair_gen() -> It<(char, char)> {
    Box::new(exhaustive_pairs_from_single(exhaustive_chars()))
}

// -- PrimitiveFloat --

pub fn exhaustive_primitive_float_gen<T: PrimitiveFloat>() -> It<T> {
    Box::new(exhaustive_primitive_floats())
}

pub fn exhaustive_primitive_float_gen_var_1<T: PrimitiveFloat>() -> It<T> {
    Box::new(exhaustive_primitive_float_range(
        T::NEGATIVE_ONE / T::TWO,
        T::POSITIVE_INFINITY,
    ))
}

struct ExhaustivePositiveNaturalFloats<T: PrimitiveFloat> {
    phantom: PhantomData<*const T>,
    done: bool,
    exponent: i64,
    limit: u64,
    mantissa: u64,
}

impl<T: PrimitiveFloat> Iterator for ExhaustivePositiveNaturalFloats<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.done {
            None
        } else {
            let f = T::from_integer_mantissa_and_exponent(self.mantissa, self.exponent).unwrap();
            if f == T::MAX_FINITE {
                self.done = true;
            } else {
                self.mantissa += 1;
                if self.mantissa == self.limit {
                    self.mantissa >>= 1;
                    self.exponent += 1;
                    self.limit = u64::power_of_2(T::MANTISSA_WIDTH + 1);
                }
            }
            Some(f)
        }
    }
}

fn exhaustive_positive_natural_floats<T: PrimitiveFloat>() -> ExhaustivePositiveNaturalFloats<T> {
    ExhaustivePositiveNaturalFloats {
        phantom: PhantomData,
        done: false,
        exponent: 0,
        limit: u64::power_of_2(T::MANTISSA_WIDTH + 1),
        mantissa: 1,
    }
}

pub fn exhaustive_primitive_float_gen_var_2<T: PrimitiveFloat>() -> It<T> {
    Box::new(once(T::ZERO).chain(exhaustive_positive_natural_floats()))
}

pub fn exhaustive_primitive_float_gen_var_3<T: PrimitiveFloat>() -> It<T> {
    Box::new(exhaustive_positive_finite_primitive_floats::<T>().filter(|f| !f.is_integer()))
}

pub fn exhaustive_primitive_float_gen_var_4<T: PrimitiveFloat>() -> It<T> {
    let limit =
        T::from_integer_mantissa_and_exponent(1, i64::wrapping_from(T::MANTISSA_WIDTH)).unwrap();
    Box::new(
        exhaustive_positive_natural_floats::<T>()
            .take_while(move |&f| f <= limit)
            .map(|f| f - T::ONE / T::TWO),
    )
}

pub fn exhaustive_primitive_float_gen_var_5<T: PrimitiveFloat>() -> It<T> {
    Box::new(
        lex_pairs(
            exhaustive_primitive_float_gen_var_2::<T>(),
            exhaustive_bools(),
        )
        .map(|(f, b)| if b { f } else { -f }),
    )
}

pub fn exhaustive_primitive_float_gen_var_6<T: PrimitiveFloat>() -> It<T> {
    Box::new(
        lex_pairs(
            exhaustive_primitive_float_gen_var_3::<T>(),
            exhaustive_bools(),
        )
        .map(|(f, b)| if b { f } else { -f }),
    )
}

pub fn exhaustive_primitive_float_gen_var_7<T: PrimitiveFloat>() -> It<T> {
    Box::new(
        lex_pairs(
            exhaustive_primitive_float_gen_var_4::<T>(),
            exhaustive_bools(),
        )
        .map(|(f, b)| if b { f } else { -f }),
    )
}

pub fn exhaustive_primitive_float_gen_var_8<T: PrimitiveFloat>() -> It<T> {
    Box::new(exhaustive_finite_primitive_floats())
}

pub fn exhaustive_primitive_float_gen_var_9<T: PrimitiveFloat>() -> It<T> {
    Box::new(
        exhaustive_primitive_floats::<T>().filter(|&f| !f.is_nan() && f != T::POSITIVE_INFINITY),
    )
}

pub fn exhaustive_primitive_float_gen_var_10<T: PrimitiveFloat>() -> It<T> {
    Box::new(
        exhaustive_primitive_floats::<T>().filter(|&f| !f.is_nan() && f != T::NEGATIVE_INFINITY),
    )
}

pub fn exhaustive_primitive_float_gen_var_11<T: PrimitiveFloat>() -> It<T> {
    Box::new(exhaustive_primitive_floats::<T>().filter(|&f| !f.is_nan()))
}

pub fn exhaustive_primitive_float_gen_var_12<T: PrimitiveFloat>() -> It<T> {
    Box::new(exhaustive_nonzero_finite_primitive_floats())
}

// -- (PrimitiveFloat, PrimitiveFloat) --

pub fn exhaustive_primitive_float_pair_gen<T: PrimitiveFloat>() -> It<(T, T)> {
    Box::new(exhaustive_pairs_from_single(exhaustive_primitive_floats()))
}

pub fn exhaustive_primitive_float_pair_gen_var_1<T: PrimitiveFloat>() -> It<(T, T)> {
    Box::new(exhaustive_pairs_from_single(
        exhaustive_primitive_floats::<T>().filter(|&f| !f.is_nan()),
    ))
}

// -- (PrimitiveFloat, PrimitiveFloat, PrimitiveFloat) --

pub fn exhaustive_primitive_float_triple_gen<T: PrimitiveFloat>() -> It<(T, T, T)> {
    Box::new(exhaustive_triples_from_single(exhaustive_primitive_floats()))
}

// -- (PrimitiveFloat, PrimitiveSigned) --

pub fn exhaustive_primitive_float_signed_pair_gen_var_1<T: PrimitiveFloat, U: PrimitiveSigned>(
) -> It<(T, U)> {
    Box::new(exhaustive_pairs_big_tiny(
        exhaustive_positive_finite_primitive_floats(),
        exhaustive_signeds(),
    ))
}

pub fn exhaustive_primitive_float_signed_pair_gen_var_2<T: PrimitiveFloat>() -> It<(T, i64)> {
    Box::new(
        exhaustive_pairs_big_tiny(
            exhaustive_primitive_float_range(T::ONE, T::TWO),
            exhaustive_signed_inclusive_range(T::MIN_EXPONENT, T::MAX_EXPONENT),
        )
        .filter(|&(m, e)| m.precision() <= T::max_precision_for_sci_exponent(e)),
    )
}

// -- (PrimitiveFloat, PrimitiveUnsigned) --

pub fn exhaustive_primitive_float_unsigned_pair_gen_var_1<
    T: PrimitiveFloat,
    U: PrimitiveUnsigned,
>() -> It<(T, U)> {
    Box::new(exhaustive_pairs_big_tiny(
        exhaustive_positive_finite_primitive_floats(),
        exhaustive_unsigneds(),
    ))
}

pub fn exhaustive_primitive_float_unsigned_pair_gen_var_2<T: PrimitiveFloat>() -> It<(T, u64)> {
    Box::new(exhaustive_pairs_big_tiny(
        exhaustive_primitive_float_range(T::ONE, T::TWO),
        exhaustive_unsigneds(),
    ))
}

// -- (PrimitiveFloat, PrimitiveUnsigned, RoundingMode) --

pub fn exhaustive_primitive_float_unsigned_rounding_mode_triple_gen_var_1<
    T: PrimitiveFloat,
    U: PrimitiveUnsigned,
>() -> It<(T, U, RoundingMode)> {
    reshape_2_1_to_3(Box::new(lex_pairs(
        exhaustive_pairs_big_tiny(
            exhaustive_positive_finite_primitive_floats(),
            exhaustive_unsigneds(),
        ),
        exhaustive_rounding_modes(),
    )))
}

pub fn exhaustive_primitive_float_unsigned_rounding_mode_triple_gen_var_2<T: PrimitiveFloat>(
) -> It<(T, u64, RoundingMode)> {
    reshape_2_1_to_3(Box::new(lex_pairs(
        exhaustive_pairs_big_tiny(
            exhaustive_primitive_float_range(T::ONE, T::TWO),
            exhaustive_unsigneds(),
        ),
        exhaustive_rounding_modes(),
    )))
}

// -- (PrimitiveFloat, RoundingMode) --

pub(crate) fn float_rounding_mode_filter_var_1<T: PrimitiveFloat>(p: &(T, RoundingMode)) -> bool {
    let &(f, rm) = p;
    match rm {
        RoundingMode::Floor | RoundingMode::Up => f >= T::ZERO,
        RoundingMode::Ceiling | RoundingMode::Down => f > T::NEGATIVE_ONE,
        RoundingMode::Nearest => f >= T::NEGATIVE_ONE / T::TWO,
        RoundingMode::Exact => f >= T::ZERO && f.is_integer(),
    }
}

pub fn exhaustive_primitive_float_rounding_mode_pair_gen_var_1<T: PrimitiveFloat>(
) -> It<(T, RoundingMode)> {
    Box::new(
        lex_pairs(
            exhaustive_finite_primitive_floats(),
            exhaustive_rounding_modes(),
        )
        .filter(float_rounding_mode_filter_var_1),
    )
}

pub fn exhaustive_primitive_float_rounding_mode_pair_gen_var_2<T: PrimitiveFloat>(
) -> It<(T, RoundingMode)> {
    Box::new(
        lex_pairs(
            exhaustive_finite_primitive_floats::<T>(),
            exhaustive_rounding_modes(),
        )
        .filter(|&(f, rm)| rm != RoundingMode::Exact || f.is_integer()),
    )
}

// -- PrimitiveInt --

pub fn exhaustive_primitive_int_gen_var_1<T: PrimitiveInt>() -> It<T> {
    Box::new(exhaustive_positive_primitive_ints())
}

pub fn exhaustive_primitive_int_gen_var_2<T: PrimitiveInt>() -> It<T> {
    Box::new(primitive_int_increasing_inclusive_range(T::TWO, T::MAX))
}

pub fn exhaustive_primitive_int_gen_var_3<T: PrimitiveInt>() -> It<T> {
    Box::new(primitive_int_increasing_range(T::ZERO, T::exact_from(36)))
}

pub fn exhaustive_primitive_int_gen_var_4<T: PrimitiveInt>() -> It<T> {
    Box::new(primitive_int_increasing_inclusive_range(
        T::TWO,
        T::exact_from(36),
    ))
}

// -- (PrimitiveInt, PrimitiveInt) --

pub fn exhaustive_primitive_int_pair_gen_var_1<T: PrimitiveInt, U: ExactFrom<u8> + PrimitiveInt>(
) -> It<(T, U)> {
    Box::new(lex_pairs(
        exhaustive_positive_primitive_ints(),
        primitive_int_increasing_inclusive_range(U::TWO, U::exact_from(36u8)),
    ))
}

pub fn exhaustive_primitive_int_pair_gen_var_2<T: PrimitiveInt, U: PrimitiveInt>() -> It<(T, U)> {
    Box::new(exhaustive_pairs_big_tiny(
        exhaustive_positive_primitive_ints(),
        exhaustive_positive_primitive_ints(),
    ))
}

pub fn exhaustive_primitive_int_pair_gen_var_3<T: PrimitiveInt, U: PrimitiveInt>() -> T1<T, U> {
    Box::new(exhaustive_pairs_big_small(
        exhaustive_positive_primitive_ints(),
        primitive_int_increasing_inclusive_range(U::TWO, U::MAX),
    ))
}

// -- (PrimitiveInt, PrimitiveUnsigned) --

pub fn exhaustive_primitive_int_unsigned_pair_gen_var_1<T: PrimitiveInt, U: PrimitiveUnsigned>(
) -> It<(T, U)> {
    Box::new(exhaustive_pairs_big_tiny(
        exhaustive_positive_primitive_ints(),
        exhaustive_unsigneds(),
    ))
}

// -- (PrimitiveInt, RoundingMode) --

pub fn exhaustive_primitive_int_rounding_mode_pair_gen_var_1<T: PrimitiveInt>(
) -> It<(T, RoundingMode)> {
    Box::new(lex_pairs(
        exhaustive_positive_primitive_ints(),
        exhaustive_rounding_modes(),
    ))
}

// -- PrimitiveSigned --

pub fn exhaustive_signed_gen<T: PrimitiveSigned>() -> It<T> {
    Box::new(exhaustive_signeds())
}

pub fn exhaustive_signed_gen_var_1<T: PrimitiveSigned>() -> It<T> {
    Box::new(exhaustive_signed_inclusive_range(T::MIN + T::ONE, T::MAX))
}

pub fn exhaustive_signed_gen_var_2<T: PrimitiveSigned>() -> It<T> {
    Box::new(exhaustive_natural_signeds())
}

pub fn exhaustive_signed_gen_var_3<T: PrimitiveSigned>() -> It<T> {
    Box::new(exhaustive_signeds().filter(|&x| x != T::ZERO && x != T::NEGATIVE_ONE))
}

pub fn exhaustive_signed_gen_var_4<T: PrimitiveSigned>() -> It<T> {
    Box::new(exhaustive_negative_signeds())
}

pub fn exhaustive_signed_gen_var_5<T: PrimitiveSigned>() -> It<T> {
    Box::new(exhaustive_nonzero_signeds())
}

// -- (PrimitiveSigned, PrimitiveSigned) --

pub fn exhaustive_signed_pair_gen<T: PrimitiveSigned>() -> It<(T, T)> {
    Box::new(exhaustive_pairs_from_single(exhaustive_signeds()))
}

pub fn exhaustive_signed_pair_gen_var_1<T: PrimitiveSigned>() -> It<(T, T)> {
    Box::new(
        exhaustive_pairs_from_single(exhaustive_natural_signeds())
            .interleave(exhaustive_pairs_from_single(exhaustive_negative_signeds())),
    )
}

pub fn exhaustive_signed_pair_gen_var_3<T: PrimitiveSigned, U: PrimitiveSigned>() -> It<(T, U)> {
    Box::new(exhaustive_pairs_big_tiny(
        exhaustive_signeds(),
        exhaustive_signeds(),
    ))
}

struct TakeWhileExtra<I: Iterator, P: Fn(&I::Item) -> bool> {
    xs: I,
    p: P,
    false_seen: bool,
}

impl<I: Iterator, P: Fn(&I::Item) -> bool> Iterator for TakeWhileExtra<I, P> {
    type Item = I::Item;

    fn next(&mut self) -> Option<I::Item> {
        loop {
            if let Some(x) = self.xs.next() {
                if (self.p)(&x) {
                    return Some(x);
                } else if self.false_seen {
                    return None;
                } else {
                    self.false_seen = true;
                }
            } else {
                return None;
            }
        }
    }
}

#[inline]
fn take_while_extra<I: Iterator, P: Fn(&I::Item) -> bool>(xs: I, p: P) -> TakeWhileExtra<I, P> {
    TakeWhileExtra {
        xs,
        p,
        false_seen: false,
    }
}

struct SignedDivisiblePairsGenerator<T: PrimitiveSigned> {
    phantom: PhantomData<*const T>,
}

impl<T: PrimitiveSigned> ExhaustiveDependentPairsYsGenerator<T, T, It<T>>
    for SignedDivisiblePairsGenerator<T>
{
    #[inline]
    fn get_ys(&self, y: &T) -> It<T> {
        // A simple take_while doesn't work. For example, if T is i8 and y is 64, trying to checked-
        // multiply y by the exhaustive signeds gives [Some(0), Some(64), Some(-64), None,
        // Some(-128), None, None, ...], where the first None corresponds to 128, which is not
        // representable as an i8. Doing a take_while would lose the Some(-128). Instead, we use
        // take_while_extra, which is like a take_while, but it waits until it sees a second None to
        // stop iterating.
        let y = *y;
        Box::new(
            take_while_extra(
                exhaustive_signeds().map(move |k| y.checked_mul(k)),
                Option::is_some,
            )
            .map(Option::unwrap),
        )
    }
}

pub fn exhaustive_signed_pair_gen_var_4<T: PrimitiveSigned>() -> It<(T, T)> {
    permute_2_1(Box::new(exhaustive_dependent_pairs(
        bit_distributor_sequence(
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::normal(1),
        ),
        exhaustive_nonzero_signeds(),
        SignedDivisiblePairsGenerator {
            phantom: PhantomData,
        },
    )))
}

pub fn exhaustive_signed_pair_gen_var_5<T: PrimitiveSigned>() -> It<(T, T)> {
    Box::new(
        exhaustive_pairs(exhaustive_signeds(), exhaustive_nonzero_signeds())
            .filter(|&(x, y)| x != T::MIN || y != T::NEGATIVE_ONE),
    )
}

pub fn exhaustive_signed_pair_gen_var_6<T: PrimitiveSigned>() -> It<(T, T)> {
    Box::new(
        exhaustive_pairs(exhaustive_signeds::<T>(), exhaustive_nonzero_signeds::<T>())
            .filter(|&(x, y)| !x.divisible_by(y)),
    )
}

pub fn exhaustive_signed_pair_gen_var_7<T: PrimitiveSigned>() -> It<(T, T)> {
    Box::new(exhaustive_pairs(
        exhaustive_signeds(),
        exhaustive_nonzero_signeds(),
    ))
}

// -- (PrimitiveSigned, PrimitiveSigned, PrimitiveSigned) --

pub fn exhaustive_signed_triple_gen<T: PrimitiveSigned>() -> It<(T, T, T)> {
    Box::new(exhaustive_triples_from_single(exhaustive_signeds()))
}

pub fn exhaustive_signed_triple_gen_var_1<T: PrimitiveSigned>() -> It<(T, T, T)> {
    Box::new(
        exhaustive_triples_from_single(exhaustive_signeds())
            .filter(|&(x, y, z)| add_mul_inputs_valid(x, y, z)),
    )
}

pub fn exhaustive_signed_triple_gen_var_2<T: PrimitiveSigned>() -> It<(T, T, T)> {
    Box::new(
        exhaustive_triples_from_single(exhaustive_signeds())
            .filter(|&(x, y, z)| sub_mul_inputs_valid(x, y, z)),
    )
}

pub fn exhaustive_signed_triple_gen_var_3<T: PrimitiveSigned>() -> It<(T, T, T)> {
    Box::new(
        exhaustive_triples_from_single(exhaustive_natural_signeds())
            .interleave(exhaustive_triples_from_single(exhaustive_negative_signeds())),
    )
}

struct SignedModEqTriplesInnerGenerator<U: PrimitiveUnsigned, S: PrimitiveSigned> {
    phantom_u: PhantomData<*const U>,
    phantom_s: PhantomData<*const S>,
}

impl<U: PrimitiveUnsigned, S: PrimitiveSigned + WrappingFrom<U>>
    ExhaustiveDependentPairsYsGenerator<(U, U), (S, S), It<(S, S)>>
    for SignedModEqTriplesInnerGenerator<U, S>
{
    #[inline]
    fn get_ys(&self, p: &(U, U)) -> It<(S, S)> {
        let &(m, k) = p;
        if k == U::ZERO {
            Box::new(exhaustive_signeds().map(|x| (x, x)))
        } else {
            let d = m.checked_mul(k).unwrap();
            let d_s = S::wrapping_from(d);
            Box::new(
                exhaustive_signed_inclusive_range(S::MIN, S::MAX.wrapping_sub(d_s))
                    .map(move |n| (n, n.wrapping_add(d_s)))
                    .interleave(
                        exhaustive_signed_inclusive_range(S::MIN, S::MAX.wrapping_sub(d_s))
                            .map(move |n| (n.wrapping_add(d_s), n)),
                    ),
            )
        }
    }
}

struct SignedModEqTriplesGenerator<U: PrimitiveUnsigned, S: PrimitiveSigned> {
    phantom_u: PhantomData<*const U>,
    phantom_s: PhantomData<*const S>,
}

impl<U: PrimitiveUnsigned, S: PrimitiveSigned + UnsignedAbs<Output = U> + WrappingFrom<U>>
    ExhaustiveDependentPairsYsGenerator<S, (S, S), It<(S, S)>>
    for SignedModEqTriplesGenerator<U, S>
{
    #[inline]
    fn get_ys(&self, m: &S) -> It<(S, S)> {
        let m = *m;
        let m_abs = m.unsigned_abs();
        if m == S::ZERO {
            Box::new(exhaustive_signeds().map(|x| (x, x)))
        } else {
            Box::new(
                exhaustive_dependent_pairs(
                    bit_distributor_sequence(
                        BitDistributorOutputType::normal(1),
                        BitDistributorOutputType::normal(1),
                    ),
                    primitive_int_increasing_inclusive_range(U::ZERO, U::MAX / m_abs)
                        .map(move |k| (m_abs, k)),
                    SignedModEqTriplesInnerGenerator {
                        phantom_u: PhantomData,
                        phantom_s: PhantomData,
                    },
                )
                .map(|p| p.1),
            )
        }
    }
}

pub fn exhaustive_signed_triple_gen_var_4<
    U: PrimitiveUnsigned,
    S: PrimitiveSigned + UnsignedAbs<Output = U> + WrappingFrom<U>,
>() -> It<(S, S, S)> {
    reshape_2_1_to_3(permute_2_1(Box::new(exhaustive_dependent_pairs(
        bit_distributor_sequence(
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::normal(1),
        ),
        exhaustive_signeds(),
        SignedModEqTriplesGenerator {
            phantom_u: PhantomData,
            phantom_s: PhantomData,
        },
    ))))
}

pub fn exhaustive_signed_triple_gen_var_5<T: PrimitiveSigned>() -> It<(T, T, T)> {
    Box::new(
        exhaustive_triples_from_single(exhaustive_signeds::<T>())
            .filter(|&(x, y, m)| !x.eq_mod(y, m)),
    )
}

// -- (PrimitiveSigned, PrimitiveSigned, PrimitiveSigned, PrimitiveSigned) --

pub fn exhaustive_signed_quadruple_gen<T: PrimitiveSigned>() -> It<(T, T, T, T)> {
    Box::new(exhaustive_quadruples_from_single(exhaustive_signeds()))
}

// -- (PrimitiveSigned, PrimitiveSigned, PrimitiveSigned, PrimitiveUnsigned) --

pub fn exhaustive_signed_signed_signed_unsigned_quadruple_gen_var_1<
    T: PrimitiveSigned,
    U: PrimitiveUnsigned,
>() -> It<(T, T, T, U)> {
    Box::new(exhaustive_quadruples_xxxy_custom_output(
        exhaustive_signeds(),
        exhaustive_unsigneds(),
        BitDistributorOutputType::normal(1),
        BitDistributorOutputType::normal(1),
        BitDistributorOutputType::normal(1),
        BitDistributorOutputType::tiny(),
    ))
}

// -- (PrimitiveSigned, PrimitiveSigned, PrimitiveUnsigned) --

struct SignedModPow2EqTriplesInnerGenerator<U: PrimitiveUnsigned, S: PrimitiveSigned> {
    phantom_u: PhantomData<*const U>,
    phantom_s: PhantomData<*const S>,
}

impl<U: PrimitiveUnsigned, S: PrimitiveSigned + WrappingFrom<U>>
    ExhaustiveDependentPairsYsGenerator<(u64, U), (S, S), It<(S, S)>>
    for SignedModPow2EqTriplesInnerGenerator<U, S>
{
    #[inline]
    fn get_ys(&self, p: &(u64, U)) -> It<(S, S)> {
        let &(pow, k) = p;
        if k == U::ZERO {
            Box::new(exhaustive_signeds().map(|x| (x, x)))
        } else {
            let d = k << pow;
            let d_s = S::wrapping_from(d);
            Box::new(
                exhaustive_signed_inclusive_range(S::MIN, S::MAX.wrapping_sub(d_s))
                    .map(move |n| (n, n.wrapping_add(d_s)))
                    .interleave(
                        exhaustive_signed_inclusive_range(S::MIN, S::MAX.wrapping_sub(d_s))
                            .map(move |n| (n.wrapping_add(d_s), n)),
                    ),
            )
        }
    }
}

struct SignedModPow2EqTriplesGenerator<U: PrimitiveUnsigned, S: PrimitiveSigned> {
    phantom_u: PhantomData<*const U>,
    phantom_s: PhantomData<*const S>,
}

impl<U: PrimitiveUnsigned, S: PrimitiveSigned + WrappingFrom<U>>
    ExhaustiveDependentPairsYsGenerator<u64, (S, S), It<(S, S)>>
    for SignedModPow2EqTriplesGenerator<U, S>
{
    #[inline]
    fn get_ys(&self, pow: &u64) -> It<(S, S)> {
        let pow = *pow;
        if pow >= S::WIDTH {
            Box::new(exhaustive_signeds().map(|x| (x, x)))
        } else {
            Box::new(
                exhaustive_dependent_pairs(
                    bit_distributor_sequence(
                        BitDistributorOutputType::normal(1),
                        BitDistributorOutputType::normal(1),
                    ),
                    primitive_int_increasing_inclusive_range(U::ZERO, U::MAX >> pow)
                        .map(move |k| (pow, k)),
                    SignedModPow2EqTriplesInnerGenerator {
                        phantom_u: PhantomData,
                        phantom_s: PhantomData,
                    },
                )
                .map(|p| p.1),
            )
        }
    }
}

pub fn exhaustive_signed_signed_unsigned_triple_gen_var_1<
    U: PrimitiveUnsigned,
    S: PrimitiveSigned + WrappingFrom<U>,
>() -> It<(S, S, u64)> {
    reshape_2_1_to_3(permute_2_1(Box::new(exhaustive_dependent_pairs(
        ruler_sequence(),
        exhaustive_unsigneds(),
        SignedModPow2EqTriplesGenerator::<U, S> {
            phantom_u: PhantomData,
            phantom_s: PhantomData,
        },
    ))))
}

// -- (PrimitiveSigned, PrimitiveSigned, RoundingMode) --

pub fn exhaustive_signed_signed_rounding_mode_triple_gen_var_1<T: PrimitiveSigned>(
) -> It<(T, T, RoundingMode)> {
    reshape_2_1_to_3(Box::new(
        lex_pairs(
            exhaustive_pairs(exhaustive_signeds::<T>(), exhaustive_nonzero_signeds::<T>()),
            exhaustive_rounding_modes(),
        )
        .filter(|&((x, y), rm)| {
            (x != T::MIN || y != T::NEGATIVE_ONE)
                && (rm != RoundingMode::Exact || x.divisible_by(y))
        }),
    ))
}

fn round_to_multiple_unsigned_helper<T: PrimitiveUnsigned>(x: T, y: T, rm: RoundingMode) -> bool {
    if x == y {
        true
    } else if y == T::ZERO {
        rm == RoundingMode::Down || rm == RoundingMode::Floor || rm == RoundingMode::Nearest
    } else {
        x.div_round(y, rm).checked_mul(y).is_some()
    }
}

fn round_to_multiple_signed_helper<
    U: PrimitiveUnsigned,
    S: CheckedFrom<U> + ConvertibleFrom<U> + PrimitiveSigned + UnsignedAbs<Output = U>,
>(
    x: S,
    y: S,
    rm: RoundingMode,
) -> bool {
    let x_abs = x.unsigned_abs();
    let y_abs = y.unsigned_abs();
    if x >= S::ZERO {
        round_to_multiple_unsigned_helper(x_abs, y_abs, rm)
            && S::convertible_from(x_abs.round_to_multiple(y_abs, rm))
    } else if !round_to_multiple_unsigned_helper(x_abs, y_abs, -rm) {
        false
    } else {
        let abs_result = x_abs.round_to_multiple(y_abs, -rm);
        abs_result == S::MIN.unsigned_abs()
            || S::checked_from(abs_result)
                .and_then(CheckedNeg::checked_neg)
                .is_some()
    }
}

pub(crate) fn round_to_multiple_signed_filter_map<
    U: PrimitiveUnsigned,
    S: CheckedFrom<U> + ConvertibleFrom<U> + PrimitiveSigned + UnsignedAbs<Output = U>,
>(
    x: S,
    y: S,
    rm: RoundingMode,
) -> Option<(S, S, RoundingMode)> {
    if rm != RoundingMode::Exact {
        if round_to_multiple_signed_helper(x, y, rm) {
            Some((x, y, rm))
        } else {
            None
        }
    } else {
        x.checked_mul(y).map(|product| (product, y, rm))
    }
}

pub fn exhaustive_signed_signed_rounding_mode_triple_gen_var_2<
    U: PrimitiveUnsigned,
    S: CheckedFrom<U> + ConvertibleFrom<U> + PrimitiveSigned + UnsignedAbs<Output = U>,
>() -> It<(S, S, RoundingMode)> {
    Box::new(
        lex_pairs(
            exhaustive_pairs(exhaustive_signeds(), exhaustive_nonzero_signeds()),
            exhaustive_rounding_modes(),
        )
        .filter_map(|((x, y), rm)| round_to_multiple_signed_filter_map::<U, S>(x, y, rm)),
    )
}

// -- (PrimitiveSigned, PrimitiveUnsigned) --

pub fn exhaustive_signed_unsigned_pair_gen<T: PrimitiveSigned, U: PrimitiveUnsigned>() -> It<(T, U)>
{
    Box::new(exhaustive_pairs(
        exhaustive_signeds(),
        exhaustive_unsigneds(),
    ))
}

pub fn exhaustive_signed_unsigned_pair_gen_var_2<T: PrimitiveSigned, U: PrimitiveUnsigned>(
) -> It<(T, U)> {
    Box::new(exhaustive_pairs_big_tiny(
        exhaustive_signeds(),
        exhaustive_unsigneds(),
    ))
}

pub fn exhaustive_signed_unsigned_pair_gen_var_3<T: PrimitiveSigned>() -> It<(T, u64)> {
    Box::new(
        lex_pairs(
            exhaustive_natural_signeds(),
            primitive_int_increasing_range(0, T::WIDTH),
        )
        .interleave(exhaustive_pairs(
            exhaustive_negative_signeds(),
            exhaustive_unsigneds(),
        )),
    )
}

pub fn exhaustive_signed_unsigned_pair_gen_var_4<T: PrimitiveSigned>() -> It<(T, u64)> {
    Box::new(lex_pairs(
        exhaustive_signeds(),
        primitive_int_increasing_range(0, T::WIDTH),
    ))
}

pub fn exhaustive_signed_unsigned_pair_gen_var_5<T: PrimitiveSigned>() -> It<(T, u64)> {
    Box::new(
        lex_pairs(
            exhaustive_negative_signeds(),
            primitive_int_increasing_range(0, T::WIDTH),
        )
        .interleave(exhaustive_pairs(
            exhaustive_natural_signeds(),
            exhaustive_unsigneds(),
        )),
    )
}

pub fn exhaustive_signed_unsigned_pair_gen_var_6<
    T: PrimitiveSigned,
    U: ExactFrom<u8> + PrimitiveUnsigned,
>() -> It<(T, U)> {
    Box::new(lex_pairs(
        exhaustive_signeds(),
        primitive_int_increasing_inclusive_range(U::TWO, U::exact_from(36u8)),
    ))
}

pub fn exhaustive_signed_unsigned_pair_gen_var_7<T: PrimitiveSigned, U: PrimitiveUnsigned>(
) -> It<(T, U)> {
    Box::new(exhaustive_pairs_big_tiny(
        exhaustive_natural_signeds(),
        exhaustive_unsigneds(),
    ))
}

pub fn exhaustive_signed_unsigned_pair_gen_var_8<
    T: PrimitiveSigned,
    U: ExactFrom<u8> + PrimitiveUnsigned,
>() -> It<(T, U)> {
    Box::new(lex_pairs(
        exhaustive_natural_signeds(),
        primitive_int_increasing_inclusive_range(U::TWO, U::exact_from(36u8)),
    ))
}

pub fn exhaustive_signed_unsigned_pair_gen_var_9<T: PrimitiveSigned, U: PrimitiveUnsigned>(
) -> It<(T, U)> {
    Box::new(
        exhaustive_pairs_big_tiny(exhaustive_signeds::<T>(), exhaustive_unsigneds::<U>())
            .filter(|&(x, y)| !x.divisible_by_power_of_2(y.exact_into())),
    )
}

struct SignedDivisibleByP2PairsGenerator<T: PrimitiveSigned> {
    phantom: PhantomData<*const T>,
}

impl<T: PrimitiveSigned> ExhaustiveDependentPairsYsGenerator<u64, T, It<T>>
    for SignedDivisibleByP2PairsGenerator<T>
{
    #[inline]
    fn get_ys(&self, pow: &u64) -> It<T> {
        let pow = *pow;
        if pow >= T::WIDTH {
            Box::new(once(T::ZERO))
        } else if pow == 0 {
            Box::new(exhaustive_signeds())
        } else {
            Box::new(
                exhaustive_signed_inclusive_range(
                    -T::low_mask(T::WIDTH - pow),
                    T::low_mask(T::WIDTH - pow),
                )
                .map(move |k| k << pow),
            )
        }
    }
}

pub fn exhaustive_signed_unsigned_pair_gen_var_10<T: PrimitiveSigned>() -> It<(T, u64)> {
    permute_2_1(Box::new(exhaustive_dependent_pairs(
        ruler_sequence(),
        exhaustive_unsigneds(),
        SignedDivisibleByP2PairsGenerator {
            phantom: PhantomData,
        },
    )))
}

pub fn exhaustive_signed_unsigned_pair_gen_var_11<T: PrimitiveSigned>() -> It<(T, u64)> {
    Box::new(
        exhaustive_pairs_big_tiny(exhaustive_natural_signeds(), exhaustive_unsigneds()).interleave(
            exhaustive_pairs_big_tiny(
                exhaustive_signeds(),
                primitive_int_increasing_inclusive_range(0, T::WIDTH),
            ),
        ),
    )
}

pub fn exhaustive_signed_unsigned_pair_gen_var_12<T: PrimitiveSigned>() -> It<(T, u64)> {
    Box::new(
        exhaustive_pairs_big_tiny(
            exhaustive_signed_range(T::MIN + T::ONE, T::ONE),
            exhaustive_unsigneds(),
        )
        .interleave(exhaustive_pairs_big_tiny(
            exhaustive_signeds(),
            primitive_int_increasing_range(0, T::WIDTH),
        )),
    )
}

pub fn exhaustive_signed_unsigned_pair_gen_var_13<T: PrimitiveSigned, U: PrimitiveInt>(
) -> It<(T, u64)> {
    Box::new(lex_pairs(
        exhaustive_signeds(),
        primitive_int_increasing_inclusive_range(0, U::WIDTH),
    ))
}

pub fn exhaustive_signed_unsigned_pair_gen_var_14<T: PrimitiveSigned, U: PrimitiveUnsigned>(
) -> It<(T, U)> {
    Box::new(exhaustive_pairs(
        exhaustive_signed_inclusive_range(
            if T::WIDTH <= u64::WIDTH {
                T::MIN
            } else {
                -T::exact_from(u64::MAX)
            },
            T::saturating_from(u64::MAX),
        ),
        exhaustive_positive_primitive_ints(),
    ))
}

pub fn exhaustive_signed_unsigned_pair_gen_var_15<T: PrimitiveSigned>() -> It<(T, u64)> {
    Box::new(
        exhaustive_pairs(exhaustive_signeds::<T>(), exhaustive_unsigneds())
            .filter(|&(x, y)| x.checked_pow(y).is_some()),
    )
}

pub fn exhaustive_signed_unsigned_pair_gen_var_16<T: PrimitiveSigned, U: PrimitiveUnsigned>(
) -> It<(T, U)> {
    Box::new(exhaustive_pairs_big_tiny(
        exhaustive_signed_range(T::MIN + T::ONE, T::ZERO),
        exhaustive_unsigneds(),
    ))
}

// -- (PrimitiveSigned, PrimitiveUnsigned, bool) --

pub fn exhaustive_signed_unsigned_bool_triple_gen_var_1<T: PrimitiveSigned>() -> It<(T, u64, bool)>
{
    Box::new(
        exhaustive_pairs_big_tiny(exhaustive_signeds(), exhaustive_unsigneds())
            .map(|(x, y)| (x, y, x < T::ZERO))
            .interleave(
                lex_pairs(
                    exhaustive_signeds(),
                    primitive_int_increasing_range(0, T::WIDTH),
                )
                .map(|(x, y)| (x, y, x >= T::ZERO)),
            ),
    )
}

// -- (PrimitiveSigned, PrimitiveUnsigned, PrimitiveUnsigned) --

pub fn exhaustive_signed_unsigned_unsigned_triple_gen_var_1<
    T: PrimitiveSigned,
    U: PrimitiveUnsigned,
    V: PrimitiveUnsigned,
>() -> It<(T, U, V)> {
    Box::new(exhaustive_triples_custom_output(
        exhaustive_signeds(),
        exhaustive_unsigneds(),
        exhaustive_unsigneds(),
        BitDistributorOutputType::normal(1),
        BitDistributorOutputType::normal(1),
        BitDistributorOutputType::tiny(),
    ))
}

pub fn exhaustive_signed_unsigned_unsigned_triple_gen_var_2<
    T: PrimitiveSigned,
    U: PrimitiveUnsigned,
>() -> It<(T, U, U)> {
    Box::new(
        exhaustive_triples_xyy_custom_output(
            exhaustive_positive_primitive_ints(),
            exhaustive_unsigneds(),
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::tiny(),
            BitDistributorOutputType::tiny(),
        )
        .interleave(exhaustive_triples_custom_output(
            exhaustive_signed_inclusive_range(T::MIN, T::ZERO),
            exhaustive_unsigneds(),
            primitive_int_increasing_inclusive_range(U::ZERO, U::exact_from(T::WIDTH)),
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::tiny(),
            BitDistributorOutputType::tiny(),
        ))
        .filter_map(|(x, y, z)| y.checked_add(z).map(|new_z| (x, y, new_z))),
    )
}

pub fn exhaustive_signed_unsigned_unsigned_triple_gen_var_3<
    T: PrimitiveSigned,
    U: ExactFrom<u8> + PrimitiveUnsigned,
    V: PrimitiveUnsigned,
>() -> It<(T, U, V)> {
    permute_1_3_2(reshape_2_1_to_3(Box::new(lex_pairs(
        exhaustive_pairs_big_tiny(exhaustive_signeds(), exhaustive_unsigneds()),
        primitive_int_increasing_inclusive_range(U::TWO, U::exact_from(36u8)),
    ))))
}

pub fn exhaustive_signed_unsigned_unsigned_triple_gen_var_4<
    T: PrimitiveSigned,
    U: PrimitiveUnsigned,
>() -> It<(T, T, U)> {
    Box::new(exhaustive_triples_xxy_custom_output(
        exhaustive_signeds(),
        exhaustive_unsigneds(),
        BitDistributorOutputType::normal(1),
        BitDistributorOutputType::normal(1),
        BitDistributorOutputType::tiny(),
    ))
}

pub fn exhaustive_signed_signed_unsigned_triple_gen_var_5<T: PrimitiveSigned>() -> It<(T, T, u64)> {
    Box::new(
        exhaustive_triples_xxy_custom_output(
            exhaustive_signeds::<T>(),
            exhaustive_unsigneds(),
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::tiny(),
        )
        .filter(|&(x, y, pow)| !x.eq_mod_power_of_2(y, pow)),
    )
}

// -- (PrimitiveSigned, PrimitiveUnsigned, PrimitiveUnsigned, PrimitiveUnsigned) --

pub fn exhaustive_signed_unsigned_unsigned_unsigned_quadruple_gen_var_1<
    T: PrimitiveSigned + UnsignedAbs<Output = U>,
    U: BitBlockAccess<Bits = U> + PrimitiveUnsigned,
>() -> It<(T, u64, u64, U)> {
    Box::new(
        exhaustive_quadruples_xyyz_custom_output(
            exhaustive_signeds(),
            exhaustive_unsigneds(),
            exhaustive_unsigneds(),
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::tiny(),
            BitDistributorOutputType::tiny(),
            BitDistributorOutputType::normal(1),
        )
        .filter_map(|(x, y, z, w): (T, u64, u64, U)| {
            y.checked_add(z).and_then(|new_z| {
                if signed_assign_bits_valid(x, y, new_z, w) {
                    Some((x, y, new_z, w))
                } else {
                    None
                }
            })
        }),
    )
}

// -- (PrimitiveSigned, PrimitiveUnsigned, RoundingMode) --

pub fn exhaustive_signed_unsigned_rounding_mode_triple_gen_var_1<T: PrimitiveSigned>(
) -> It<(T, u64, RoundingMode)> {
    Box::new(
        lex_pairs(
            exhaustive_pairs_big_small(exhaustive_signeds::<T>(), exhaustive_unsigneds::<u64>()),
            exhaustive_rounding_modes(),
        )
        .filter_map(|((x, pow), rm)| round_to_multiple_of_power_of_2_filter_map(x, pow, rm)),
    )
}

// -- (PrimitiveSigned, RoundingMode) --

pub fn exhaustive_signed_rounding_mode_pair_gen<T: PrimitiveSigned>() -> It<(T, RoundingMode)> {
    Box::new(lex_pairs(exhaustive_signeds(), exhaustive_rounding_modes()))
}

pub fn exhaustive_signed_rounding_mode_pair_gen_var_1<T: PrimitiveSigned>() -> It<(T, RoundingMode)>
{
    Box::new(lex_pairs(
        exhaustive_nonzero_signeds(),
        exhaustive_rounding_modes(),
    ))
}

pub fn exhaustive_signed_rounding_mode_pair_gen_var_2<T: PrimitiveSigned>() -> It<(T, RoundingMode)>
{
    Box::new(lex_pairs(
        exhaustive_signed_inclusive_range(T::MIN + T::ONE, T::MAX),
        exhaustive_rounding_modes(),
    ))
}

pub fn exhaustive_signed_rounding_mode_pair_gen_var_3<T: PrimitiveSigned>() -> It<(T, RoundingMode)>
{
    Box::new(lex_pairs(
        exhaustive_nonzero_signeds().filter(|&x| x != T::MIN),
        exhaustive_rounding_modes(),
    ))
}

// -- (PrimitiveSigned, Vec<bool>) --

struct SignedBoolVecPairGeneratorVar1;

impl<T: PrimitiveSigned>
    ExhaustiveDependentPairsYsGenerator<T, Vec<bool>, LexFixedLengthVecsFromSingle<ExhaustiveBools>>
    for SignedBoolVecPairGeneratorVar1
{
    #[inline]
    fn get_ys(&self, &x: &T) -> LexFixedLengthVecsFromSingle<ExhaustiveBools> {
        lex_fixed_length_vecs_from_single(
            u64::exact_from(x.to_bits_asc().len()),
            exhaustive_bools(),
        )
    }
}

pub fn exhaustive_signed_bool_vec_pair_gen_var_1<T: PrimitiveSigned>() -> It<(T, Vec<bool>)> {
    Box::new(exhaustive_dependent_pairs(
        bit_distributor_sequence(
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::normal(1),
        ),
        exhaustive_signeds(),
        SignedBoolVecPairGeneratorVar1,
    ))
}

// -- PrimitiveUnsigned --

pub fn exhaustive_unsigned_gen<T: PrimitiveUnsigned>() -> It<T> {
    Box::new(exhaustive_unsigneds())
}

pub fn exhaustive_unsigned_gen_var_1() -> It<u32> {
    Box::new(primitive_int_increasing_range(0, NUMBER_OF_CHARS))
}

pub fn exhaustive_unsigned_gen_var_2<T: PrimitiveInt>() -> It<u64> {
    Box::new(primitive_int_increasing_inclusive_range(1, T::WIDTH))
}

pub fn exhaustive_unsigned_gen_var_4<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned + SaturatingFrom<T>,
>() -> It<U> {
    Box::new(primitive_int_increasing_inclusive_range(
        U::TWO,
        U::saturating_from(T::MAX),
    ))
}

pub fn exhaustive_unsigned_gen_var_5<T: PrimitiveInt>() -> It<u64> {
    Box::new(primitive_int_increasing_inclusive_range(0, T::WIDTH))
}

pub fn exhaustive_unsigned_gen_var_6() -> It<u8> {
    Box::new(
        lex_union3s(
            primitive_int_increasing_inclusive_range(b'0', b'9'),
            primitive_int_increasing_inclusive_range(b'a', b'z'),
            primitive_int_increasing_inclusive_range(b'A', b'Z'),
        )
        .map(Union3::unwrap),
    )
}

pub fn exhaustive_unsigned_gen_var_7<T: PrimitiveUnsigned>() -> It<T> {
    Box::new(primitive_int_increasing_inclusive_range(
        T::power_of_2(T::WIDTH - 1),
        T::MAX,
    ))
}

pub fn exhaustive_unsigned_gen_var_8<T: PrimitiveFloat>() -> It<u64> {
    Box::new(primitive_int_increasing_inclusive_range(
        0,
        T::LARGEST_ORDERED_REPRESENTATION,
    ))
}

pub fn exhaustive_unsigned_gen_var_9<T: PrimitiveUnsigned>() -> It<T> {
    Box::new(primitive_int_increasing_inclusive_range(
        T::ZERO,
        T::power_of_2(T::WIDTH - 1),
    ))
}

pub fn exhaustive_unsigned_gen_var_10<T: PrimitiveInt>() -> It<u64> {
    Box::new(primitive_int_increasing_range(0, T::WIDTH))
}

pub fn exhaustive_unsigned_gen_var_11<T: PrimitiveInt>() -> It<u64> {
    Box::new(primitive_int_increasing_inclusive_range(0, T::WIDTH - 2))
}

// -- (PrimitiveUnsigned, PrimitiveInt) --

pub fn exhaustive_unsigned_primitive_int_gen_var_1<T: PrimitiveUnsigned, U: PrimitiveInt>(
) -> It<(T, U)> {
    Box::new(exhaustive_pairs(
        exhaustive_unsigneds(),
        exhaustive_positive_primitive_ints(),
    ))
}

pub fn exhaustive_unsigned_primitive_int_pair_gen_var_2<
    T: PrimitiveUnsigned,
    U: ExactFrom<u8> + PrimitiveInt,
>() -> It<(T, U)> {
    Box::new(lex_pairs(
        exhaustive_unsigneds(),
        primitive_int_increasing_inclusive_range(U::TWO, U::exact_from(36u8)),
    ))
}

// -- (PrimitiveUnsigned, PrimitiveInt, PrimitiveUnsigned) --

pub fn exhaustive_unsigned_primitive_int_unsigned_triple_gen_var_3<
    T: PrimitiveUnsigned,
    U: PrimitiveInt,
    V: PrimitiveUnsigned,
>() -> It<(T, u64, V)> {
    Box::new(exhaustive_triples(
        exhaustive_unsigneds(),
        primitive_int_increasing_inclusive_range(1, U::WIDTH),
        exhaustive_unsigneds(),
    ))
}

// -- (PrimitiveUnsigned, PrimitiveSigned) --

pub fn exhaustive_unsigned_signed_pair_gen<T: PrimitiveUnsigned, U: PrimitiveSigned>() -> It<(T, U)>
{
    Box::new(exhaustive_pairs(
        exhaustive_unsigneds(),
        exhaustive_signeds(),
    ))
}

pub fn exhaustive_unsigned_signed_pair_gen_var_1<T: PrimitiveUnsigned, U: PrimitiveSigned>(
) -> It<(T, U)> {
    Box::new(exhaustive_pairs_big_tiny(
        exhaustive_unsigneds(),
        exhaustive_signeds(),
    ))
}

struct IntegerMantissaAndExponentGenerator<T: PrimitiveFloat> {
    phantom: PhantomData<*const T>,
}

impl<T: PrimitiveFloat> ExhaustiveDependentPairsYsGenerator<(u64, i64), (u64, i64), It<(u64, i64)>>
    for IntegerMantissaAndExponentGenerator<T>
{
    #[inline]
    fn get_ys(&self, p: &(u64, i64)) -> It<(u64, i64)> {
        let &(mantissa, exponent) = p;
        Box::new(exhaustive_natural_signeds().flat_map(move |i| {
            Some((
                mantissa.arithmetic_checked_shl(i)?,
                exponent.checked_sub(i)?,
            ))
        }))
    }
}

pub fn exhaustive_unsigned_signed_pair_gen_var_2<T: PrimitiveFloat>() -> It<(u64, i64)> {
    Box::new(
        exhaustive_dependent_pairs(
            bit_distributor_sequence(
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::tiny(),
            ),
            exhaustive_positive_finite_primitive_floats().map(T::integer_mantissa_and_exponent),
            IntegerMantissaAndExponentGenerator::<T> {
                phantom: PhantomData,
            },
        )
        .map(|p| p.1),
    )
}

// -- (PrimitiveUnsigned, PrimitiveSigned, PrimitiveUnsigned) --

struct ModPowerOfTwoPairWithExtraSignedGenerator<T: PrimitiveUnsigned, U: PrimitiveSigned> {
    phantom_t: PhantomData<*const T>,
    phantom_u: PhantomData<*const U>,
}

type LongType<T, U> =
    ExhaustivePairs<T, PrimitiveIntIncreasingRange<T>, U, Chain<Once<U>, PrimitiveIntUpDown<U>>>;

impl<T: PrimitiveUnsigned, U: PrimitiveSigned>
    ExhaustiveDependentPairsYsGenerator<
        u64,
        (T, U),
        ExhaustivePairs<
            T,
            PrimitiveIntIncreasingRange<T>,
            U,
            Chain<Once<U>, PrimitiveIntUpDown<U>>,
        >,
    > for ModPowerOfTwoPairWithExtraSignedGenerator<T, U>
{
    #[inline]
    fn get_ys(&self, &pow: &u64) -> LongType<T, U> {
        exhaustive_pairs(
            primitive_int_increasing_inclusive_range(T::ZERO, T::low_mask(pow)),
            exhaustive_signeds(),
        )
    }
}

pub fn exhaustive_unsigned_signed_unsigned_triple_gen_var_1<
    T: PrimitiveUnsigned,
    U: PrimitiveSigned,
>() -> It<(T, U, u64)> {
    reshape_2_1_to_3(permute_2_1(Box::new(exhaustive_dependent_pairs(
        ruler_sequence(),
        primitive_int_increasing_inclusive_range(0, T::WIDTH),
        ModPowerOfTwoPairWithExtraSignedGenerator::<T, U> {
            phantom_t: PhantomData,
            phantom_u: PhantomData,
        },
    ))))
}

pub fn exhaustive_unsigned_signed_unsigned_triple_gen_var_2<
    T: PrimitiveUnsigned,
    U: PrimitiveSigned,
>() -> It<(T, U, T)> {
    Box::new(
        exhaustive_triples_xyx(
            exhaustive_unsigneds(),
            exhaustive_signed_inclusive_range(
                if U::WIDTH <= u64::WIDTH {
                    U::MIN
                } else {
                    -U::exact_from(u64::MAX)
                },
                U::saturating_from(u64::MAX),
            ),
        )
        .filter_map(|(x, y, z): (T, U, T)| Some((x, y, x.checked_add(z)?.checked_add(T::ONE)?))),
    )
}

// -- (PrimitiveUnsigned, PrimitiveUnsigned) --

pub fn exhaustive_unsigned_pair_gen<T: PrimitiveUnsigned, U: PrimitiveUnsigned>() -> It<(T, U)> {
    Box::new(exhaustive_pairs(
        exhaustive_unsigneds(),
        exhaustive_unsigneds(),
    ))
}

pub fn exhaustive_unsigned_pair_gen_var_1() -> It<(u32, u32)> {
    Box::new(exhaustive_pairs_from_single(
        primitive_int_increasing_range(0, NUMBER_OF_CHARS),
    ))
}

type T1<T, U> = It<(T, U)>;

pub fn exhaustive_unsigned_pair_gen_var_2<T: PrimitiveUnsigned, U: PrimitiveUnsigned>() -> T1<T, U>
{
    Box::new(exhaustive_pairs_big_tiny(
        exhaustive_unsigneds(),
        exhaustive_unsigneds(),
    ))
}

pub fn exhaustive_unsigned_pair_gen_var_3<T: PrimitiveUnsigned>() -> It<(T, u64)> {
    Box::new(lex_pairs(
        exhaustive_unsigneds(),
        primitive_int_increasing_range(0, T::WIDTH),
    ))
}

pub fn exhaustive_unsigned_pair_gen_var_4<T: PrimitiveUnsigned, U: PrimitiveInt>() -> It<(T, u64)> {
    Box::new(lex_pairs(
        exhaustive_unsigneds(),
        primitive_int_increasing_inclusive_range(1, U::WIDTH),
    ))
}

pub fn exhaustive_unsigned_pair_gen_var_5<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned + SaturatingFrom<T>,
>() -> It<(T, U)> {
    Box::new(exhaustive_pairs_big_small(
        exhaustive_unsigneds(),
        primitive_int_increasing_inclusive_range(U::TWO, U::saturating_from(T::MAX)),
    ))
}

//TODO make better
pub fn exhaustive_unsigned_pair_gen_var_6<T: PrimitiveUnsigned>() -> It<(T, T)> {
    Box::new(exhaustive_pairs_from_single(exhaustive_unsigneds()).filter(|(x, y)| x <= y))
}

pub fn exhaustive_unsigned_pair_gen_var_7<
    T: PrimitiveInt + SaturatingFrom<U>,
    U: PrimitiveUnsigned,
    V: PrimitiveUnsigned,
>() -> It<(T, V)> {
    Box::new(exhaustive_pairs_big_tiny(
        primitive_int_increasing_inclusive_range(T::TWO, T::saturating_from(U::MAX)),
        exhaustive_unsigneds(),
    ))
}

struct UnsignedDivisiblePairsGenerator<T: PrimitiveUnsigned> {
    phantom: PhantomData<*const T>,
}

impl<T: PrimitiveUnsigned> ExhaustiveDependentPairsYsGenerator<T, T, It<T>>
    for UnsignedDivisiblePairsGenerator<T>
{
    #[inline]
    fn get_ys(&self, y: &T) -> It<T> {
        let y = *y;
        Box::new(
            exhaustive_unsigneds()
                .map(move |k| y.checked_mul(k))
                .take_while(Option::is_some)
                .map(Option::unwrap),
        )
    }
}

pub fn exhaustive_unsigned_pair_gen_var_8<T: PrimitiveUnsigned>() -> It<(T, T)> {
    permute_2_1(Box::new(exhaustive_dependent_pairs(
        bit_distributor_sequence(
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::normal(1),
        ),
        exhaustive_positive_primitive_ints(),
        UnsignedDivisiblePairsGenerator {
            phantom: PhantomData,
        },
    )))
}

pub fn exhaustive_unsigned_pair_gen_var_9<T: PrimitiveUnsigned, U: PrimitiveUnsigned>() -> T1<T, U>
{
    Box::new(exhaustive_pairs(
        exhaustive_unsigneds(),
        exhaustive_positive_primitive_ints(),
    ))
}

pub fn exhaustive_unsigned_pair_gen_var_10<T: PrimitiveUnsigned>() -> It<(T, T)> {
    Box::new(
        exhaustive_pairs(
            exhaustive_unsigneds::<T>(),
            exhaustive_positive_primitive_ints::<T>(),
        )
        .filter(|&(x, y)| !x.divisible_by(y)),
    )
}

pub fn exhaustive_unsigned_pair_gen_var_11<T: PrimitiveUnsigned, U: PrimitiveUnsigned>() -> T1<T, U>
{
    Box::new(
        exhaustive_pairs_big_tiny(exhaustive_unsigneds::<T>(), exhaustive_unsigneds::<U>())
            .filter(|&(x, y)| !x.divisible_by_power_of_2(y.exact_into())),
    )
}

struct UnsignedDivisibleByP2PairsGenerator<T: PrimitiveUnsigned> {
    phantom: PhantomData<*const T>,
}

impl<T: PrimitiveUnsigned> ExhaustiveDependentPairsYsGenerator<u64, T, It<T>>
    for UnsignedDivisibleByP2PairsGenerator<T>
{
    #[inline]
    fn get_ys(&self, pow: &u64) -> It<T> {
        let pow = *pow;
        if pow >= T::WIDTH {
            Box::new(once(T::ZERO))
        } else {
            Box::new(
                primitive_int_increasing_inclusive_range(T::ZERO, T::low_mask(T::WIDTH - pow))
                    .map(move |k| k << pow),
            )
        }
    }
}

pub fn exhaustive_unsigned_pair_gen_var_12<T: PrimitiveUnsigned>() -> It<(T, u64)> {
    permute_2_1(Box::new(exhaustive_dependent_pairs(
        ruler_sequence(),
        exhaustive_unsigneds(),
        UnsignedDivisibleByP2PairsGenerator {
            phantom: PhantomData,
        },
    )))
}

//TODO make better
pub fn exhaustive_unsigned_pair_gen_var_13<T: PrimitiveUnsigned>() -> It<(T, T)> {
    Box::new(
        exhaustive_pairs_from_single(exhaustive_unsigneds::<T>())
            .flat_map(|(x, y)| Some((x, x.checked_add(y)?.checked_add(T::ONE)?))),
    )
}

struct ModPowerOfTwoSingleGenerator<T: PrimitiveUnsigned> {
    phantom: PhantomData<*const T>,
}

impl<T: PrimitiveUnsigned>
    ExhaustiveDependentPairsYsGenerator<u64, T, PrimitiveIntIncreasingRange<T>>
    for ModPowerOfTwoSingleGenerator<T>
{
    #[inline]
    fn get_ys(&self, &pow: &u64) -> PrimitiveIntIncreasingRange<T> {
        primitive_int_increasing_inclusive_range(T::ZERO, T::low_mask(pow))
    }
}

pub fn exhaustive_unsigned_pair_gen_var_14<T: PrimitiveUnsigned>() -> It<(T, u64)> {
    permute_2_1(Box::new(exhaustive_dependent_pairs(
        ruler_sequence(),
        primitive_int_increasing_inclusive_range(0, T::WIDTH),
        ModPowerOfTwoSingleGenerator {
            phantom: PhantomData,
        },
    )))
}

pub fn exhaustive_unsigned_pair_gen_var_15<T: PrimitiveUnsigned>() -> It<(T, u64)> {
    Box::new(
        exhaustive_unsigneds()
            .map(|x| (T::ZERO, x))
            .interleave(exhaustive_pairs_big_tiny(
                exhaustive_positive_primitive_ints(),
                primitive_int_increasing_range(0, T::WIDTH),
            )),
    )
}

pub fn exhaustive_unsigned_pair_gen_var_16<T: PrimitiveFloat>() -> It<(u64, u64)> {
    Box::new(exhaustive_pairs_from_single(
        primitive_int_increasing_inclusive_range(0, T::LARGEST_ORDERED_REPRESENTATION),
    ))
}

pub fn exhaustive_unsigned_pair_gen_var_17<T: PrimitiveUnsigned, U: PrimitiveInt>() -> It<(T, u64)>
{
    Box::new(lex_pairs(
        exhaustive_unsigneds(),
        primitive_int_increasing_inclusive_range(0, U::WIDTH),
    ))
}

pub fn exhaustive_unsigned_pair_gen_var_18<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
) -> It<(T, U)> {
    Box::new(exhaustive_pairs(
        primitive_int_increasing_inclusive_range(T::ZERO, T::saturating_from(u64::MAX)),
        exhaustive_positive_primitive_ints(),
    ))
}

pub fn exhaustive_unsigned_pair_gen_var_19<T: PrimitiveFloat>() -> It<(u64, u64)> {
    Box::new(exhaustive_pairs(
        primitive_int_increasing_range(0, u64::power_of_2(T::MANTISSA_WIDTH)),
        primitive_int_increasing_range(0, u64::power_of_2(T::EXPONENT_WIDTH)),
    ))
}

pub fn exhaustive_unsigned_pair_gen_var_20<T: PrimitiveUnsigned>() -> It<(T, T)> {
    Box::new(exhaustive_pairs_from_single(exhaustive_unsigneds()))
}

pub fn exhaustive_unsigned_pair_gen_var_21<T: PrimitiveUnsigned>() -> It<(T, u64)> {
    Box::new(
        exhaustive_pairs(exhaustive_unsigneds::<T>(), exhaustive_unsigneds())
            .filter(|&(x, y)| x.checked_pow(y).is_some()),
    )
}

pub fn exhaustive_unsigned_pair_gen_var_22<T: PrimitiveUnsigned>() -> It<(T, u64)> {
    Box::new(exhaustive_unsigned_pair_gen_var_14().map(|(x, p)| (x, T::WIDTH - p)))
}

// -- (PrimitiveUnsigned, PrimitiveUnsigned, bool) --

pub fn exhaustive_unsigned_unsigned_bool_triple_gen_var_1<T: PrimitiveUnsigned>(
) -> It<(T, u64, bool)> {
    Box::new(
        exhaustive_pairs_big_tiny(exhaustive_unsigneds(), exhaustive_unsigneds())
            .map(|(x, y)| (x, y, false))
            .interleave(
                lex_pairs(
                    exhaustive_unsigneds(),
                    primitive_int_increasing_range(0, T::WIDTH),
                )
                .map(|(x, y)| (x, y, true)),
            ),
    )
}

// -- (PrimitiveUnsigned, PrimitiveUnsigned, PrimitiveUnsigned) --

pub fn exhaustive_unsigned_triple_gen<T: PrimitiveUnsigned>() -> It<(T, T, T)> {
    Box::new(exhaustive_triples_from_single(exhaustive_unsigneds()))
}

pub fn exhaustive_unsigned_triple_gen_var_1<T: PrimitiveUnsigned>() -> It<(T, T, T)> {
    Box::new(
        exhaustive_triples_from_single(exhaustive_unsigneds())
            .filter(|&(x, y, z)| add_mul_inputs_valid(x, y, z)),
    )
}

pub fn exhaustive_unsigned_triple_gen_var_2<T: PrimitiveUnsigned>() -> It<(T, T, T)> {
    Box::new(
        exhaustive_triples_from_single(exhaustive_unsigneds())
            .filter(|&(x, y, z)| sub_mul_inputs_valid(x, y, z)),
    )
}

pub fn exhaustive_unsigned_triple_gen_var_3<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
) -> It<(T, T, U)> {
    Box::new(exhaustive_triples_xxy_custom_output(
        exhaustive_unsigneds(),
        exhaustive_unsigneds(),
        BitDistributorOutputType::normal(1),
        BitDistributorOutputType::normal(1),
        BitDistributorOutputType::tiny(),
    ))
}

pub fn exhaustive_unsigned_triple_gen_var_4<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
) -> It<(T, U, U)> {
    Box::new(
        exhaustive_triples_xyy_custom_output(
            exhaustive_unsigneds(),
            exhaustive_unsigneds(),
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::tiny(),
            BitDistributorOutputType::tiny(),
        )
        .filter_map(|(x, y, z): (T, U, U)| y.checked_add(z).map(|new_z| (x, y, new_z))),
    )
}

pub fn exhaustive_unsigned_triple_gen_var_5<
    T: PrimitiveUnsigned,
    U: ExactFrom<u8> + PrimitiveUnsigned,
    V: PrimitiveUnsigned,
>() -> It<(T, U, V)> {
    permute_1_3_2(reshape_2_1_to_3(Box::new(lex_pairs(
        exhaustive_pairs_big_tiny(exhaustive_unsigneds(), exhaustive_unsigneds()),
        primitive_int_increasing_inclusive_range(U::TWO, U::exact_from(36u8)),
    ))))
}

struct UnsignedModEqTriplesInnerGenerator<T: PrimitiveUnsigned> {
    phantom: PhantomData<*const T>,
}

impl<T: PrimitiveUnsigned> ExhaustiveDependentPairsYsGenerator<(T, T), (T, T), It<(T, T)>>
    for UnsignedModEqTriplesInnerGenerator<T>
{
    #[inline]
    fn get_ys(&self, p: &(T, T)) -> It<(T, T)> {
        let &(m, k) = p;
        if k == T::ZERO {
            Box::new(exhaustive_unsigneds().map(|x| (x, x)))
        } else {
            let d = m.checked_mul(k).unwrap();
            Box::new(
                primitive_int_increasing_inclusive_range(T::ZERO, T::MAX - d)
                    .map(move |n| (n, n + d))
                    .interleave(
                        primitive_int_increasing_inclusive_range(T::ZERO, T::MAX - d)
                            .map(move |n| (n + d, n)),
                    ),
            )
        }
    }
}

struct UnsignedModEqTriplesGenerator<T: PrimitiveUnsigned> {
    phantom: PhantomData<*const T>,
}

impl<T: PrimitiveUnsigned> ExhaustiveDependentPairsYsGenerator<T, (T, T), It<(T, T)>>
    for UnsignedModEqTriplesGenerator<T>
{
    #[inline]
    fn get_ys(&self, m: &T) -> It<(T, T)> {
        let m = *m;
        if m == T::ZERO {
            Box::new(exhaustive_unsigneds().map(|x| (x, x)))
        } else {
            Box::new(
                exhaustive_dependent_pairs(
                    bit_distributor_sequence(
                        BitDistributorOutputType::normal(1),
                        BitDistributorOutputType::normal(1),
                    ),
                    primitive_int_increasing_inclusive_range(T::ZERO, T::MAX / m)
                        .map(move |k| (m, k)),
                    UnsignedModEqTriplesInnerGenerator {
                        phantom: PhantomData,
                    },
                )
                .map(|p| p.1),
            )
        }
    }
}

pub fn exhaustive_unsigned_triple_gen_var_6<T: PrimitiveUnsigned>() -> It<(T, T, T)> {
    reshape_2_1_to_3(permute_2_1(Box::new(exhaustive_dependent_pairs(
        bit_distributor_sequence(
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::normal(1),
        ),
        exhaustive_unsigneds(),
        UnsignedModEqTriplesGenerator {
            phantom: PhantomData,
        },
    ))))
}

pub fn exhaustive_unsigned_triple_gen_var_7<T: PrimitiveUnsigned>() -> It<(T, T, T)> {
    Box::new(
        exhaustive_triples_from_single(exhaustive_unsigneds::<T>())
            .filter(|&(x, y, m)| !x.eq_mod(y, m)),
    )
}

struct UnsignedModPow2EqTriplesInnerGenerator<T: PrimitiveUnsigned> {
    phantom: PhantomData<*const T>,
}

impl<T: PrimitiveUnsigned> ExhaustiveDependentPairsYsGenerator<(u64, T), (T, T), It<(T, T)>>
    for UnsignedModPow2EqTriplesInnerGenerator<T>
{
    #[inline]
    fn get_ys(&self, p: &(u64, T)) -> It<(T, T)> {
        let &(pow, k) = p;
        if k == T::ZERO {
            Box::new(exhaustive_unsigneds().map(|x| (x, x)))
        } else {
            let d = k << pow;
            Box::new(
                primitive_int_increasing_inclusive_range(T::ZERO, T::MAX - d)
                    .map(move |n| (n, n + d))
                    .interleave(
                        primitive_int_increasing_inclusive_range(T::ZERO, T::MAX - d)
                            .map(move |n| (n + d, n)),
                    ),
            )
        }
    }
}

struct UnsignedModPow2EqTriplesGenerator<T: PrimitiveUnsigned> {
    phantom: PhantomData<*const T>,
}

impl<T: PrimitiveUnsigned> ExhaustiveDependentPairsYsGenerator<u64, (T, T), It<(T, T)>>
    for UnsignedModPow2EqTriplesGenerator<T>
{
    #[inline]
    fn get_ys(&self, pow: &u64) -> It<(T, T)> {
        let pow = *pow;
        if pow >= T::WIDTH {
            Box::new(exhaustive_unsigneds().map(|x| (x, x)))
        } else {
            Box::new(
                exhaustive_dependent_pairs(
                    bit_distributor_sequence(
                        BitDistributorOutputType::normal(1),
                        BitDistributorOutputType::normal(1),
                    ),
                    primitive_int_increasing_inclusive_range(T::ZERO, T::MAX >> pow)
                        .map(move |k| (pow, k)),
                    UnsignedModPow2EqTriplesInnerGenerator {
                        phantom: PhantomData,
                    },
                )
                .map(|p| p.1),
            )
        }
    }
}

pub fn exhaustive_unsigned_triple_gen_var_8<T: PrimitiveUnsigned>() -> It<(T, T, u64)> {
    reshape_2_1_to_3(permute_2_1(Box::new(exhaustive_dependent_pairs(
        ruler_sequence(),
        exhaustive_unsigneds(),
        UnsignedModPow2EqTriplesGenerator {
            phantom: PhantomData,
        },
    ))))
}

pub fn exhaustive_unsigned_triple_gen_var_9<T: PrimitiveUnsigned>() -> It<(T, T, u64)> {
    Box::new(
        exhaustive_triples_xxy_custom_output(
            exhaustive_unsigneds::<T>(),
            exhaustive_unsigneds(),
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::tiny(),
        )
        .filter(|&(x, y, pow)| !x.eq_mod_power_of_2(y, pow)),
    )
}

struct ModPowerOfTwoPairGenerator<T: PrimitiveUnsigned> {
    phantom: PhantomData<*const T>,
}

impl<T: PrimitiveUnsigned>
    ExhaustiveDependentPairsYsGenerator<
        u64,
        (T, T),
        ExhaustivePairs1Input<PrimitiveIntIncreasingRange<T>>,
    > for ModPowerOfTwoPairGenerator<T>
{
    #[inline]
    fn get_ys(&self, &pow: &u64) -> ExhaustivePairs1Input<PrimitiveIntIncreasingRange<T>> {
        exhaustive_pairs_from_single(primitive_int_increasing_inclusive_range(
            T::ZERO,
            T::low_mask(pow),
        ))
    }
}

pub fn exhaustive_unsigned_triple_gen_var_10<T: PrimitiveUnsigned>() -> It<(T, T, u64)> {
    reshape_2_1_to_3(permute_2_1(Box::new(exhaustive_dependent_pairs(
        ruler_sequence(),
        primitive_int_increasing_inclusive_range(0, T::WIDTH),
        ModPowerOfTwoPairGenerator {
            phantom: PhantomData,
        },
    ))))
}

pub fn exhaustive_unsigned_triple_gen_var_11<T: PrimitiveUnsigned>() -> It<(T, T, T)> {
    Box::new(
        exhaustive_triples_from_single(exhaustive_unsigneds::<T>())
            .flat_map(|(x, y, z)| Some((x, y, max(x, y).checked_add(z)?.checked_add(T::ONE)?))),
    )
}

pub fn exhaustive_unsigned_triple_gen_var_12<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
) -> It<(T, U, U)> {
    Box::new(exhaustive_triples_xyy_custom_output(
        exhaustive_unsigneds(),
        exhaustive_unsigneds(),
        BitDistributorOutputType::normal(1),
        BitDistributorOutputType::tiny(),
        BitDistributorOutputType::tiny(),
    ))
}

pub fn exhaustive_unsigned_triple_gen_var_13<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
) -> It<(T, U, T)> {
    Box::new(
        exhaustive_triples_xyx_custom_output(
            exhaustive_unsigneds(),
            exhaustive_unsigneds(),
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::tiny(),
            BitDistributorOutputType::normal(1),
        )
        .filter_map(|(x, y, z): (T, U, T)| Some((x, y, x.checked_add(z)?.checked_add(T::ONE)?))),
    )
}

pub fn exhaustive_unsigned_triple_gen_var_14<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
) -> It<(T, U, T)> {
    Box::new(
        exhaustive_triples_xyx(exhaustive_unsigneds(), exhaustive_unsigneds()).filter_map(
            |(x, y, z): (T, U, T)| Some((x, y, x.checked_add(z)?.checked_add(T::ONE)?)),
        ),
    )
}

struct ModPowerOfTwoPairWithExtraUnsignedGenerator<T: PrimitiveUnsigned, U: PrimitiveUnsigned> {
    phantom_t: PhantomData<*const T>,
    phantom_u: PhantomData<*const U>,
}

impl<T: PrimitiveUnsigned, U: PrimitiveUnsigned>
    ExhaustiveDependentPairsYsGenerator<
        u64,
        (T, U),
        ExhaustivePairs<T, PrimitiveIntIncreasingRange<T>, U, PrimitiveIntIncreasingRange<U>>,
    > for ModPowerOfTwoPairWithExtraUnsignedGenerator<T, U>
{
    #[inline]
    fn get_ys(
        &self,
        &pow: &u64,
    ) -> ExhaustivePairs<T, PrimitiveIntIncreasingRange<T>, U, PrimitiveIntIncreasingRange<U>> {
        exhaustive_pairs(
            primitive_int_increasing_inclusive_range(T::ZERO, T::low_mask(pow)),
            exhaustive_unsigneds(),
        )
    }
}

pub fn exhaustive_unsigned_triple_gen_var_15<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
) -> It<(T, U, u64)> {
    reshape_2_1_to_3(permute_2_1(Box::new(exhaustive_dependent_pairs(
        ruler_sequence(),
        primitive_int_increasing_inclusive_range(0, T::WIDTH),
        ModPowerOfTwoPairWithExtraUnsignedGenerator::<T, U> {
            phantom_t: PhantomData,
            phantom_u: PhantomData,
        },
    ))))
}

pub fn exhaustive_unsigned_triple_gen_var_16<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
) -> It<(T, U, T)> {
    Box::new(
        exhaustive_triples_xyx(
            exhaustive_unsigneds(),
            primitive_int_increasing_inclusive_range(U::ZERO, U::saturating_from(u64::MAX)),
        )
        .filter_map(|(x, y, z): (T, U, T)| Some((x, y, x.checked_add(z)?.checked_add(T::ONE)?))),
    )
}

// -- (PrimitiveUnsigned, PrimitiveUnsigned, PrimitiveUnsigned, PrimitiveUnsigned) --

pub fn exhaustive_unsigned_quadruple_gen<T: PrimitiveUnsigned>() -> It<(T, T, T, T)> {
    Box::new(exhaustive_quadruples_from_single(exhaustive_unsigneds()))
}

pub fn exhaustive_unsigned_quadruple_gen_var_1<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
) -> It<(T, u64, u64, U)> {
    Box::new(
        exhaustive_quadruples_xyyz_custom_output(
            exhaustive_unsigneds(),
            exhaustive_unsigneds(),
            exhaustive_unsigneds(),
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::tiny(),
            BitDistributorOutputType::tiny(),
            BitDistributorOutputType::normal(1),
        )
        .filter_map(|(x, y, z, w): (T, u64, u64, U)| {
            y.checked_add(z).and_then(|new_z| {
                if unsigned_assign_bits_valid(y, new_z, w) {
                    Some((x, y, new_z, w))
                } else {
                    None
                }
            })
        }),
    )
}

pub fn exhaustive_unsigned_quadruple_gen_var_2<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
) -> It<(T, T, T, U)> {
    Box::new(exhaustive_quadruples_xxxy_custom_output(
        exhaustive_unsigneds(),
        exhaustive_unsigneds(),
        BitDistributorOutputType::normal(1),
        BitDistributorOutputType::normal(1),
        BitDistributorOutputType::normal(1),
        BitDistributorOutputType::tiny(),
    ))
}

struct ModPowerOfTwoTripleGenerator<T: PrimitiveUnsigned> {
    phantom: PhantomData<*const T>,
}

impl<T: PrimitiveUnsigned>
    ExhaustiveDependentPairsYsGenerator<
        u64,
        (T, T, T),
        ExhaustiveTriples1Input<PrimitiveIntIncreasingRange<T>>,
    > for ModPowerOfTwoTripleGenerator<T>
{
    #[inline]
    fn get_ys(&self, &pow: &u64) -> ExhaustiveTriples1Input<PrimitiveIntIncreasingRange<T>> {
        exhaustive_triples_from_single(primitive_int_increasing_inclusive_range(
            T::ZERO,
            T::low_mask(pow),
        ))
    }
}

pub fn exhaustive_unsigned_quadruple_gen_var_3<T: PrimitiveUnsigned>() -> It<(T, T, T, u64)> {
    reshape_3_1_to_4(permute_2_1(Box::new(exhaustive_dependent_pairs(
        ruler_sequence(),
        primitive_int_increasing_inclusive_range(0, T::WIDTH),
        ModPowerOfTwoTripleGenerator {
            phantom: PhantomData,
        },
    ))))
}

pub fn exhaustive_unsigned_quadruple_gen_var_4<T: PrimitiveUnsigned>() -> It<(T, T, T, T)> {
    Box::new(
        exhaustive_quadruples_from_single(exhaustive_unsigneds::<T>()).flat_map(|(x, y, z, w)| {
            Some((x, y, z, max!(x, y, z).checked_add(w)?.checked_add(T::ONE)?))
        }),
    )
}

pub fn exhaustive_unsigned_quadruple_gen_var_5<
    T: CheckedFrom<DT> + PrimitiveUnsigned,
    DT: From<T> + HasHalf<Half = T> + JoinHalves + PrimitiveUnsigned + SplitInHalf,
>() -> It<(T, T, T, T)> {
    Box::new(
        exhaustive_triples_xxy(exhaustive_unsigneds(), exhaustive_positive_primitive_ints()).map(
            |(x_1, x_0, d)| {
                let inv = limbs_invert_limb_naive::<T, DT>(d << LeadingZeros::leading_zeros(d));
                (x_1, x_0, d, inv)
            },
        ),
    )
}

pub fn exhaustive_unsigned_quadruple_gen_var_6<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
) -> It<(T, T, U, T)> {
    Box::new(
        exhaustive_quadruples_xxyx(exhaustive_unsigneds::<T>(), exhaustive_unsigneds::<U>())
            .flat_map(|(x, y, z, w)| {
                Some((x, y, z, max(x, y).checked_add(w)?.checked_add(T::ONE)?))
            }),
    )
}

pub fn exhaustive_unsigned_quadruple_gen_var_7<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
) -> It<(T, U, U, T)> {
    Box::new(
        exhaustive_quadruples_xyyx(exhaustive_unsigneds::<T>(), exhaustive_unsigneds::<U>())
            .flat_map(|(x, y, z, w)| Some((x, y, z, x.checked_add(w)?.checked_add(T::ONE)?))),
    )
}

struct ModPowerOfTwoTripleWithExtraUnsignedGenerator<T: PrimitiveUnsigned, U: PrimitiveUnsigned> {
    phantom_t: PhantomData<*const T>,
    phantom_u: PhantomData<*const U>,
}

impl<T: PrimitiveUnsigned, U: PrimitiveUnsigned>
    ExhaustiveDependentPairsYsGenerator<
        u64,
        (T, T, U),
        ExhaustiveTriplesXXY<T, PrimitiveIntIncreasingRange<T>, U, PrimitiveIntIncreasingRange<U>>,
    > for ModPowerOfTwoTripleWithExtraUnsignedGenerator<T, U>
{
    #[inline]
    fn get_ys(
        &self,
        &pow: &u64,
    ) -> ExhaustiveTriplesXXY<T, PrimitiveIntIncreasingRange<T>, U, PrimitiveIntIncreasingRange<U>>
    {
        exhaustive_triples_xxy(
            primitive_int_increasing_inclusive_range(T::ZERO, T::low_mask(pow)),
            exhaustive_unsigneds(),
        )
    }
}

pub fn exhaustive_unsigned_quadruple_gen_var_8<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
) -> It<(T, T, U, u64)> {
    reshape_3_1_to_4(permute_2_1(Box::new(exhaustive_dependent_pairs(
        ruler_sequence(),
        primitive_int_increasing_inclusive_range(0, T::WIDTH),
        ModPowerOfTwoTripleWithExtraUnsignedGenerator::<T, U> {
            phantom_t: PhantomData,
            phantom_u: PhantomData,
        },
    ))))
}

struct ModPowerOfTwoQuadrupleWithTwoExtraUnsignedsGenerator<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned,
> {
    phantom_t: PhantomData<*const T>,
    phantom_u: PhantomData<*const U>,
}

impl<T: PrimitiveUnsigned, U: PrimitiveUnsigned>
    ExhaustiveDependentPairsYsGenerator<
        u64,
        (T, U, U),
        ExhaustiveTriplesXYY<T, PrimitiveIntIncreasingRange<T>, U, PrimitiveIntIncreasingRange<U>>,
    > for ModPowerOfTwoQuadrupleWithTwoExtraUnsignedsGenerator<T, U>
{
    #[inline]
    fn get_ys(
        &self,
        &pow: &u64,
    ) -> ExhaustiveTriplesXYY<T, PrimitiveIntIncreasingRange<T>, U, PrimitiveIntIncreasingRange<U>>
    {
        exhaustive_triples_xyy(
            primitive_int_increasing_inclusive_range(T::ZERO, T::low_mask(pow)),
            exhaustive_unsigneds(),
        )
    }
}

pub fn exhaustive_unsigned_quadruple_gen_var_9<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
) -> It<(T, U, U, u64)> {
    reshape_3_1_to_4(permute_2_1(Box::new(exhaustive_dependent_pairs(
        ruler_sequence(),
        primitive_int_increasing_inclusive_range(0, T::WIDTH),
        ModPowerOfTwoQuadrupleWithTwoExtraUnsignedsGenerator::<T, U> {
            phantom_t: PhantomData,
            phantom_u: PhantomData,
        },
    ))))
}

// -- (PrimitiveUnsigned, PrimitiveUnsigned, RoundingMode) --

pub fn exhaustive_unsigned_unsigned_rounding_mode_triple_gen_var_1<T: PrimitiveUnsigned>(
) -> It<(T, T, RoundingMode)> {
    reshape_2_1_to_3(Box::new(
        lex_pairs(
            exhaustive_pairs(
                exhaustive_unsigneds::<T>(),
                exhaustive_positive_primitive_ints::<T>(),
            ),
            exhaustive_rounding_modes(),
        )
        .filter(|&((x, y), rm)| rm != RoundingMode::Exact || x.divisible_by(y)),
    ))
}

pub(crate) fn round_to_multiple_unsigned_filter_map<T: PrimitiveUnsigned>(
    x: T,
    y: T,
    rm: RoundingMode,
) -> Option<(T, T, RoundingMode)> {
    if x == y {
        Some((x, y, rm))
    } else if y == T::ZERO {
        if rm == RoundingMode::Floor || rm == RoundingMode::Down || rm == RoundingMode::Nearest {
            Some((x, y, rm))
        } else {
            None
        }
    } else if rm != RoundingMode::Exact {
        x.div_round(y, rm).checked_mul(y).map(|_| (x, y, rm))
    } else {
        x.checked_mul(y).map(|product| (product, y, rm))
    }
}

pub(crate) fn round_to_multiple_of_power_of_2_filter_map<T: PrimitiveInt>(
    n: T,
    u: u64,
    rm: RoundingMode,
) -> Option<(T, u64, RoundingMode)> {
    if n == T::ZERO || rm != RoundingMode::Exact {
        n.shr_round(u, rm)
            .arithmetic_checked_shl(u)
            .map(|_| (n, u, rm))
    } else {
        n.arithmetic_checked_shl(u).map(|shifted| (shifted, u, rm))
    }
}

pub fn exhaustive_unsigned_unsigned_rounding_mode_triple_gen_var_3<T: PrimitiveUnsigned>(
) -> It<(T, T, RoundingMode)> {
    Box::new(
        lex_pairs(
            exhaustive_pairs(
                exhaustive_unsigneds::<T>(),
                exhaustive_positive_primitive_ints::<T>(),
            ),
            exhaustive_rounding_modes(),
        )
        .filter_map(|((x, y), rm)| round_to_multiple_unsigned_filter_map(x, y, rm)),
    )
}

pub fn exhaustive_unsigned_unsigned_rounding_mode_triple_gen_var_4<T: PrimitiveUnsigned>(
) -> It<(T, u64, RoundingMode)> {
    Box::new(
        lex_pairs(
            exhaustive_pairs_big_small(exhaustive_unsigneds::<T>(), exhaustive_unsigneds::<u64>()),
            exhaustive_rounding_modes(),
        )
        .filter_map(|((x, pow), rm)| round_to_multiple_of_power_of_2_filter_map(x, pow, rm)),
    )
}

// -- (PrimitiveUnsigned, PrimitiveUnsigned, Vec<bool>) --

struct UnsignedUnsignedBoolVecTripleGeneratorVar1;

impl<T: PrimitiveUnsigned>
    ExhaustiveDependentPairsYsGenerator<
        (T, u64),
        Vec<bool>,
        LexFixedLengthVecsFromSingle<ExhaustiveBools>,
    > for UnsignedUnsignedBoolVecTripleGeneratorVar1
{
    #[inline]
    fn get_ys(&self, &(x, log_base): &(T, u64)) -> LexFixedLengthVecsFromSingle<ExhaustiveBools> {
        lex_fixed_length_vecs_from_single(
            x.significant_bits()
                .div_round(log_base, RoundingMode::Ceiling),
            exhaustive_bools(),
        )
    }
}

pub fn exhaustive_unsigned_unsigned_bool_vec_triple_gen_var_1<
    T: PrimitiveUnsigned,
    U: PrimitiveInt,
>() -> It<(T, u64, Vec<bool>)> {
    reshape_2_1_to_3(Box::new(exhaustive_dependent_pairs(
        bit_distributor_sequence(
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::normal(1),
        ),
        lex_pairs(
            exhaustive_unsigneds(),
            primitive_int_increasing_inclusive_range(1, U::WIDTH),
        ),
        UnsignedUnsignedBoolVecTripleGeneratorVar1,
    )))
}

// -- (PrimitiveUnsigned, RoundingMode) --

pub fn exhaustive_unsigned_rounding_mode_pair_gen<T: PrimitiveUnsigned>() -> It<(T, RoundingMode)> {
    Box::new(lex_pairs(
        exhaustive_unsigneds(),
        exhaustive_rounding_modes(),
    ))
}

// -- (PrimitiveUnsigned, String) --

pub fn valid_digit_chars(base: u8) -> Vec<char> {
    let mut chars = Vec::new();
    if base <= 10 {
        chars.extend('0'..char::from(base + b'0'));
    } else {
        chars.extend('0'..='9');
        chars.extend('a'..char::from(base - 10 + b'a'));
        chars.extend('A'..char::from(base - 10 + b'A'));
    }
    chars
}

struct DigitStringGenerator;

impl
    ExhaustiveDependentPairsYsGenerator<
        u64,
        String,
        StringsFromCharVecs<ExhaustiveVecs<char, PrimitiveIntIncreasingRange<u64>, IntoIter<char>>>,
    > for DigitStringGenerator
{
    #[inline]
    fn get_ys(
        &self,
        &base: &u64,
    ) -> StringsFromCharVecs<ExhaustiveVecs<char, PrimitiveIntIncreasingRange<u64>, IntoIter<char>>>
    {
        assert!((2..=36).contains(&base));
        strings_from_char_vecs(exhaustive_vecs_min_length(
            1,
            valid_digit_chars(u8::wrapping_from(base)).into_iter(),
        ))
    }
}

pub fn exhaustive_unsigned_string_pair_gen_var_1() -> It<(u64, String)> {
    Box::new(exhaustive_dependent_pairs(
        bit_distributor_sequence(
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::normal(1),
        ),
        primitive_int_increasing_inclusive_range(2, 36),
        DigitStringGenerator,
    ))
}

pub fn exhaustive_unsigned_string_pair_gen_var_2() -> It<(u64, String)> {
    Box::new(exhaustive_pairs(
        primitive_int_increasing_inclusive_range(2, 36),
        exhaustive_strings(),
    ))
}

struct TargetedIntegerFromStringBaseInputs {
    neg: bool,
    u: u64,
    s: String,
    uss: It<(u64, String)>,
}

impl Iterator for TargetedIntegerFromStringBaseInputs {
    type Item = (u64, String);

    fn next(&mut self) -> Option<(u64, String)> {
        Some(if self.neg {
            self.neg = false;
            let next = self.uss.next().unwrap();
            self.u = next.0;
            self.s = next.1;
            (self.u, self.s.clone())
        } else {
            self.neg = true;
            let mut s = '-'.to_string();
            s.push_str(&self.s);
            (self.u, s)
        })
    }
}

pub fn exhaustive_unsigned_string_pair_gen_var_3() -> It<(u64, String)> {
    Box::new(TargetedIntegerFromStringBaseInputs {
        neg: true,
        u: 0,
        s: String::new(),
        uss: exhaustive_unsigned_string_pair_gen_var_1(),
    })
}

// -- (PrimitiveUnsigned, Vec<bool>) --

struct UnsignedBoolVecPairGeneratorVar1;

impl<T: PrimitiveUnsigned>
    ExhaustiveDependentPairsYsGenerator<T, Vec<bool>, LexFixedLengthVecsFromSingle<ExhaustiveBools>>
    for UnsignedBoolVecPairGeneratorVar1
{
    #[inline]
    fn get_ys(&self, &x: &T) -> LexFixedLengthVecsFromSingle<ExhaustiveBools> {
        lex_fixed_length_vecs_from_single(x.significant_bits(), exhaustive_bools())
    }
}

pub fn exhaustive_unsigned_bool_vec_pair_gen_var_1<T: PrimitiveUnsigned>() -> It<(T, Vec<bool>)> {
    Box::new(exhaustive_dependent_pairs(
        bit_distributor_sequence(
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::normal(1),
        ),
        exhaustive_unsigneds(),
        UnsignedBoolVecPairGeneratorVar1,
    ))
}

// -- RoundingMode --

pub fn exhaustive_rounding_mode_gen() -> It<RoundingMode> {
    Box::new(exhaustive_rounding_modes())
}

// -- (RoundingMode, RoundingMode) --

pub fn exhaustive_rounding_mode_pair_gen() -> It<(RoundingMode, RoundingMode)> {
    Box::new(lex_pairs_from_single(exhaustive_rounding_modes()))
}

// -- (RoundingMode, RoundingMode, RoundingMode) --

pub fn exhaustive_rounding_mode_triple_gen() -> It<(RoundingMode, RoundingMode, RoundingMode)> {
    Box::new(lex_triples_from_single(exhaustive_rounding_modes()))
}

// -- String --

pub fn exhaustive_string_gen() -> It<String> {
    Box::new(exhaustive_strings())
}

pub fn exhaustive_string_gen_var_1() -> It<String> {
    Box::new(exhaustive_strings_using_chars(exhaustive_ascii_chars()))
}

pub fn exhaustive_string_gen_var_2() -> It<String> {
    Box::new(exhaustive_strings_using_chars(ROUNDING_MODE_CHARS.chars()))
}

pub fn exhaustive_string_gen_var_3() -> It<String> {
    Box::new(strings_from_char_vecs(exhaustive_vecs_min_length(
        1,
        '0'..='9',
    )))
}

struct TargetedIntegerFromStrStringsVar1 {
    neg: bool,
    s: String,
    ss: It<String>,
}

impl Iterator for TargetedIntegerFromStrStringsVar1 {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        Some(if self.neg {
            self.neg = false;
            self.s = self.ss.next().unwrap();
            self.s.clone()
        } else {
            self.neg = true;
            format!("-{}", self.s)
        })
    }
}

pub fn exhaustive_string_gen_var_4() -> It<String> {
    Box::new(TargetedIntegerFromStrStringsVar1 {
        neg: true,
        s: String::new(),
        ss: exhaustive_string_gen_var_3(),
    })
}

pub fn exhaustive_string_gen_var_5() -> It<String> {
    Box::new(strings_from_char_vecs(exhaustive_vecs_min_length(
        1,
        '0'..='1',
    )))
}

pub fn exhaustive_string_gen_var_6() -> It<String> {
    Box::new(strings_from_char_vecs(exhaustive_vecs_min_length(
        1,
        '0'..='7',
    )))
}

pub fn exhaustive_string_gen_var_7() -> It<String> {
    Box::new(strings_from_char_vecs(exhaustive_vecs_min_length(
        1,
        lex_union3s('0'..='9', 'a'..='f', 'A'..='F').map(Union3::unwrap),
    )))
}

pub fn exhaustive_string_gen_var_8() -> It<String> {
    Box::new(
        strings_from_char_vecs(exhaustive_vecs_min_length(
            1,
            lex_union3s('0'..='9', 'a'..='f', 'A'..='F').map(Union3::unwrap),
        ))
        .map(|s| format!("\"0x{}\"", s)),
    )
}

struct TargetedIntegerFromStrStringsVar2 {
    neg: bool,
    s: String,
    ss: It<String>,
}

impl Iterator for TargetedIntegerFromStrStringsVar2 {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        Some(if self.neg {
            self.neg = false;
            self.s = self.ss.next().unwrap();
            format!("\"0x{}\"", self.s)
        } else {
            self.neg = true;
            format!("\"-0x{}\"", self.s)
        })
    }
}

pub fn exhaustive_string_gen_var_9() -> It<String> {
    Box::new(TargetedIntegerFromStrStringsVar2 {
        neg: true,
        s: String::new(),
        ss: exhaustive_string_gen_var_7(),
    })
}

pub fn exhaustive_string_gen_var_10() -> It<String> {
    Box::new(exhaustive_strings_using_chars(
        PRIMITIVE_FLOAT_CHARS.chars(),
    ))
}

// -- (String, String) --

pub fn exhaustive_string_pair_gen() -> It<(String, String)> {
    Box::new(exhaustive_pairs_from_single(exhaustive_strings()))
}

pub fn exhaustive_string_pair_gen_var_1() -> It<(String, String)> {
    Box::new(exhaustive_pairs_from_single(
        exhaustive_strings_using_chars(exhaustive_ascii_chars()),
    ))
}

// -- Vec<bool> --

pub fn exhaustive_bool_vec_gen_var_1<T: PrimitiveUnsigned>() -> It<Vec<bool>> {
    Box::new(
        shortlex_vecs_length_inclusive_range(0, T::WIDTH, exhaustive_bools()).interleave(
            exhaustive_pairs_big_tiny(
                lex_fixed_length_vecs_from_single(T::WIDTH, exhaustive_bools()),
                exhaustive_positive_primitive_ints(),
            )
            .map(|(bs, n)| bs.into_iter().chain(repeat_n(false, n)).collect()),
        ),
    )
}

pub fn exhaustive_bool_vec_gen_var_2<T: PrimitiveSigned>() -> It<Vec<bool>> {
    Box::new(
        shortlex_vecs_length_inclusive_range(0, T::WIDTH - 1, exhaustive_bools()).interleave(
            exhaustive_pairs_big_tiny(
                lex_fixed_length_vecs_from_single(T::WIDTH - 1, exhaustive_bools()),
                exhaustive_nonzero_signeds::<isize>(),
            )
            .map(|(bs, n)| {
                bs.into_iter()
                    .chain(repeat_n(n < 0, n.unsigned_abs()))
                    .collect()
            }),
        ),
    )
}

pub fn exhaustive_bool_vec_gen_var_3<T: PrimitiveUnsigned>() -> It<Vec<bool>> {
    Box::new(
        shortlex_vecs_length_inclusive_range(0, T::WIDTH, exhaustive_bools()).interleave(
            exhaustive_pairs_big_tiny(
                lex_fixed_length_vecs_from_single(T::WIDTH, exhaustive_bools()),
                exhaustive_positive_primitive_ints(),
            )
            .map(|(bs, n)| repeat_n(false, n).chain(bs.into_iter()).collect()),
        ),
    )
}

pub fn exhaustive_bool_vec_gen_var_4<T: PrimitiveSigned>() -> It<Vec<bool>> {
    Box::new(
        shortlex_vecs_length_inclusive_range(0, T::WIDTH - 1, exhaustive_bools()).interleave(
            exhaustive_pairs_big_tiny(
                lex_fixed_length_vecs_from_single(T::WIDTH - 1, exhaustive_bools()),
                exhaustive_nonzero_signeds::<isize>(),
            )
            .map(|(bs, n)| {
                repeat_n(n < 0, n.unsigned_abs())
                    .chain(bs.into_iter())
                    .collect()
            }),
        ),
    )
}

// -- Vec<PrimitiveUnsigned> --

pub fn exhaustive_unsigned_vec_gen<T: PrimitiveUnsigned>() -> It<Vec<T>> {
    Box::new(exhaustive_vecs(exhaustive_unsigneds()))
}

pub fn exhaustive_unsigned_vec_gen_var_1<T: PrimitiveUnsigned>() -> It<Vec<T>> {
    Box::new(
        exhaustive_vecs_min_length(1, exhaustive_unsigneds())
            .filter(|xs| *xs.last().unwrap() != T::ZERO),
    )
}

// --(Vec<PrimitiveUnsigned>, PrimitiveInt) --

pub fn exhaustive_unsigned_vec_primitive_int_pair_gen_var_1<
    T: PrimitiveUnsigned,
    U: PrimitiveInt,
>() -> It<(Vec<T>, U)> {
    Box::new(exhaustive_pairs_big_small(
        exhaustive_vecs_min_length(1, exhaustive_unsigneds())
            .filter(|xs| *xs.last().unwrap() != T::ZERO),
        exhaustive_positive_primitive_ints(),
    ))
}

// --(Vec<PrimitiveUnsigned>, PrimitiveUnsigned) --

pub fn exhaustive_unsigned_vec_unsigned_pair_gen<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
) -> It<(Vec<T>, U)> {
    Box::new(exhaustive_pairs(
        exhaustive_vecs(exhaustive_unsigneds()),
        exhaustive_unsigneds(),
    ))
}

struct UnsignedVecUnsignedPairGeneratorVar1;

impl<T: PrimitiveUnsigned>
    ExhaustiveDependentPairsYsGenerator<
        (usize, usize),
        Vec<T>,
        ExhaustiveFixedLengthVecs1Input<PrimitiveIntIncreasingRange<T>>,
    > for UnsignedVecUnsignedPairGeneratorVar1
{
    #[inline]
    fn get_ys(
        &self,
        &p: &(usize, usize),
    ) -> ExhaustiveFixedLengthVecs1Input<PrimitiveIntIncreasingRange<T>> {
        exhaustive_fixed_length_vecs_from_single(u64::exact_from(p.1), exhaustive_unsigneds())
    }
}

//TODO generate (usize, usize) pairs better
pub fn exhaustive_unsigned_vec_unsigned_pair_gen_var_1<T: PrimitiveUnsigned>() -> T1<Vec<T>, usize>
{
    Box::new(
        exhaustive_dependent_pairs(
            bit_distributor_sequence(
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
            ),
            exhaustive_pairs_from_single(exhaustive_unsigneds()).filter(|(x, y)| x <= y),
            UnsignedVecUnsignedPairGeneratorVar1,
        )
        .map(|((x, _), zs)| (zs, x)),
    )
}

struct UnsignedVecUnsignedPairGeneratorVar2<T: PrimitiveUnsigned, U: PrimitiveUnsigned> {
    phantom_t: PhantomData<*const T>,
    phantom_u: PhantomData<*const U>,
}

impl<T: PrimitiveUnsigned, U: PrimitiveUnsigned>
    ExhaustiveDependentPairsYsGenerator<u64, Vec<U>, It<Vec<U>>>
    for UnsignedVecUnsignedPairGeneratorVar2<T, U>
{
    #[inline]
    fn get_ys(&self, &log_base: &u64) -> It<Vec<U>> {
        Box::new(
            exhaustive_vecs_length_inclusive_range(
                0,
                T::WIDTH.div_round(log_base, RoundingMode::Ceiling),
                primitive_int_increasing_inclusive_range(
                    U::ZERO,
                    U::low_mask(min(T::WIDTH, log_base)),
                ),
            )
            .filter(move |xs| digits_valid::<T, U>(log_base, xs)),
        )
    }
}

pub fn exhaustive_unsigned_vec_unsigned_pair_gen_var_2<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>() -> It<(Vec<U>, u64)> {
    permute_2_1(Box::new(exhaustive_dependent_pairs(
        ruler_sequence(),
        primitive_int_increasing_inclusive_range(1, U::WIDTH),
        UnsignedVecUnsignedPairGeneratorVar2::<T, U> {
            phantom_t: PhantomData,
            phantom_u: PhantomData,
        },
    )))
}

pub fn exhaustive_unsigned_vec_unsigned_pair_gen_var_3<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>() -> It<(Vec<U>, u64)> {
    Box::new(
        exhaustive_unsigned_vec_unsigned_pair_gen_var_2::<T, U>()
            .map(|(xs, y)| (xs.into_iter().rev().collect(), y)),
    )
}

// var 4 is in malachite-nz

struct ValidDigitsGenerator<T: PrimitiveUnsigned, U: PrimitiveUnsigned> {
    phantom_t: PhantomData<*const T>,
    phantom_u: PhantomData<*const U>,
}

impl<T: PrimitiveUnsigned + WrappingFrom<U>, U: PrimitiveUnsigned>
    ExhaustiveDependentPairsYsGenerator<U, Vec<T>, It<Vec<T>>> for ValidDigitsGenerator<T, U>
{
    #[inline]
    fn get_ys(&self, base: &U) -> It<Vec<T>> {
        Box::new(exhaustive_vecs(primitive_int_increasing_range(
            T::ZERO,
            T::wrapping_from(*base),
        )))
    }
}

pub fn exhaustive_unsigned_vec_unsigned_pair_gen_var_5<
    T: PrimitiveUnsigned + WrappingFrom<U>,
    U: PrimitiveUnsigned + SaturatingFrom<T>,
>() -> It<(Vec<T>, U)> {
    permute_2_1(Box::new(exhaustive_dependent_pairs(
        bit_distributor_sequence(
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::normal(1),
        ),
        primitive_int_increasing_inclusive_range(U::TWO, U::saturating_from(T::MAX)),
        ValidDigitsGenerator {
            phantom_t: PhantomData,
            phantom_u: PhantomData,
        },
    )))
}

pub fn exhaustive_unsigned_vec_unsigned_pair_gen_var_6<T: PrimitiveUnsigned>() -> It<(Vec<T>, u64)>
{
    Box::new(exhaustive_pairs(
        exhaustive_vecs(exhaustive_unsigneds()),
        primitive_int_increasing_inclusive_range(1, T::WIDTH),
    ))
}

struct DigitsDesc<T: PrimitiveUnsigned> {
    max_digits: Vec<T>,
    ds: ShortlexVecs<T, PrimitiveIntIncreasingRange<u64>, PrimitiveIntIncreasingRange<T>>,
}

impl<T: PrimitiveUnsigned> Iterator for DigitsDesc<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Vec<T>> {
        loop {
            let digits = self.ds.next()?;
            if digits.len() < self.max_digits.len() || digits <= self.max_digits {
                return Some(digits);
            }
        }
    }
}

struct DigitsDescGenerator<T: PrimitiveUnsigned, U: Digits<T> + PrimitiveUnsigned> {
    phantom_t: PhantomData<*const T>,
    phantom_u: PhantomData<*const U>,
}

impl<T: PrimitiveUnsigned, U: Digits<T> + PrimitiveUnsigned>
    ExhaustiveDependentPairsYsGenerator<T, Vec<T>, DigitsDesc<T>> for DigitsDescGenerator<T, U>
{
    #[inline]
    fn get_ys(&self, base: &T) -> DigitsDesc<T> {
        let max_digits = U::MAX.to_digits_desc(base);
        DigitsDesc {
            ds: shortlex_vecs_length_inclusive_range(
                0,
                u64::exact_from(max_digits.len()),
                primitive_int_increasing_range(T::ZERO, *base),
            ),
            max_digits,
        }
    }
}

pub fn exhaustive_unsigned_vec_unsigned_pair_gen_var_7<
    T: PrimitiveUnsigned + SaturatingFrom<U>,
    U: Digits<T> + PrimitiveUnsigned,
>() -> It<(Vec<T>, T)> {
    permute_2_1(Box::new(exhaustive_dependent_pairs(
        ruler_sequence(),
        primitive_int_increasing_inclusive_range(T::TWO, T::saturating_from(U::MAX)),
        DigitsDescGenerator::<T, U> {
            phantom_t: PhantomData,
            phantom_u: PhantomData,
        },
    )))
}

pub fn exhaustive_unsigned_vec_unsigned_pair_gen_var_8<
    T: PrimitiveUnsigned + SaturatingFrom<U>,
    U: Digits<T> + PrimitiveUnsigned,
>() -> It<(Vec<T>, T)> {
    Box::new(
        exhaustive_unsigned_vec_unsigned_pair_gen_var_7::<T, U>().map(|(mut xs, base)| {
            xs.reverse();
            (xs, base)
        }),
    )
}

pub fn exhaustive_unsigned_vec_unsigned_pair_gen_var_9<T: PrimitiveUnsigned>() -> It<(Vec<T>, T)> {
    Box::new(exhaustive_pairs(
        exhaustive_vecs(exhaustive_unsigneds()),
        primitive_int_increasing_inclusive_range(T::TWO, T::MAX),
    ))
}

struct PowerOfTwoDigitsGenerator;

impl<T: PrimitiveUnsigned>
    ExhaustiveDependentPairsYsGenerator<
        u64,
        Vec<T>,
        ExhaustiveVecs<T, PrimitiveIntIncreasingRange<u64>, PrimitiveIntIncreasingRange<T>>,
    > for PowerOfTwoDigitsGenerator
{
    #[inline]
    fn get_ys(
        &self,
        &log_base: &u64,
    ) -> ExhaustiveVecs<T, PrimitiveIntIncreasingRange<u64>, PrimitiveIntIncreasingRange<T>> {
        exhaustive_vecs(primitive_int_increasing_inclusive_range(
            T::ZERO,
            T::low_mask(log_base),
        ))
    }
}

pub fn exhaustive_unsigned_vec_unsigned_pair_gen_var_10<T: PrimitiveUnsigned>() -> It<(Vec<T>, u64)>
{
    permute_2_1(Box::new(exhaustive_dependent_pairs(
        bit_distributor_sequence(
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::normal(1),
        ),
        primitive_int_increasing_inclusive_range(1, T::WIDTH),
        PowerOfTwoDigitsGenerator,
    )))
}

pub fn exhaustive_unsigned_vec_unsigned_pair_gen_var_11<T: PrimitiveUnsigned>() -> It<(Vec<T>, u64)>
{
    Box::new(exhaustive_pairs(
        exhaustive_vecs(exhaustive_unsigneds()),
        primitive_int_increasing_inclusive_range(1, T::WIDTH),
    ))
}

pub fn exhaustive_unsigned_vec_unsigned_pair_gen_var_12<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned + SaturatingFrom<T>,
>() -> It<(Vec<T>, U)> {
    Box::new(exhaustive_pairs(
        exhaustive_vecs(exhaustive_unsigneds()),
        primitive_int_increasing_inclusive_range(U::TWO, U::saturating_from(T::MAX)),
    ))
}

// --(Vec<PrimitiveUnsigned>, PrimitiveUnsigned, PrimitiveUnsigned) --

pub fn exhaustive_unsigned_vec_unsigned_unsigned_triple_gen<T: PrimitiveUnsigned>(
) -> It<(Vec<T>, T, T)> {
    Box::new(exhaustive_triples_xyy(
        exhaustive_vecs(exhaustive_unsigneds()),
        exhaustive_unsigneds(),
    ))
}

pub fn exhaustive_unsigned_vec_unsigned_unsigned_triple_gen_var_1<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned,
    V: PrimitiveUnsigned,
>() -> It<(Vec<T>, U, V)> {
    Box::new(exhaustive_triples_custom_output(
        exhaustive_vecs(exhaustive_unsigneds()),
        exhaustive_unsigneds(),
        exhaustive_unsigneds(),
        BitDistributorOutputType::normal(1),
        BitDistributorOutputType::tiny(),
        BitDistributorOutputType::normal(1),
    ))
}

struct UnsignedVecUnsignedUnsignedTripleGeneratorVar2;

impl<T: PrimitiveUnsigned>
    ExhaustiveDependentPairsYsGenerator<
        (usize, usize),
        Vec<T>,
        ExhaustiveVecs<T, PrimitiveIntIncreasingRange<u64>, PrimitiveIntIncreasingRange<T>>,
    > for UnsignedVecUnsignedUnsignedTripleGeneratorVar2
{
    #[inline]
    fn get_ys(
        &self,
        &(i, j): &(usize, usize),
    ) -> ExhaustiveVecs<T, PrimitiveIntIncreasingRange<u64>, PrimitiveIntIncreasingRange<T>> {
        exhaustive_vecs_min_length(u64::exact_from(i * j), exhaustive_unsigneds())
    }
}

pub fn exhaustive_unsigned_vec_unsigned_unsigned_triple_gen_var_2<T: PrimitiveUnsigned>(
) -> It<(Vec<T>, usize, usize)> {
    reshape_1_2_to_3(permute_2_1(Box::new(exhaustive_dependent_pairs(
        bit_distributor_sequence(
            BitDistributorOutputType::tiny(),
            BitDistributorOutputType::normal(1),
        ),
        exhaustive_pairs_from_single(exhaustive_unsigneds()),
        UnsignedVecUnsignedUnsignedTripleGeneratorVar2,
    ))))
}

// --(Vec<PrimitiveUnsigned>, Vec<PrimitiveUnsigned>) --

struct UnsignedVecPairLenGenerator;

impl<T: PrimitiveUnsigned>
    ExhaustiveDependentPairsYsGenerator<
        (u64, u64),
        (Vec<T>, Vec<T>),
        ExhaustivePairs<
            Vec<T>,
            ExhaustiveFixedLengthVecs1Input<PrimitiveIntIncreasingRange<T>>,
            Vec<T>,
            ExhaustiveFixedLengthVecs1Input<PrimitiveIntIncreasingRange<T>>,
        >,
    > for UnsignedVecPairLenGenerator
{
    #[allow(clippy::type_complexity)]
    #[inline]
    fn get_ys(
        &self,
        &(i, j): &(u64, u64),
    ) -> ExhaustivePairs<
        Vec<T>,
        ExhaustiveFixedLengthVecs1Input<PrimitiveIntIncreasingRange<T>>,
        Vec<T>,
        ExhaustiveFixedLengthVecs1Input<PrimitiveIntIncreasingRange<T>>,
    > {
        exhaustive_pairs(
            exhaustive_fixed_length_vecs_from_single(i, exhaustive_unsigneds()),
            exhaustive_fixed_length_vecs_from_single(j, exhaustive_unsigneds()),
        )
    }
}

pub fn exhaustive_unsigned_vec_pair_gen_var_1<T: PrimitiveUnsigned>() -> It<(Vec<T>, Vec<T>)> {
    Box::new(
        exhaustive_dependent_pairs(
            bit_distributor_sequence(
                BitDistributorOutputType::tiny(),
                BitDistributorOutputType::normal(1),
            ),
            //TODO
            exhaustive_pairs_from_single(exhaustive_unsigneds()).filter(|(x, y)| x >= y),
            UnsignedVecPairLenGenerator,
        )
        .map(|p| p.1),
    )
}

pub fn exhaustive_unsigned_vec_pair_gen_var_2<T: PrimitiveUnsigned>() -> It<(Vec<T>, Vec<T>)> {
    Box::new(
        exhaustive_dependent_pairs(
            bit_distributor_sequence(
                BitDistributorOutputType::tiny(),
                BitDistributorOutputType::normal(1),
            ),
            //TODO
            exhaustive_pairs_from_single(exhaustive_positive_primitive_ints())
                .filter(|(x, y)| x >= y),
            UnsignedVecPairLenGenerator,
        )
        .map(|p| p.1),
    )
}

pub fn exhaustive_unsigned_vec_pair_gen_var_3<T: PrimitiveUnsigned>() -> It<(Vec<T>, Vec<T>)> {
    Box::new(exhaustive_pairs_from_single(exhaustive_vecs_min_length(
        1,
        exhaustive_unsigneds(),
    )))
}

pub fn exhaustive_unsigned_vec_pair_gen_var_4<T: PrimitiveUnsigned>() -> It<(Vec<T>, Vec<T>)> {
    Box::new(
        exhaustive_pairs_from_single(exhaustive_vecs_min_length(1, exhaustive_unsigneds())).filter(
            |&(ref xs, ref es)| {
                !xs.is_empty()
                    && (es.len() > 1 || es.len() == 1 && es[0] > T::ONE)
                    && *es.last().unwrap() != T::ZERO
            },
        ),
    )
}

// --(Vec<PrimitiveUnsigned>, Vec<PrimitiveUnsigned>, PrimitiveUnsigned) --

pub fn exhaustive_unsigned_vec_unsigned_vec_unsigned_triple_gen_var_1<T: PrimitiveUnsigned>(
) -> It<(Vec<T>, Vec<T>, T)> {
    reshape_2_1_to_3(Box::new(exhaustive_pairs(
        exhaustive_dependent_pairs(
            bit_distributor_sequence(
                BitDistributorOutputType::tiny(),
                BitDistributorOutputType::normal(1),
            ),
            //TODO
            exhaustive_pairs_from_single(exhaustive_unsigneds()).filter(|(x, y)| x >= y),
            UnsignedVecPairLenGenerator,
        )
        .map(|p| p.1),
        exhaustive_unsigneds(),
    )))
}

// var 2 is in malachite-nz

// --(Vec<PrimitiveUnsigned>, Vec<PrimitiveUnsigned>, Vec<PrimitiveUnsigned>) --

pub struct UnsignedVecTripleXYYLenGenerator;

impl<T: PrimitiveUnsigned>
    ExhaustiveDependentPairsYsGenerator<
        (u64, u64),
        (Vec<T>, Vec<T>, Vec<T>),
        ExhaustiveTriplesXYY<
            Vec<T>,
            ExhaustiveFixedLengthVecs1Input<PrimitiveIntIncreasingRange<T>>,
            Vec<T>,
            ExhaustiveFixedLengthVecs1Input<PrimitiveIntIncreasingRange<T>>,
        >,
    > for UnsignedVecTripleXYYLenGenerator
{
    #[allow(clippy::type_complexity)]
    #[inline]
    fn get_ys(
        &self,
        &(i, j): &(u64, u64),
    ) -> ExhaustiveTriplesXYY<
        Vec<T>,
        ExhaustiveFixedLengthVecs1Input<PrimitiveIntIncreasingRange<T>>,
        Vec<T>,
        ExhaustiveFixedLengthVecs1Input<PrimitiveIntIncreasingRange<T>>,
    > {
        exhaustive_triples_xyy(
            exhaustive_fixed_length_vecs_from_single(i, exhaustive_unsigneds()),
            exhaustive_fixed_length_vecs_from_single(j, exhaustive_unsigneds()),
        )
    }
}

pub fn exhaustive_unsigned_vec_triple_gen_var_1<T: PrimitiveUnsigned>(
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(
        exhaustive_dependent_pairs(
            bit_distributor_sequence(
                BitDistributorOutputType::tiny(),
                BitDistributorOutputType::normal(1),
            ),
            exhaustive_pairs_from_single(exhaustive_unsigneds::<u64>()).flat_map(|(x, y)| {
                let y = y.checked_add(1)?;
                let x = x.checked_add(y.arithmetic_checked_shl(1)?)?;
                Some((x, y))
            }),
            UnsignedVecTripleXYYLenGenerator,
        )
        .map(|p| p.1),
    )
}

pub struct UnsignedVecTripleLenGenerator;

impl<T: PrimitiveUnsigned>
    ExhaustiveDependentPairsYsGenerator<
        (u64, u64, u64),
        (Vec<T>, Vec<T>, Vec<T>),
        ExhaustiveTriples<
            Vec<T>,
            ExhaustiveFixedLengthVecs1Input<PrimitiveIntIncreasingRange<T>>,
            Vec<T>,
            ExhaustiveFixedLengthVecs1Input<PrimitiveIntIncreasingRange<T>>,
            Vec<T>,
            ExhaustiveFixedLengthVecs1Input<PrimitiveIntIncreasingRange<T>>,
        >,
    > for UnsignedVecTripleLenGenerator
{
    #[allow(clippy::type_complexity)]
    #[inline]
    fn get_ys(
        &self,
        &(i, j, k): &(u64, u64, u64),
    ) -> ExhaustiveTriples<
        Vec<T>,
        ExhaustiveFixedLengthVecs1Input<PrimitiveIntIncreasingRange<T>>,
        Vec<T>,
        ExhaustiveFixedLengthVecs1Input<PrimitiveIntIncreasingRange<T>>,
        Vec<T>,
        ExhaustiveFixedLengthVecs1Input<PrimitiveIntIncreasingRange<T>>,
    > {
        exhaustive_triples(
            exhaustive_fixed_length_vecs_from_single(i, exhaustive_unsigneds()),
            exhaustive_fixed_length_vecs_from_single(j, exhaustive_unsigneds()),
            exhaustive_fixed_length_vecs_from_single(k, exhaustive_unsigneds()),
        )
    }
}

pub fn exhaustive_unsigned_vec_triple_gen_var_2<T: PrimitiveUnsigned>(
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(
        exhaustive_dependent_pairs(
            bit_distributor_sequence(
                BitDistributorOutputType::tiny(),
                BitDistributorOutputType::normal(1),
            ),
            exhaustive_triples_from_single(exhaustive_unsigneds::<u64>()).flat_map(|(o, x, y)| {
                let y = y.checked_add(1)?;
                let x = x.checked_add(y)?;
                let o = x.checked_add(y)?.checked_add(o)?;
                Some((o, x, y))
            }),
            UnsignedVecTripleLenGenerator,
        )
        .map(|p| p.1),
    )
}

pub fn exhaustive_unsigned_vec_triple_gen_var_3<T: PrimitiveUnsigned>(
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(
        exhaustive_dependent_pairs(
            bit_distributor_sequence(
                BitDistributorOutputType::tiny(),
                BitDistributorOutputType::normal(1),
            ),
            exhaustive_triples_from_single(exhaustive_unsigneds::<u64>()).flat_map(|(o, x, y)| {
                let y = y.checked_add(1)?;
                let x = x.checked_add(1)?;
                let o = x.checked_add(y)?.checked_add(o)?;
                Some((o, x, y))
            }),
            UnsignedVecTripleLenGenerator,
        )
        .map(|p| p.1),
    )
}

// vars 4 through 23 are in malachite-nz

pub fn exhaustive_unsigned_vec_triple_gen_var_24<T: PrimitiveUnsigned>(
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(
        exhaustive_dependent_pairs(
            bit_distributor_sequence(
                BitDistributorOutputType::tiny(),
                BitDistributorOutputType::normal(1),
            ),
            exhaustive_pairs_from_single(exhaustive_unsigneds::<u64>()).flat_map(|(x, y)| {
                let y = y.checked_add(1)?;
                let x = x.checked_add(y)?;
                Some((x, y))
            }),
            UnsignedVecTripleXYYLenGenerator,
        )
        .map(|p| p.1),
    )
}

pub fn exhaustive_unsigned_vec_triple_gen_var_25<T: PrimitiveUnsigned>(
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(
        exhaustive_dependent_pairs(
            bit_distributor_sequence(
                BitDistributorOutputType::tiny(),
                BitDistributorOutputType::normal(1),
            ),
            exhaustive_pairs_from_single(exhaustive_unsigneds::<u64>()).flat_map(|(x, y)| {
                let y = y.checked_add(2)?;
                let x = x.checked_add(y.arithmetic_checked_shl(1)?)?;
                Some((x, y))
            }),
            UnsignedVecTripleXYYLenGenerator,
        )
        .map(|p| p.1),
    )
}

pub fn exhaustive_unsigned_vec_triple_gen_var_26<T: PrimitiveUnsigned>(
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(
        exhaustive_dependent_pairs(
            bit_distributor_sequence(
                BitDistributorOutputType::tiny(),
                BitDistributorOutputType::normal(1),
            ),
            exhaustive_pairs_from_single(exhaustive_unsigneds::<u64>()).flat_map(|(x, y)| {
                let y = y.checked_add(2)?;
                let x = x.checked_add(y)?;
                Some((x, y))
            }),
            UnsignedVecTripleXYYLenGenerator,
        )
        .map(|p| p.1),
    )
}

struct UnsignedVecTripleXXXLenGenerator;

impl<T: PrimitiveUnsigned>
    ExhaustiveDependentPairsYsGenerator<
        u64,
        (Vec<T>, Vec<T>, Vec<T>),
        ExhaustiveTriples1Input<ExhaustiveFixedLengthVecs1Input<PrimitiveIntIncreasingRange<T>>>,
    > for UnsignedVecTripleXXXLenGenerator
{
    #[inline]
    fn get_ys(
        &self,
        &i: &u64,
    ) -> ExhaustiveTriples1Input<ExhaustiveFixedLengthVecs1Input<PrimitiveIntIncreasingRange<T>>>
    {
        exhaustive_triples_from_single(exhaustive_fixed_length_vecs_from_single(
            i,
            exhaustive_unsigneds(),
        ))
    }
}

pub fn exhaustive_unsigned_vec_triple_gen_var_27<T: PrimitiveUnsigned>(
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(
        exhaustive_dependent_pairs(
            bit_distributor_sequence(
                BitDistributorOutputType::tiny(),
                BitDistributorOutputType::normal(1),
            ),
            primitive_int_increasing_inclusive_range::<u64>(2, u64::MAX),
            UnsignedVecTripleXXXLenGenerator,
        )
        .map(|p| p.1),
    )
}

// -- large types --

pub fn exhaustive_large_type_gen_var_1<T: PrimitiveUnsigned>() -> It<(Vec<T>, Vec<T>, T, T)> {
    reshape_2_2_to_4(Box::new(exhaustive_pairs(
        exhaustive_dependent_pairs(
            bit_distributor_sequence(
                BitDistributorOutputType::tiny(),
                BitDistributorOutputType::normal(1),
            ),
            //TODO
            exhaustive_pairs_from_single(exhaustive_unsigneds()).filter(|(x, y)| x >= y),
            UnsignedVecPairLenGenerator,
        )
        .map(|p| p.1),
        exhaustive_pairs_from_single(exhaustive_unsigneds()),
    )))
}
