use common::GenerationMode;
use inputs::common::{permute_1_3_2, permute_2_1, reshape_2_1_to_3};
use malachite_base::chars::NUMBER_OF_CHARS;
use malachite_base::limbs::limbs_test_zero;
use malachite_base::num::{PrimitiveInteger, PrimitiveSigned, PrimitiveUnsigned};
use malachite_base::round::RoundingMode;
use rust_wheels::iterators::bools::exhaustive_bools;
use rust_wheels::iterators::chars::exhaustive_chars;
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::general::{random_x, range_increasing_x};
use rust_wheels::iterators::integers_geometric::natural_u32s_geometric;
use rust_wheels::iterators::orderings::{exhaustive_orderings, random_orderings};
use rust_wheels::iterators::primitive_ints::{exhaustive_i, exhaustive_negative_i,
                                             exhaustive_positive_x, exhaustive_u,
                                             random_negative_i, random_positive_i,
                                             random_positive_u, random_range};
use rust_wheels::iterators::rounding_modes::{exhaustive_rounding_modes, random_rounding_modes};
use rust_wheels::iterators::tuples::{exhaustive_pairs, exhaustive_pairs_from_single, lex_pairs,
                                     lex_triples, log_pairs, random_pairs,
                                     random_pairs_from_single, random_triples,
                                     random_triples_from_single, sqrt_pairs};
use rust_wheels::iterators::vecs::{exhaustive_vecs, random_vecs};
use std::char;
use std::cmp::Ordering;

type It<T> = Box<Iterator<Item = T>>;

pub fn unsigneds<T: 'static + PrimitiveUnsigned>(gm: GenerationMode) -> It<T> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_u()),
        GenerationMode::Random(_) => Box::new(random_x(&EXAMPLE_SEED)),
    }
}

pub fn signeds<T: 'static + PrimitiveSigned>(gm: GenerationMode) -> It<T> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_i()),
        GenerationMode::Random(_) => Box::new(random_x(&EXAMPLE_SEED)),
    }
}

pub fn positive_unsigneds<T: 'static + PrimitiveUnsigned>(gm: GenerationMode) -> It<T> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_positive_x()),
        GenerationMode::Random(_) => Box::new(random_positive_u(&EXAMPLE_SEED)),
    }
}

pub fn unsigneds_no_max<T: 'static + PrimitiveUnsigned>(gm: GenerationMode) -> It<T> {
    Box::new(unsigneds(gm).filter(|&u| u != T::MAX))
}

pub fn signeds_no_max<T: 'static + PrimitiveSigned>(gm: GenerationMode) -> It<T> {
    Box::new(signeds(gm).filter(|&i| i != T::MAX))
}

pub fn signeds_no_min<T: 'static + PrimitiveSigned>(gm: GenerationMode) -> It<T> {
    Box::new(signeds(gm).filter(|&i| i != T::MIN))
}

pub fn pairs_of_unsigneds<T: 'static + PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (T, T)>> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_pairs_from_single(exhaustive_u())),
        GenerationMode::Random(_) => Box::new(random_pairs_from_single(random_x(&EXAMPLE_SEED))),
    }
}

pub fn u32s_range_1(gm: GenerationMode) -> It<u32> {
    match gm {
        GenerationMode::Exhaustive => Box::new(range_increasing_x(0, NUMBER_OF_CHARS - 1)),
        GenerationMode::Random(_) => Box::new(random_range(&EXAMPLE_SEED, 0, NUMBER_OF_CHARS - 1)),
    }
}

pub fn pairs_of_u32s_range_1(gm: GenerationMode) -> Box<Iterator<Item = (u32, u32)>> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_pairs_from_single(range_increasing_x(
            0,
            NUMBER_OF_CHARS - 1,
        ))),
        GenerationMode::Random(_) => Box::new(random_pairs_from_single(random_range(
            &EXAMPLE_SEED,
            0,
            NUMBER_OF_CHARS - 1,
        ))),
    }
}

