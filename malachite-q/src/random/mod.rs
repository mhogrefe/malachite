use malachite_base::bools::random::{random_bools, RandomBools};
use malachite_base::num::random::geometric::GeometricRandomNaturalValues;
use malachite_base::random::Seed;
use malachite_nz::natural::random::{
    random_naturals, random_positive_naturals, striped_random_naturals,
    striped_random_positive_naturals, RandomNaturals, StripedRandomNaturals,
};
use malachite_nz::natural::Natural;
use Rational;

#[derive(Clone, Debug)]
pub struct RandomRationalsFromSingle<I: Iterator<Item = Natural>> {
    xs: I,
}

impl<I: Iterator<Item = Natural>> Iterator for RandomRationalsFromSingle<I> {
    type Item = Rational;

    fn next(&mut self) -> Option<Rational> {
        Some(Rational::from_naturals(
            self.xs.next().unwrap(),
            self.xs.next().unwrap(),
        ))
    }
}

pub fn random_positive_rationals(
    seed: Seed,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
) -> RandomRationalsFromSingle<RandomNaturals<GeometricRandomNaturalValues<u64>>> {
    RandomRationalsFromSingle {
        xs: random_positive_naturals(seed, mean_bits_numerator, mean_bits_denominator),
    }
}

#[derive(Clone, Debug)]
pub struct RandomRationalsFromDouble<I: Iterator<Item = Natural>, J: Iterator<Item = Natural>> {
    xs: I,
    ys: J,
}

impl<I: Iterator<Item = Natural>, J: Iterator<Item = Natural>> Iterator
    for RandomRationalsFromDouble<I, J>
{
    type Item = Rational;

    fn next(&mut self) -> Option<Rational> {
        Some(Rational::from_naturals(
            self.xs.next().unwrap(),
            self.ys.next().unwrap(),
        ))
    }
}

pub fn random_non_negative_rationals(
    seed: Seed,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
) -> RandomRationalsFromDouble<
    RandomNaturals<GeometricRandomNaturalValues<u64>>,
    RandomNaturals<GeometricRandomNaturalValues<u64>>,
> {
    RandomRationalsFromDouble {
        xs: random_naturals(
            seed.fork("numerator"),
            mean_bits_numerator,
            mean_bits_denominator,
        ),
        ys: random_positive_naturals(
            seed.fork("denominator"),
            mean_bits_numerator,
            mean_bits_denominator,
        ),
    }
}

#[derive(Clone, Debug)]
pub struct NegativeRationals<I: Iterator<Item = Rational>> {
    xs: I,
}

impl<I: Iterator<Item = Rational>> Iterator for NegativeRationals<I> {
    type Item = Rational;

    fn next(&mut self) -> Option<Rational> {
        self.xs.next().map(|mut q| {
            q.sign = false;
            q
        })
    }
}

pub fn random_negative_rationals(
    seed: Seed,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
) -> NegativeRationals<RandomRationalsFromSingle<RandomNaturals<GeometricRandomNaturalValues<u64>>>>
{
    NegativeRationals {
        xs: random_positive_rationals(seed, mean_bits_numerator, mean_bits_denominator),
    }
}

#[derive(Clone, Debug)]
pub struct RandomRationalsFromSingleAndSign<I: Iterator<Item = Natural>> {
    bs: RandomBools,
    xs: I,
}

impl<I: Iterator<Item = Natural>> Iterator for RandomRationalsFromSingleAndSign<I> {
    type Item = Rational;

    fn next(&mut self) -> Option<Rational> {
        Some(Rational::from_sign_and_naturals(
            self.bs.next().unwrap(),
            self.xs.next().unwrap(),
            self.xs.next().unwrap(),
        ))
    }
}

pub fn random_nonzero_rationals(
    seed: Seed,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
) -> RandomRationalsFromSingleAndSign<RandomNaturals<GeometricRandomNaturalValues<u64>>> {
    RandomRationalsFromSingleAndSign {
        bs: random_bools(seed.fork("sign")),
        xs: random_positive_naturals(seed.fork("abs"), mean_bits_numerator, mean_bits_denominator),
    }
}

