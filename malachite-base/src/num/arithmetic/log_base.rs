use num::arithmetic::traits::{
    CeilingLogBase, CeilingLogBasePowerOf2, CheckedLogBase, CheckedLogBase2,
    CheckedLogBasePowerOf2, CheckedMul, FloorLogBase, FloorLogBasePowerOf2,
};
use num::basic::traits::{One, Zero};
use std::fmt::Debug;

#[doc(hidden)]
pub fn _floor_log_base_naive<T: CheckedMul<T, Output = T> + Copy + Debug + Ord + One + Zero>(
    x: T,
    base: T,
) -> u64 {
    assert_ne!(x, T::ZERO);
    assert!(base > T::ONE);
    let mut result = 0;
    let mut p = T::ONE;
    // loop always executes at least once
    while p <= x {
        result += 1;
        if let Some(next_p) = p.checked_mul(base) {
            p = next_p;
        } else {
            break;
        }
    }
    result - 1
}

#[doc(hidden)]
pub fn _ceiling_log_base_naive<T: CheckedMul<T, Output = T> + Copy + Debug + Ord + One + Zero>(
    x: T,
    base: T,
) -> u64 {
    assert_ne!(x, T::ZERO);
    assert!(base > T::ONE);
    let mut result = 0;
    let mut p = T::ONE;
    while p < x {
        result += 1;
        if let Some(next_p) = p.checked_mul(base) {
            p = next_p;
        } else {
            break;
        }
    }
    result
}

#[doc(hidden)]
pub fn _checked_log_base_naive<T: CheckedMul<T, Output = T> + Copy + Debug + Ord + One + Zero>(
    x: T,
    base: T,
) -> Option<u64> {
    assert_ne!(x, T::ZERO);
    assert!(base > T::ONE);
    let mut result = 0;
    let mut p = T::ONE;
    while p < x {
        result += 1;
        if let Some(next_p) = p.checked_mul(base) {
            p = next_p;
        } else {
            return None;
        }
    }
    if p == x {
        Some(result)
    } else {
        None
    }
}

fn _floor_log_base<
    T: CheckedLogBase2
        + CheckedMul<T, Output = T>
        + Copy
        + Debug
        + FloorLogBasePowerOf2
        + Ord
        + One
        + Zero,
>(
    x: T,
    base: T,
) -> u64 {
    if let Some(log_base) = base.checked_log_base_2() {
        x.floor_log_base_power_of_2(log_base)
    } else {
        _floor_log_base_naive(x, base)
    }
}

fn _ceiling_log_base<
    T: CheckedLogBase2
        + CheckedMul<T, Output = T>
        + Copy
        + Debug
        + CeilingLogBasePowerOf2
        + Ord
        + One
        + Zero,
>(
    x: T,
    base: T,
) -> u64 {
    if let Some(log_base) = base.checked_log_base_2() {
        x.ceiling_log_base_power_of_2(log_base)
    } else {
        _ceiling_log_base_naive(x, base)
    }
}

fn _checked_log_base<
    T: CheckedLogBase2
        + CheckedMul<T, Output = T>
        + Copy
        + Debug
        + CheckedLogBasePowerOf2
        + Ord
        + One
        + Zero,
>(
    x: T,
    base: T,
) -> Option<u64> {
    if let Some(log_base) = base.checked_log_base_2() {
        x.checked_log_base_power_of_2(log_base)
    } else {
        _checked_log_base_naive(x, base)
    }
}

macro_rules! impl_arithmetic_traits {
    ($t:ident) => {
        impl FloorLogBase for $t {
            /// Returns the floor of the base-$b$ logarithm of a positive integer.
            ///
            /// $f(x, b) = \lfloor\log_b x\rfloor$.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is
            /// `self.significant_bits() / base.significant_bits()`.
            ///
            /// # Panics
            /// Panics if `self` is 0 or `base` is less than 2.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::log_base` module.
            #[inline]
            fn floor_log_base(self, base: $t) -> u64 {
                _floor_log_base(self, base)
            }
        }

        impl CeilingLogBase for $t {
            /// Returns the ceiling of the base-$b$ logarithm of a positive integer.
            ///
            /// $f(x, b) = \lceil\log_b x\rceil$.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is
            /// `self.significant_bits() / base.significant_bits()`.
            ///
            /// # Panics
            /// Panics if `self` is 0 or `base` is less than 2.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::log_base` module.
            #[inline]
            fn ceiling_log_base(self, base: $t) -> u64 {
                _ceiling_log_base(self, base)
            }
        }

        impl CheckedLogBase for $t {
            /// Returns the base-$b$ logarithm of a positive integer. If the integer is not a
            /// power of $b$, `None` is returned.
            ///
            /// $$
            /// f(x, b) = \\begin{cases}
            ///     \operatorname{Some}(\log_b x) & \log_b x \in \Z \\\\
            ///     \operatorname{None} & \textrm{otherwise},
            /// \\end{cases}
            /// $$
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is
            /// `self.significant_bits() / base.significant_bits()`.
            ///
            /// # Panics
            /// Panics if `self` is 0 or `base` is less than 2.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::log_base` module.
            #[inline]
            fn checked_log_base(self, base: $t) -> Option<u64> {
                _checked_log_base(self, base)
            }
        }
    };
}
apply_to_unsigneds!(impl_arithmetic_traits);