pub fn small_u32s(gm: GenerationMode) -> Box<Iterator<Item = u32>> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_u()),
        GenerationMode::Random(scale) => Box::new(natural_u32s_geometric(&EXAMPLE_SEED, scale)),
    }
}

pub fn small_u64s(gm: GenerationMode) -> Box<Iterator<Item = u64>> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_u()),
        GenerationMode::Random(scale) => {
            Box::new(natural_u32s_geometric(&EXAMPLE_SEED, scale).map(|i| i.into()))
        }
    }
}

fn sqrt_pairs_of_unsigneds<T: 'static + PrimitiveUnsigned, U: 'static + PrimitiveUnsigned>(
) -> Box<Iterator<Item = (T, U)>> {
    Box::new(sqrt_pairs(exhaustive_u(), exhaustive_u()))
}

fn random_pairs_of_primitive_and_geometric_u32<T: 'static + PrimitiveInteger>(
    scale: u32,
) -> Box<Iterator<Item = (T, u32)>> {
    Box::new(random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_x(seed)),
        &(|seed| natural_u32s_geometric(seed, scale)),
    ))
}

fn random_pairs_of_primitive_and_geometric_u64<T: 'static + PrimitiveInteger>(
    scale: u32,
) -> It<(T, u64)> {
    Box::new(random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_x(seed)),
        &(|seed| natural_u32s_geometric(seed, scale).map(|i| i.into())),
    ))
}

pub fn pairs_of_unsigned_and_small_u32<T: 'static + PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (T, u32)>> {
    match gm {
        GenerationMode::Exhaustive => sqrt_pairs_of_unsigneds(),
        GenerationMode::Random(scale) => random_pairs_of_primitive_and_geometric_u32(scale),
    }
}

pub fn pairs_of_unsigned_and_small_u64<T: 'static + PrimitiveUnsigned>(
    gm: GenerationMode,
) -> It<(T, u64)> {
    match gm {
        GenerationMode::Exhaustive => sqrt_pairs_of_unsigneds(),
        GenerationMode::Random(scale) => random_pairs_of_primitive_and_geometric_u64(scale),
    }
}

pub fn pairs_of_small_usize_and_unsigned<T: 'static + PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (usize, T)>> {
    match gm {
        GenerationMode::Exhaustive => permute_2_1(Box::new(log_pairs(
            exhaustive_u(),
            exhaustive_u::<u32>().map(|u| u as usize),
        ))),
        GenerationMode::Random(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| natural_u32s_geometric(seed, scale).map(|u| u as usize)),
            &(|seed| random_x(seed)),
        )),
    }
}

fn log_pairs_of_positive_primitive_and_unsigned<
    T: 'static + PrimitiveInteger,
    U: 'static + PrimitiveUnsigned,
>() -> Box<Iterator<Item = (T, U)>> {
    Box::new(log_pairs(exhaustive_positive_x(), exhaustive_u()))
}

pub fn pairs_of_positive_unsigned_and_small_u32<T: 'static + PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (T, u32)>> {
    match gm {
        GenerationMode::Exhaustive => log_pairs_of_positive_primitive_and_unsigned(),
        GenerationMode::Random(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_positive_u(seed)),
            &(|seed| natural_u32s_geometric(seed, scale)),
        )),
    }
}

pub fn pairs_of_positive_signed_and_small_u32<T: 'static + PrimitiveSigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (T, u32)>> {
    match gm {
        GenerationMode::Exhaustive => log_pairs_of_positive_primitive_and_unsigned(),
        GenerationMode::Random(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_positive_i(seed)),
            &(|seed| natural_u32s_geometric(seed, scale)),
        )),
    }
}

fn sqrt_pairs_of_signed_and_unsigned<
    T: 'static + PrimitiveSigned,
    U: 'static + PrimitiveUnsigned,
>() -> Box<Iterator<Item = (T, U)>> {
    Box::new(sqrt_pairs(exhaustive_i(), exhaustive_u()))
}