#[derive(Clone, Debug)]
pub struct RandomRationalsFromDoubleAndSign<
    I: Iterator<Item = Natural>,
    J: Iterator<Item = Natural>,
> {
    bs: RandomBools,
    xs: I,
    ys: J,
}

impl<I: Iterator<Item = Natural>, J: Iterator<Item = Natural>> Iterator
    for RandomRationalsFromDoubleAndSign<I, J>
{
    type Item = Rational;

    fn next(&mut self) -> Option<Rational> {
        Some(Rational::from_sign_and_naturals(
            self.bs.next().unwrap(),
            self.xs.next().unwrap(),
            self.ys.next().unwrap(),
        ))
    }
}

pub fn random_rationals(
    seed: Seed,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
) -> RandomRationalsFromDoubleAndSign<
    RandomNaturals<GeometricRandomNaturalValues<u64>>,
    RandomNaturals<GeometricRandomNaturalValues<u64>>,
> {
    RandomRationalsFromDoubleAndSign {
        bs: random_bools(seed.fork("sign")),
        xs: random_naturals(
            seed.fork("numerator"),
            mean_bits_numerator,
            mean_bits_denominator,
        ),
        ys: random_positive_naturals(
            seed.fork("denominator"),
            mean_bits_numerator,
            mean_bits_denominator,
        ),
    }
}

pub fn striped_random_positive_rationals(
    seed: Seed,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
) -> RandomRationalsFromSingle<StripedRandomNaturals<GeometricRandomNaturalValues<u64>>> {
    RandomRationalsFromSingle {
        xs: striped_random_positive_naturals(
            seed,
            mean_stripe_numerator,
            mean_stripe_denominator,
            mean_bits_numerator,
            mean_bits_denominator,
        ),
    }
}

pub fn striped_random_non_negative_rationals(
    seed: Seed,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
) -> RandomRationalsFromDouble<
    StripedRandomNaturals<GeometricRandomNaturalValues<u64>>,
    StripedRandomNaturals<GeometricRandomNaturalValues<u64>>,
> {
    RandomRationalsFromDouble {
        xs: striped_random_naturals(
            seed.fork("numerator"),
            mean_stripe_numerator,
            mean_stripe_denominator,
            mean_bits_numerator,
            mean_bits_denominator,
        ),
        ys: striped_random_positive_naturals(
            seed.fork("denominator"),
            mean_stripe_numerator,
            mean_stripe_denominator,
            mean_bits_numerator,
            mean_bits_denominator,
        ),
    }
}

pub fn striped_random_negative_rationals(
    seed: Seed,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
) -> NegativeRationals<
    RandomRationalsFromSingle<StripedRandomNaturals<GeometricRandomNaturalValues<u64>>>,
> {
    NegativeRationals {
        xs: striped_random_positive_rationals(
            seed,
            mean_stripe_numerator,
            mean_stripe_denominator,
            mean_bits_numerator,
            mean_bits_denominator,
        ),
    }
}
pub fn striped_random_nonzero_rationals(
    seed: Seed,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
) -> RandomRationalsFromSingleAndSign<StripedRandomNaturals<GeometricRandomNaturalValues<u64>>> {
    RandomRationalsFromSingleAndSign {
        bs: random_bools(seed.fork("sign")),
        xs: striped_random_positive_naturals(
            seed.fork("abs"),
            mean_stripe_numerator,
            mean_stripe_denominator,
            mean_bits_numerator,
            mean_bits_denominator,
        ),
    }
}

pub fn striped_random_rationals(
    seed: Seed,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
) -> RandomRationalsFromDoubleAndSign<
    StripedRandomNaturals<GeometricRandomNaturalValues<u64>>,
    StripedRandomNaturals<GeometricRandomNaturalValues<u64>>,
> {
    RandomRationalsFromDoubleAndSign {
        bs: random_bools(seed.fork("sign")),
        xs: striped_random_naturals(
            seed.fork("numerator"),
            mean_stripe_numerator,
            mean_stripe_denominator,
            mean_bits_numerator,
            mean_bits_denominator,
        ),
        ys: striped_random_positive_naturals(
            seed.fork("denominator"),
            mean_stripe_numerator,
            mean_stripe_denominator,
            mean_bits_numerator,
            mean_bits_denominator,
        ),
    }
}
