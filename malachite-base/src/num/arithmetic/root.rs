use num::arithmetic::sqrt::floor_inverse_checked_binary;
use num::arithmetic::traits::{
    CeilingRoot, CeilingRootAssign, CheckedRoot, DivRound, FloorRoot, FloorRootAssign, Parity,
    RootAssignRem, RootRem,
};
use num::basic::unsigneds::PrimitiveUnsigned;
use num::conversion::traits::WrappingFrom;
use rounding_modes::RoundingMode;

#[doc(hidden)]
pub fn _floor_root_binary<T: PrimitiveUnsigned>(x: T, exp: u64) -> T {
    if exp == 0 {
        panic!("Cannot take 0th root");
    } else if exp == 1 || x < T::TWO {
        x
    } else {
        let bits = x.significant_bits();
        if bits <= exp {
            T::ONE
        } else {
            let p = T::power_of_2(bits.div_round(exp, RoundingMode::Ceiling));
            floor_inverse_checked_binary(|i| i.checked_pow(exp), x, p >> 1, p)
        }
    }
}

#[doc(hidden)]
pub fn _ceiling_root_binary<T: PrimitiveUnsigned>(x: T, exp: u64) -> T {
    let floor_root = _floor_root_binary(x, exp);
    if floor_root.pow(exp) == x {
        floor_root
    } else {
        floor_root + T::ONE
    }
}

#[doc(hidden)]
pub fn _checked_root_binary<T: PrimitiveUnsigned>(x: T, exp: u64) -> Option<T> {
    let floor_root = _floor_root_binary(x, exp);
    if floor_root.pow(exp) == x {
        Some(floor_root)
    } else {
        None
    }
}

#[doc(hidden)]
pub fn _root_rem_binary<T: PrimitiveUnsigned>(x: T, exp: u64) -> (T, T) {
    let floor_root = _floor_root_binary(x, exp);
    (floor_root, x - floor_root.pow(exp))
}

macro_rules! impl_root_unsigned {
    ($t: ident) => {
        impl FloorRoot for $t {
            type Output = $t;

            /// Returns the floor of the $n$th root of an integer.
            ///
            /// $f(x, n) = \lfloor\sqrt\[n\]{x}\rfloor$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `exp` is zero.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::root` module.
            #[inline]
            fn floor_root(self, exp: u64) -> $t {
                _floor_root_binary(self, exp)
            }
        }

        impl CeilingRoot for $t {
            type Output = $t;

            /// Returns the ceiling of the $n$th root of an integer.
            ///
            /// $f(x) = \lceil\sqrt\[n\]{x}\rceil$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `exp` is zero.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::root` module.
            #[inline]
            fn ceiling_root(self, exp: u64) -> $t {
                _ceiling_root_binary(self, exp)
            }
        }

        impl CheckedRoot for $t {
            type Output = $t;

            /// Returns the the $n$th root of an integer, or `None` if the integer is not a perfect
            /// $n$th power.
            ///
            /// $$
            /// f(x) = \\begin{cases}
            ///     \operatorname{Some}(sqrt\[n\]{x}) & \sqrt\[n\]{x} \in \Z \\\\
            ///     \operatorname{None} & \textrm{otherwise},
            /// \\end{cases}
            /// $$
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `exp` is zero.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::root` module.
            #[inline]
            fn checked_root(self, exp: u64) -> Option<$t> {
                _checked_root_binary(self, exp)
            }
        }

        impl RootRem for $t {
            type RootOutput = $t;
            type RemOutput = $t;

            /// Returns the floor of the $n$th root of an integer, and the remainder (the
            /// difference between the integer and the $n$th power of the floor).
            ///
            /// $f(x, n) = (\lfloor\sqrt\[n\]{x}\rfloor, x - \lfloor\sqrt\[n\]{x}\rfloor^2)$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `exp` is zero.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::root` module.
            #[inline]
            fn root_rem(self, exp: u64) -> ($t, $t) {
                _root_rem_binary(self, exp)
            }
        }

        impl RootAssignRem for $t {
            type RemOutput = $t;

            /// Replaces an integer with the floor of its $n$th root, and returns the remainder
            /// (the difference between the original integer and the $n$th power of the floor).
            ///
            /// $f(x) = x - \lfloor\sqrt\[n\]{x}\rfloor^2$,
            ///
            /// $x \gets \lfloor\sqrt\[n\]{x}\rfloor$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `exp` is zero.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::root` module.
            #[inline]
            fn root_assign_rem(&mut self, exp: u64) -> $t {
                let (root, rem) = self.root_rem(exp);
                *self = root;
                rem
            }
        }
    };
}
apply_to_unsigneds!(impl_root_unsigned);

