use num::arithmetic::traits::{
    CeilingModPowerOf2, CeilingModPowerOf2Assign, CheckedNeg, ModPowerOf2, ModPowerOf2Assign,
    NegModPowerOf2, NegModPowerOf2Assign, RemPowerOf2, RemPowerOf2Assign, WrappingNeg,
};
use num::basic::integers::PrimitiveInt;
use num::basic::traits::Zero;
use num::conversion::traits::{CheckedFrom, WrappingFrom};

const ERROR_MESSAGE: &str = "Result exceeds width of output type";

fn _mod_power_of_2_unsigned<T: PrimitiveInt>(x: T, pow: u64) -> T {
    if x == T::ZERO || pow >= T::WIDTH {
        x
    } else {
        x & T::low_mask(pow)
    }
}

fn _mod_power_of_2_assign_unsigned<T: PrimitiveInt>(x: &mut T, pow: u64) {
    if *x != T::ZERO && pow < T::WIDTH {
        *x &= T::low_mask(pow)
    }
}

#[inline]
fn _neg_mod_power_of_2_unsigned<T: ModPowerOf2<Output = T> + PrimitiveInt>(x: T, pow: u64) -> T {
    if x != T::ZERO && pow > T::WIDTH {
        panic!("{}", ERROR_MESSAGE);
    }
    x.wrapping_neg().mod_power_of_2(pow)
}

macro_rules! impl_mod_power_of_2_unsigned {
    ($s:ident) => {
        impl ModPowerOf2 for $s {
            type Output = $s;

            /// Divides a value by a power of 2, returning just the remainder.
            ///
            /// If the quotient were computed, the quotient and remainder would satisfy
            /// $x = q2^p + r$ and $0 \leq r < 2^p$.
            ///
            /// $$
            /// f(x, p) = x - 2^p\left \lfloor \frac{x}{2^p} \right \rfloor.
            /// $$
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::mod_power_of_2` module.
            #[inline]
            fn mod_power_of_2(self, pow: u64) -> $s {
                _mod_power_of_2_unsigned(self, pow)
            }
        }

        impl ModPowerOf2Assign for $s {
            /// Divides a value by a power of 2, replacing the first value by the remainder.
            ///
            /// If the quotient were computed, the quotient and remainder would satisfy
            /// $x = q2^p + r$ and $0 \leq r < 2^p$.
            ///
            /// $$
            /// x \gets x - 2^p\left \lfloor \frac{x}{2^p} \right \rfloor.
            /// $$
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::mod_power_of_2` module.
            #[inline]
            fn mod_power_of_2_assign(&mut self, pow: u64) {
                _mod_power_of_2_assign_unsigned(self, pow);
            }
        }

        impl RemPowerOf2 for $s {
            type Output = $s;

            /// Divides a value by a power of 2, returning just the remainder. For unsigned
            /// integers, rem is equivalent to mod.
            ///
            /// If the quotient were computed, the quotient and remainder would satisfy
            /// $x = q2^p + r$ and $0 \leq r < 2^p$.
            ///
            /// $$
            /// f(x, p) = x - 2^p\left \lfloor \frac{x}{2^p} \right \rfloor.
            /// $$
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::mod_power_of_2` module.
            #[inline]
            fn rem_power_of_2(self, pow: u64) -> $s {
                self.mod_power_of_2(pow)
            }
        }

        impl RemPowerOf2Assign for $s {
            /// Divides a value by a power of 2, replacing the first value by the remainder. For
            /// unsigned integers, rem is equivalent to mod.
            ///
            /// If the quotient were computed, the quotient and remainder would satisfy
            /// $x = q2^p + r$ and $0 \leq r < 2^p$.
            ///
            /// $$
            /// x \gets x - 2^p\left \lfloor \frac{x}{2^p} \right \rfloor.
            /// $$
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::mod_power_of_2` module.
            #[inline]
            fn rem_power_of_2_assign(&mut self, pow: u64) {
                self.mod_power_of_2_assign(pow)
            }
        }

        impl NegModPowerOf2 for $s {
            type Output = $s;

            /// Divides the negative of a value by a power of 2, returning just the remainder.
            ///
            /// If the quotient were computed, the quotient and remainder would satisfy
            /// $x = q2^p - r$ and $0 \leq r < 2^p$.
            ///
            /// $$
            /// f(x, p) = 2^p\left \lceil \frac{x}{2^p} \right \rceil - x.
            /// $$
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `self` is nonzero and `pow` is greater than `$s::WIDTH`.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::mod_power_of_2` module.
            #[inline]
            fn neg_mod_power_of_2(self, pow: u64) -> $s {
                _neg_mod_power_of_2_unsigned(self, pow)
            }
        }

        impl NegModPowerOf2Assign for $s {
            /// Divides the negative of a value by a power of 2, returning just the remainder.
            ///
            /// If the quotient were computed, the quotient and remainder would satisfy
            /// $x = q2^p - r$ and $0 \leq r < 2^p$.
            ///
            /// $$
            /// x \gets 2^p\left \lceil \frac{x}{2^p} \right \rceil - x.
            /// $$
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `self` is nonzero and `pow` is greater than `$s::WIDTH`.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::mod_power_of_2` module.
            #[inline]
            fn neg_mod_power_of_2_assign(&mut self, pow: u64) {
                *self = self.neg_mod_power_of_2(pow)
            }
        }
    };
}
apply_to_unsigneds!(impl_mod_power_of_2_unsigned);

