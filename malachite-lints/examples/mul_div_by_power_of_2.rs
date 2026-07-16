use malachite_base::num::arithmetic::traits::PowerOf2;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_q::Rational;

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
}