pub fn pairs_of_signed_and_small_u64<T: 'static + PrimitiveSigned>(
    gm: GenerationMode,
) -> It<(T, u64)> {
    match gm {
        GenerationMode::Exhaustive => sqrt_pairs_of_signed_and_unsigned(),
        GenerationMode::Random(scale) => random_pairs_of_primitive_and_geometric_u64(scale),
    }
}

type ItU<T> = It<(T, u64)>;
fn exhaustive_pairs_of_unsigned_and_u64_width_range<T: 'static + PrimitiveUnsigned>() -> ItU<T> {
    Box::new(lex_pairs(
        exhaustive_u(),
        range_increasing_x(0, u64::from(T::WIDTH) - 1),
    ))
}

fn random_pairs_of_primitive_and_u64_width_range<T: 'static + PrimitiveInteger>() -> It<(T, u64)> {
    Box::new(random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_x(seed)),
        &(|seed| random_range(seed, 0, u64::from(T::WIDTH) - 1)),
    ))
}

pub fn pairs_of_unsigned_and_u64_width_range<T: 'static + PrimitiveUnsigned>(
    gm: GenerationMode,
) -> It<(T, u64)> {
    match gm {
        GenerationMode::Exhaustive => exhaustive_pairs_of_unsigned_and_u64_width_range(),
        GenerationMode::Random(_) => random_pairs_of_primitive_and_u64_width_range(),
    }
}

pub fn pairs_of_signed_and_u64_width_range<T: 'static + PrimitiveSigned>(
    gm: GenerationMode,
) -> It<(T, u64)> {
    match gm {
        GenerationMode::Exhaustive => Box::new(lex_pairs(
            exhaustive_i(),
            range_increasing_x(0, u64::from(T::WIDTH) - 1),
        )),
        GenerationMode::Random(_) => random_pairs_of_primitive_and_u64_width_range(),
    }
}

pub fn pairs_of_signed_and_u64_width_range_var_1<T: 'static + PrimitiveSigned>(
    gm: GenerationMode,
) -> It<(T, u64)> {
    Box::new(
        pairs_of_signed_and_small_u64(gm)
            .filter(|&(n, index)| n < T::ZERO || index < u64::from(T::WIDTH)),
    )
}

pub fn pairs_of_signed_and_u64_width_range_var_2<T: 'static + PrimitiveSigned>(
    gm: GenerationMode,
) -> It<(T, u64)> {
    Box::new(
        pairs_of_signed_and_small_u64(gm)
            .filter(|&(n, index)| n >= T::ZERO || index < u64::from(T::WIDTH)),
    )
}

pub fn triples_of_unsigned_u64_width_range_and_bool_var_1<T: 'static + PrimitiveUnsigned>(
    gm: GenerationMode,
) -> It<(T, u64, bool)> {
    let unfiltered: It<(T, u64, bool)> = match gm {
        GenerationMode::Exhaustive => reshape_2_1_to_3(Box::new(lex_pairs(
            sqrt_pairs_of_unsigneds(),
            exhaustive_bools(),
        ))),
        GenerationMode::Random(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| random_x(seed)),
            &(|seed| natural_u32s_geometric(seed, scale).map(|i| i.into())),
            &(|seed| random_x(seed)),
        )),
    };
    Box::new(unfiltered.filter(|&(_, index, bit)| !bit || index < u64::from(T::WIDTH)))
}

pub fn triples_of_signed_u64_width_range_and_bool_var_1<T: 'static + PrimitiveSigned>(
    gm: GenerationMode,
) -> It<(T, u64, bool)> {
    let unfiltered: It<(T, u64, bool)> = match gm {
        GenerationMode::Exhaustive => reshape_2_1_to_3(Box::new(lex_pairs(
            sqrt_pairs_of_signed_and_unsigned(),
            exhaustive_bools(),
        ))),
        GenerationMode::Random(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| random_x(seed)),
            &(|seed| natural_u32s_geometric(seed, scale).map(|i| i.into())),
            &(|seed| random_x(seed)),
        )),
    };
    Box::new(unfiltered.filter(|&(n, index, bit)| {
        if bit {
            index < u64::from(T::WIDTH) || n < T::ZERO
        } else {
            index < u64::from(T::WIDTH) || n >= T::ZERO
        }
    }))
}