fn _mod_power_of_2_signed<U: ModPowerOf2<Output = U> + WrappingFrom<S>, S: PrimitiveInt>(
    x: S,
    pow: u64,
) -> U {
    if x < S::ZERO && pow > S::WIDTH {
        panic!("{}", ERROR_MESSAGE);
    }
    U::wrapping_from(x).mod_power_of_2(pow)
}

fn _mod_power_of_2_assign_signed<U, S: CheckedFrom<U> + Copy + ModPowerOf2<Output = U>>(
    x: &mut S,
    pow: u64,
) {
    *x = S::checked_from(x.mod_power_of_2(pow)).expect(ERROR_MESSAGE);
}

fn _rem_power_of_2_signed<
    U: ModPowerOf2<Output = U> + WrappingFrom<S>,
    S: Copy + Ord + WrappingFrom<U> + WrappingNeg<Output = S> + Zero,
>(
    x: S,
    pow: u64,
) -> S {
    if x >= S::ZERO {
        S::wrapping_from(U::wrapping_from(x).mod_power_of_2(pow))
    } else {
        S::wrapping_from(U::wrapping_from(x.wrapping_neg()).mod_power_of_2(pow)).wrapping_neg()
    }
}

fn _ceiling_mod_power_of_2_signed<
    U: ModPowerOf2<Output = U> + NegModPowerOf2<Output = U> + WrappingFrom<S>,
    S: CheckedFrom<U> + CheckedNeg<Output = S> + Copy + Ord + WrappingNeg<Output = S> + Zero,
>(
    x: S,
    pow: u64,
) -> S {
    let abs_result = if x >= S::ZERO {
        U::wrapping_from(x).neg_mod_power_of_2(pow)
    } else {
        U::wrapping_from(x.wrapping_neg()).mod_power_of_2(pow)
    };
    S::checked_from(abs_result)
        .expect(ERROR_MESSAGE)
        .checked_neg()
        .expect(ERROR_MESSAGE)
}

