// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::gaussian_integer::GaussianInteger;
use crate::gaussian_integer::exhaustive::gaussian_integer_from_pair;
use crate::integer::Integer;
use crate::integer::random::{
    RandomIntegers, StripedRandomIntegers, random_integers, striped_random_integers,
};
use core::iter::Map;
use malachite_base::num::conversion::traits::ImaginaryFrom;
use malachite_base::num::random::geometric::GeometricRandomSigneds;
use malachite_base::random::Seed;
use malachite_base::tuples::random::{RandomPairsFromSingle, random_pairs_from_single};

// Generates random purely real Gaussian integers whose real parts have a specified mean bit length.
#[inline]
pub fn random_real_gaussian_integers(
    seed: Seed,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
) -> Map<RandomIntegers<GeometricRandomSigneds<i64>>, fn(Integer) -> GaussianInteger> {
    random_integers(seed, mean_bits_numerator, mean_bits_denominator).map(GaussianInteger::from)
}

// Generates random purely imaginary Gaussian integers whose imaginary parts have a specified mean
// bit length.
#[inline]
pub fn random_imaginary_gaussian_integers(
    seed: Seed,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
) -> Map<RandomIntegers<GeometricRandomSigneds<i64>>, fn(Integer) -> GaussianInteger> {
    random_integers(seed, mean_bits_numerator, mean_bits_denominator)
        .map(GaussianInteger::imaginary_from)
}

// Generates random Gaussian integers whose real and imaginary parts have a specified mean bit
// length.
#[inline]
pub fn random_gaussian_integers(
    seed: Seed,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
) -> Map<
    RandomPairsFromSingle<RandomIntegers<GeometricRandomSigneds<i64>>>,
    fn((Integer, Integer)) -> GaussianInteger,
> {
    random_pairs_from_single(random_integers(
        seed,
        mean_bits_numerator,
        mean_bits_denominator,
    ))
    .map(gaussian_integer_from_pair)
}

// Generates random purely real striped Gaussian integers whose real parts have a specified mean bit
// length.
#[inline]
pub fn striped_random_real_gaussian_integers(
    seed: Seed,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
) -> Map<StripedRandomIntegers<GeometricRandomSigneds<i64>>, fn(Integer) -> GaussianInteger> {
    striped_random_integers(
        seed,
        mean_stripe_numerator,
        mean_stripe_denominator,
        mean_bits_numerator,
        mean_bits_denominator,
    )
    .map(GaussianInteger::from)
}

// Generates random purely imaginary striped Gaussian integers whose imaginary parts have a
// specified mean bit length.
#[inline]
pub fn striped_random_imaginary_gaussian_integers(
    seed: Seed,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
) -> Map<StripedRandomIntegers<GeometricRandomSigneds<i64>>, fn(Integer) -> GaussianInteger> {
    striped_random_integers(
        seed,
        mean_stripe_numerator,
        mean_stripe_denominator,
        mean_bits_numerator,
        mean_bits_denominator,
    )
    .map(GaussianInteger::imaginary_from)
}

// Generates random striped Gaussian integers whose real and imaginary parts have a specified mean
// bit length.
#[inline]
pub fn striped_random_gaussian_integers(
    seed: Seed,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
) -> Map<
    RandomPairsFromSingle<StripedRandomIntegers<GeometricRandomSigneds<i64>>>,
    fn((Integer, Integer)) -> GaussianInteger,
> {
    random_pairs_from_single(striped_random_integers(
        seed,
        mean_stripe_numerator,
        mean_stripe_denominator,
        mean_bits_numerator,
        mean_bits_denominator,
    ))
    .map(gaussian_integer_from_pair)
}
