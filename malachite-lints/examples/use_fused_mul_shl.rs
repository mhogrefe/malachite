use malachite_base::num::arithmetic::traits::{
    AddMul, AddMulAssign, ModAdd, ModAddAssign, ModAddMul, ModMul, ModPowerOf2Add,
    ModPowerOf2AddMul, ModPowerOf2Mul, ModPowerOf2Shl, ModPowerOf2Sub, ModShl, ModSub, SubMul,
};
use malachite_float::Float;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_q::Rational;

const X: Natural = Natural::const_from(5);
const Y: Natural = Natural::const_from(3);
const Z: Natural = Natural::const_from(7);
const M: Natural = Natural::const_from(11);
const F: Float = Float::const_from_unsigned(3);
const G: Float = Float::const_from_unsigned(5);
const P: Rational = Rational::const_from_unsigned(3);
const Q: Rational = Rational::const_from_unsigned(5);
const I: Integer = Integer::const_from_signed(5);
const J: Integer = Integer::const_from_signed(3);
const K: Integer = Integer::const_from_signed(7);

fn main() {
    let i = I;
    let j = J;
    let k = K;
    let bits = 3u64;
    // The operator forms on types with the fused traits: flagged.
    let _ = &i + ((&j * &k) << bits);
    let _ = &i - ((&j * &k) << bits);
    // The shifted product may come first for addition, which is commutative.
    let _ = ((&j * &k) << bits) + &i;
    let mut a = i.clone();
    a += (&j * &k) << bits;
    let mut b = i.clone();
    b -= (&j * &k) << bits;
    let _ = &P + ((&Q * &P) << 2u32);
    // `Natural` has `AddMulShl` but no `SubMulShl`: addition is flagged, subtraction is not.
    let _ = &X + ((&Y * &Z) << bits);
    let _ = &X - ((&Y * &Z) << bits);
    // A shifted product subtracted from nothing is not a `sub_mul_shl`: not flagged.
    let _ = ((&j * &k) << bits) - &i;
    // A signed shift amount may be negative, shifting the other way: not flagged.
    let signed = 3i32;
    let _ = &i + ((&j * &k) << signed);
    // `Float` has no `AddMulShl`: not flagged.
    let _ = &F + ((&G * &F) << bits);

    // A fused multiply with a shifted factor: flagged.
    let _ = (&i).add_mul(&(&j << bits), &k);
    let _ = (&i).sub_mul(&j, &(&k << bits));
    let mut c = X.clone();
    c.add_mul_assign(&Y << bits, &Z);
    // Without a shift: not flagged.
    let _ = (&i).add_mul(&j, &k);

    // Arithmetic modulo a power of 2: flagged when the power matches throughout.
    let pow = 8u64;
    let x = 5u32;
    let y = 6u32;
    let _ = x.mod_power_of_2_add(
        x.mod_power_of_2_mul(y, pow).mod_power_of_2_shl(2u64, pow),
        pow,
    );
    let _ = x.mod_power_of_2_sub(
        x.mod_power_of_2_shl(2u64, pow).mod_power_of_2_mul(y, pow),
        pow,
    );
    let _ = x.mod_power_of_2_add_mul(x.mod_power_of_2_shl(2u64, pow), x, pow);
    // Different powers: not flagged.
    let _ = x.mod_power_of_2_add(x.mod_power_of_2_mul(y, 4).mod_power_of_2_shl(2u64, 4), pow);

    // Arithmetic modulo a number: flagged when the modulus matches throughout.
    let m = 11u32;
    let _ = x.mod_add(x.mod_mul(y, m).mod_shl(2u64, m), m);
    let _ = x.mod_sub(x.mod_mul(x.mod_shl(2u64, m), m), m);
    let _ = x.mod_add_mul(x, x.mod_shl(2u64, m), m);
    let mut d = x;
    d.mod_add_assign(x.mod_mul(y, m).mod_shl(2u64, m), m);
    let _ = (&X).mod_add((&Y).mod_mul(&Z, &M).mod_shl(2u64, &M), &M);
    // Different moduli: not flagged.
    let _ = x.mod_add(x.mod_mul(y, 7).mod_shl(2u64, 7), m);
    // No shift: not flagged.
    let _ = x.mod_add(x.mod_mul(y, m), m);
}