macro_rules! impl_mod_power_of_2_signed {
    ($u:ident, $s:ident) => {
        impl ModPowerOf2 for $s {
            type Output = $u;

            /// Divides a value by a power of 2, returning just the remainder. The remainder is
            /// non-negative.
            ///
            /// If the quotient were computed, the quotient and remainder would satisfy
            /// $x = q2^p + r$ and $0 \leq r < 2^p$.
            ///
            /// $$
            /// f(x, p) = x - 2^p\left \lfloor \frac{x}{2^p} \right \rfloor.
            /// $$
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `self` is negative and `pow` is greater than `$s::WIDTH`.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::mod_power_of_2` module.
            #[inline]
            fn mod_power_of_2(self, pow: u64) -> $u {
                _mod_power_of_2_signed(self, pow)
            }
        }

        impl ModPowerOf2Assign for $s {
            /// Divides a value by a power of 2, replacing the first value by the remainder. The
            /// remainder is non-negative.
            ///
            /// If the quotient were computed, he quotient and remainder would satisfy
            /// $x = q2^p + r$ and $0 \leq r < 2^p$.
            ///
            /// $$
            /// x \gets x - 2^p\left \lfloor \frac{x}{2^p} \right \rfloor.
            /// $$
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `self` is negative and `pow` is greater than or equal to `$s::WIDTH`.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::mod_power_of_2` module.
            #[inline]
            fn mod_power_of_2_assign(&mut self, pow: u64) {
                _mod_power_of_2_assign_signed(self, pow);
            }
        }

        impl RemPowerOf2 for $s {
            type Output = $s;

            /// Divides a value by a power of 2, returning just the remainder. The remainder has
            /// the same sign as the first value.
            ///
            /// If the quotient were computed, the quotient and remainder would satisfy
            /// $x = q2^p + r$ and $0 \leq |r| < 2^p$.
            ///
            /// $$
            /// f(x, p) = x - 2^p\operatorname{sgn}(x)\left \lfloor \frac{|x|}{2^p} \right \rfloor.
            /// $$
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::mod_power_of_2` module.
            #[inline]
            fn rem_power_of_2(self, pow: u64) -> $s {
                _rem_power_of_2_signed::<$u, $s>(self, pow)
            }
        }

        impl RemPowerOf2Assign for $s {
            /// Divides a value by a power of 2, replacing the first value by the remainder. The
            /// remainder has the same sign as the first value.
            ///
            /// If the quotient were computed, he quotient and remainder would satisfy
            /// $x = q2^p + r$ and $0 \leq r < 2^p$.
            ///
            /// $$
            /// x \gets x - 2^p\operatorname{sgn}(x)\left \lfloor \frac{|x|}{2^p} \right \rfloor.
            /// $$
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::mod_power_of_2` module.
            #[inline]
            fn rem_power_of_2_assign(&mut self, pow: u64) {
                *self = self.rem_power_of_2(pow)
            }
        }

        impl CeilingModPowerOf2 for $s {
            type Output = $s;

            /// Divides a value by a power of 2, returning just the remainder. The remainder is
            /// non-positive.
            ///
            /// If the quotient were computed, the quotient and remainder would satisfy
            /// $x = q2^p + r$ and $0 \leq -r < 2^p$.
            ///
            /// $$
            /// f(x, y) =  x - 2^p\left \lceil \frac{x}{2^p} \right \rceil.
            /// $$
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `self` is positive or `$s::MIN`, and `pow` is greater than or equal to
            /// `$s::WIDTH`.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::mod_power_of_2` module.
            #[inline]
            fn ceiling_mod_power_of_2(self, pow: u64) -> $s {
                _ceiling_mod_power_of_2_signed::<$u, $s>(self, pow)
            }
        }

        impl CeilingModPowerOf2Assign for $s {
            /// Divides a value by a power of 2, replacing the first value by the remainder. The
            /// remainder has the opposite sign of the second value.
            ///
            /// If the quotient were computed, the quotient and remainder would satisfy
            /// $x = q2^p + r$ and $0 \leq -r < 2^p$.
            ///
            /// $$
            /// x \gets x - 2^p\left \lceil\frac{x}{2^p} \right \rceil.
            /// $$
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `self` is positive or `$s::MIN`, and `pow` is greater than or equal to
            /// `$s::WIDTH`.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::mod_power_of_2` module.
            #[inline]
            fn ceiling_mod_power_of_2_assign(&mut self, pow: u64) {
                *self = self.ceiling_mod_power_of_2(pow)
            }
        }
    };
}
apply_to_unsigned_signed_pairs!(impl_mod_power_of_2_signed);
