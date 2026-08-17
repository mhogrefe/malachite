use malachite_base::num::arithmetic::traits::{AddMul, MulAddMul, WrappingAddMul};
use malachite_base::num::basic::traits::Two;
use malachite_nz::natural::Natural;
use malachite_float::Float;
use malachite_q::Rational;

const X: Natural = Natural::const_from(5);
const Y: Natural = Natural::const_from(3);
const Z: Natural = Natural::const_from(7);
const W: Natural = Natural::TWO;
const F: Float = Float::const_from_unsigned(3);
const G: Float = Float::const_from_unsigned(5);
const H: Float = Float::const_from_unsigned(7);
const P: Rational = Rational::const_from_unsigned(3);
const Q: Rational = Rational::const_from_unsigned(5);
const R: Rational = Rational::const_from_unsigned(7);

// A type with `Mul` and `Add` but no fused operation.
#[derive(Clone, Copy)]
struct Meters(u32);

impl core::ops::Mul for Meters {
    type Output = Meters;

    fn mul(self, other: Meters) -> Meters {
        Meters(self.0 * other.0)
    }
}

impl core::ops::Add for Meters {
    type Output = Meters;

    fn add(self, other: Meters) -> Meters {
        Meters(self.0 + other.0)
    }
}

fn main() {
    // Bignums, three operands: flagged.
    let _ = &X + &Y * &Z;
    let _ = &X - &Y * &Z;
    // The product may come first for addition, which is commutative.
    let _ = &Y * &Z + &X;
    // Bignums, four operands: one `mul_add_mul`, not a nested `add_mul`.
    let _ = &X * &Y + &Z * &W;
    let _ = &X * &Y - &Z * &W;
    // The assigning forms.
    let mut a = X.clone();
    a += &Y * &Z;
    let mut b = X.clone();
    b -= &Y * &Z;
    // Primitive integers are NOT flagged for the operator form: `add_mul` wraps, while `+` and
    // `*` panic on overflow in a debug build, so this is not an equivalent rewrite.
    let i: u32 = 5;
    let _ = i + i * i;
    let _ = i * i + i * i;
    let mut j: u32 = 5;
    j += i * i;
    // The explicitly wrapping composition on a primitive integer IS flagged: `wrapping_add_mul`
    // wraps in exactly the same way.
    let _ = i.wrapping_add(i.wrapping_mul(i));
    let _ = i.wrapping_sub(i.wrapping_mul(i));
    let _ = i.wrapping_mul(i).wrapping_add(i.wrapping_mul(i));

    // `Rational` gained `AddMul` and `SubMul` with the `fmpq_addmul` port, and the lint is driven
    // by which impls exist, so it picked them up with no change of its own.
    let _ = &P + &Q * &R;
    let _ = &P - &Q * &R;
    // The addend is also a factor. A fused call cannot borrow and consume it at once, but the
    // repeated operand can be factored out -- `n * (Y + 1)` -- so this is flagged with that
    // advice instead.
    let n = X.clone();
    let _ = &n * &Y + n;
    let n2 = X.clone();
    let _ = &n2 * &Y - n2;
    // A product subtracted from nothing: `y * z - x` is not `x.sub_mul(y, z)`, so it is not
    // flagged.
    let _ = &Y * &Z - &X;
    // No multiplication at all: not flagged.
    let _ = &X + &Y;
    let _ = &X - &Y;
    // A type with no fused operation: not flagged.
    let m = Meters(3);
    let _ = m + m * m;
    // A wrapping add whose argument is not a wrapping multiply: not flagged.
    let _ = i.wrapping_add(i);
    // The addend occurs *inside* a factor rather than being one, so there is neither a fused
    // call nor a tidy factoring: not flagged.
    let n3 = X.clone();
    let _ = &Y * (&n3 + &Z) + n3;
    // Primitive floats are not flagged either: their `add_mul` is `self + y * z`, so it neither
    // fuses the rounding nor saves work.
    let _ = 1.5f64 + 2.5f64 * 3.5f64;
    // `Float` has the fused traits but is not flagged: its fused operations round once instead
    // of twice, computing a different value at a higher cost, so the choice between the
    // spellings is semantic and the lint must not make it.
    let _ = &F + &G * &H;
    let _ = &F - &G * &H;

    println!("{a} {b} {j}");
}