pub fn pairs_of_negative_signed_not_min_and_small_u32s<T: 'static + PrimitiveSigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (T, u32)>> {
    match gm {
        GenerationMode::Exhaustive => Box::new(log_pairs(
            exhaustive_negative_i().filter(|&i| i != T::MIN),
            exhaustive_u(),
        )),
        GenerationMode::Random(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_negative_i(seed).filter(|&i| i != T::MIN)),
            &(|seed| natural_u32s_geometric(seed, scale)),
        )),
    }
}

pub fn chars(gm: GenerationMode) -> Box<Iterator<Item = char>> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_chars()),
        GenerationMode::Random(_) => Box::new(random_x(&EXAMPLE_SEED)),
    }
}

pub fn chars_var_1(gm: GenerationMode) -> Box<Iterator<Item = char>> {
    Box::new(chars(gm).filter(|&c| c != '\u{0}'))
}

pub fn chars_var_2(gm: GenerationMode) -> Box<Iterator<Item = char>> {
    Box::new(chars(gm).filter(|&c| c != char::MAX))
}

pub fn pairs_of_chars(gm: GenerationMode) -> Box<Iterator<Item = (char, char)>> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_pairs_from_single(exhaustive_chars())),
        GenerationMode::Random(_) => Box::new(random_pairs_from_single(random_x(&EXAMPLE_SEED))),
    }
}

pub fn rounding_modes(gm: GenerationMode) -> Box<Iterator<Item = RoundingMode>> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_rounding_modes()),
        GenerationMode::Random(_) => Box::new(random_rounding_modes(&EXAMPLE_SEED)),
    }
}

pub fn pairs_of_rounding_modes(
    gm: GenerationMode,
) -> Box<Iterator<Item = (RoundingMode, RoundingMode)>> {
    match gm {
        GenerationMode::Exhaustive => Box::new(lex_pairs(
            exhaustive_rounding_modes(),
            exhaustive_rounding_modes(),
        )),
        GenerationMode::Random(_) => Box::new(random_pairs_from_single(random_rounding_modes(
            &EXAMPLE_SEED,
        ))),
    }
}

pub fn triples_of_rounding_modes(
    gm: GenerationMode,
) -> Box<Iterator<Item = (RoundingMode, RoundingMode, RoundingMode)>> {
    match gm {
        GenerationMode::Exhaustive => Box::new(lex_triples(
            exhaustive_rounding_modes(),
            exhaustive_rounding_modes(),
            exhaustive_rounding_modes(),
        )),
        GenerationMode::Random(_) => Box::new(random_triples_from_single(random_rounding_modes(
            &EXAMPLE_SEED,
        ))),
    }
}

fn random_pairs_of_primitive_and_rounding_mode<T: 'static + PrimitiveInteger>(
) -> Box<Iterator<Item = (T, RoundingMode)>> {
    Box::new(random_pairs(
        &EXAMPLE_SEED,
        &(|seed| random_x(seed)),
        &(|seed| random_rounding_modes(seed)),
    ))
}

pub fn pairs_of_unsigned_and_rounding_mode<T: 'static + PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (T, RoundingMode)>> {
    match gm {
        GenerationMode::Exhaustive => {
            Box::new(lex_pairs(exhaustive_u(), exhaustive_rounding_modes()))
        }
        GenerationMode::Random(_) => random_pairs_of_primitive_and_rounding_mode(),
    }
}

