use malachite_base::num::arithmetic::traits::{DivRound, DivRoundAssign, PowerOf2};
use malachite_base::rounding_modes::RoundingMode::*;
use malachite_nz::gaussian_integer::GaussianInteger;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_q::Rational;
use malachite_q::gaussian_rational::GaussianRational;

fn main() {
    let n = const { Natural::const_from(100) };
    let i = const { Integer::const_from_signed(-100) };
    let q = const { Rational::const_from_unsigneds(1, 3) };
    // Multiplying by power_of_2, in either operand order: flagged.
    let _ = &n * Natural::power_of_2(5);
    let _ = Natural::power_of_2(5) * &n;
    let mut m = n.clone();
    m *= Natural::power_of_2(5);
    // Dividing by power_of_2: flagged; for `Integer` the advice is `shr_round`, since `/` truncates
    // while `>>` takes the floor.
    let _ = &n / Natural::power_of_2(5);
    let _ = &i / Integer::power_of_2(5);
    let _ = &q / Rational::power_of_2(-5i64);
    let mut p = q.clone();
    p /= Rational::power_of_2(5i64);
    // power_of_2 as the dividend is not a shift of the divisor: fine.
    let _ = Rational::power_of_2(5i64) / &q;
    // Other operations with power_of_2: fine.
    let _ = &i + Integer::power_of_2(5);
    // Multiplying by something that is not power_of_2: fine.
    let _ = &n * const { Natural::const_from(32) };
    let g = GaussianInteger::from(3u32);
    let mut h = GaussianRational::from(3u32);
    let _ = &g * GaussianInteger::power_of_2(5);
    let _ = &h * GaussianRational::power_of_2(5u64);
    let _ = &h / GaussianRational::power_of_2(-5i64);
    h /= GaussianRational::power_of_2(2u64);
    // `GaussianInteger` division is nearest-rounding, with no right shift: not flagged.
    let _ = &g / GaussianInteger::power_of_2(5);
    // `div_round` by power_of_2: flagged, use `shr_round` with the same rounding mode.
    let _ = (&n).div_round(Natural::power_of_2(5), Ceiling).0;
    let _ = (&i).div_round(Integer::power_of_2(5), Nearest).0;
    // `div_round_assign`: flagged, use `shr_round_assign`.
    let mut r = i.clone();
    r.div_round_assign(Integer::power_of_2(3), Floor);
    // `div_round` by something that is not power_of_2: fine.
    let _ = (&n).div_round(const { Natural::const_from(32) }, Ceiling).0;
}