macro_rules! impl_root_signed {
    ($t: ident) => {
        impl FloorRoot for $t {
            type Output = $t;

            /// Returns the floor of the $n$th root of an integer.
            ///
            /// $f(x, n) = \lfloor\sqrt\[n\]{x}\rfloor$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `exp` is zero, or if `self` is negative and `exp` is even.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::root` module.
            #[inline]
            fn floor_root(self, exp: u64) -> $t {
                if self >= 0 {
                    $t::wrapping_from(_floor_root_binary(self.unsigned_abs(), exp))
                } else if exp.odd() {
                    -$t::wrapping_from(_ceiling_root_binary(self.unsigned_abs(), exp))
                } else {
                    panic!("Cannot take even root of a negative number");
                }
            }
        }

        impl CeilingRoot for $t {
            type Output = $t;

            /// Returns the ceiling of the $n$th root of an integer.
            ///
            /// $f(x) = \lceil\sqrt\[n\]{x}\rceil$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `exp` is zero, or if `self` is negative and `exp` is even.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::root` module.
            #[inline]
            fn ceiling_root(self, exp: u64) -> $t {
                if self >= 0 {
                    $t::wrapping_from(_ceiling_root_binary(self.unsigned_abs(), exp))
                } else if exp.odd() {
                    -$t::wrapping_from(_floor_root_binary(self.unsigned_abs(), exp))
                } else {
                    panic!("Cannot take even root of a negative number");
                }
            }
        }

        impl CheckedRoot for $t {
            type Output = $t;

            /// Returns the the $n$th root of an integer, or `None` if the integer is not a perfect
            /// $n$th power.
            ///
            /// $$
            /// f(x) = \\begin{cases}
            ///     \operatorname{Some}(sqrt\[n\]{x}) & \sqrt\[n\]{x} \in \Z \\\\
            ///     \operatorname{None} & \textrm{otherwise},
            /// \\end{cases}
            /// $$
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `exp` is zero, or if `self` is negative and `exp` is even.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::root` module.
            #[inline]
            fn checked_root(self, exp: u64) -> Option<$t> {
                if self >= 0 {
                    _checked_root_binary(self.unsigned_abs(), exp).map($t::wrapping_from)
                } else if exp.odd() {
                    _checked_root_binary(self.unsigned_abs(), exp).map(|x| -$t::wrapping_from(x))
                } else {
                    panic!("Cannot take even root of a negative number");
                }
            }
        }
    };
}
apply_to_signeds!(impl_root_signed);

macro_rules! impl_root_primitive_int {
    ($t: ident) => {
        impl FloorRootAssign for $t {
            /// Replaces an integer with the floor of its $n$th root.
            ///
            /// $x \gets \lfloor\sqrt\[n\]{x}\rfloor$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `exp` is zero, or if `self` is negative and `exp` is even.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::root` module.
            #[inline]
            fn floor_root_assign(&mut self, exp: u64) {
                *self = self.floor_root(exp);
            }
        }

        impl CeilingRootAssign for $t {
            /// Replaces an integer with the ceiling of its $n$th root.
            ///
            /// $x \gets \lceil\sqrt\[n\]{x}\rceil$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `exp` is zero, or if `self` is negative and `exp` is even.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::root` module.
            #[inline]
            fn ceiling_root_assign(&mut self, exp: u64) {
                *self = self.ceiling_root(exp);
            }
        }
    };
}
apply_to_primitive_ints!(impl_root_primitive_int);