pub fn pairs_of_signed_and_rounding_mode<T: 'static + PrimitiveSigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (T, RoundingMode)>> {
    match gm {
        GenerationMode::Exhaustive => {
            Box::new(lex_pairs(exhaustive_i(), exhaustive_rounding_modes()))
        }
        GenerationMode::Random(_) => random_pairs_of_primitive_and_rounding_mode(),
    }
}

pub fn vecs_of_unsigned<T: 'static + PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = Vec<T>>> {
    match gm {
        GenerationMode::Exhaustive => Box::new(exhaustive_vecs(exhaustive_u())),
        GenerationMode::Random(scale) => {
            Box::new(random_vecs(&EXAMPLE_SEED, scale, &(|seed| random_x(seed))))
        }
    }
}

fn pairs_of_ordering_and_vec_of_unsigned<T: 'static + PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Ordering, Vec<T>)>> {
    match gm {
        GenerationMode::Exhaustive => permute_2_1(Box::new(lex_pairs(
            exhaustive_vecs(exhaustive_u()),
            exhaustive_orderings(),
        ))),
        GenerationMode::Random(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_orderings(seed)),
            &(|seed| random_vecs(seed, scale, &(|seed_2| random_x(seed_2)))),
        )),
    }
}

pub fn pairs_of_ordering_and_vec_of_unsigned_var_1(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Ordering, Vec<u32>)>> {
    Box::new(
        pairs_of_ordering_and_vec_of_unsigned(gm)
            .filter(|&(sign, ref limbs)| limbs_test_zero(limbs) == (sign == Ordering::Equal)),
    )
}

fn exhaustive_pairs_of_unsigned_vec_and_unsigned<T: 'static + PrimitiveUnsigned>(
) -> Box<Iterator<Item = (Vec<T>, T)>> {
    Box::new(exhaustive_pairs(
        exhaustive_vecs(exhaustive_u()),
        exhaustive_u(),
    ))
}

pub fn triples_of_unsigned_vec_small_usize_and_unsigned<T: 'static + PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Vec<T>, usize, T)>> {
    match gm {
        GenerationMode::Exhaustive => permute_1_3_2(reshape_2_1_to_3(Box::new(log_pairs(
            exhaustive_pairs_of_unsigned_vec_and_unsigned(),
            exhaustive_u::<u32>().map(|u| u as usize),
        )))),
        GenerationMode::Random(scale) => Box::new(random_triples(
            &EXAMPLE_SEED,
            &(|seed| random_vecs(seed, scale, &(|seed_2| random_x(seed_2)))),
            &(|seed| natural_u32s_geometric(seed, scale).map(|u| u as usize)),
            &(|seed| random_x(seed)),
        )),
    }
}

fn pairs_of_unsigned_vec_and_small_usize<T: 'static + PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Vec<T>, usize)>> {
    match gm {
        GenerationMode::Exhaustive => Box::new(log_pairs(
            exhaustive_vecs(exhaustive_u()),
            exhaustive_u::<u32>().map(|u| u as usize),
        )),
        GenerationMode::Random(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_vecs(seed, scale, &(|seed_2| random_x(seed_2)))),
            &(|seed| natural_u32s_geometric(seed, scale).map(|u| u as usize)),
        )),
    }
}

pub fn pairs_of_unsigned_vec_and_small_usize_var_1<T: 'static + PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Vec<T>, usize)>> {
    Box::new(pairs_of_unsigned_vec_and_small_usize(gm).filter(|&(ref xs, u)| u <= xs.len()))
}

pub fn pairs_of_unsigned_vec_and_unsigned<T: 'static + PrimitiveUnsigned>(
    gm: GenerationMode,
) -> Box<Iterator<Item = (Vec<T>, T)>> {
    match gm {
        GenerationMode::Exhaustive => exhaustive_pairs_of_unsigned_vec_and_unsigned(),
        GenerationMode::Random(scale) => Box::new(random_pairs(
            &EXAMPLE_SEED,
            &(|seed| random_vecs(seed, scale, &(|seed_2| random_x(seed_2)))),
            &(|seed| random_x(seed)),
        )),
    }
}
