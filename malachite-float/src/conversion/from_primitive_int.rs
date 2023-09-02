use crate::Float;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::rounding_modes::RoundingMode;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use std::cmp::Ordering;

impl Float {
    #[doc(hidden)]
    #[inline]
    pub fn from_unsigned_times_power_of_2<T: PrimitiveUnsigned>(x: T, pow: i64) -> Float
    where
        Natural: From<T>,
    {
        Float::from_natural_times_power_of_2(Natural::from(x), pow)
    }

    #[doc(hidden)]
    #[inline]
    pub fn from_unsigned_times_power_of_2_prec_round<T: PrimitiveUnsigned>(
        x: T,
        pow: i64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Float, Ordering)
    where
        Natural: From<T>,
    {
        Float::from_natural_times_power_of_2_prec_round(Natural::from(x), pow, prec, rm)
    }

    #[doc(hidden)]
    #[inline]
    pub fn from_unsigned_times_power_of_2_prec<T: PrimitiveUnsigned>(
        x: T,
        pow: i64,
        prec: u64,
    ) -> (Float, Ordering)
    where
        Natural: From<T>,
    {
        Float::from_natural_times_power_of_2_prec(Natural::from(x), pow, prec)
    }

    /// Converts a primitive unsigned integer to a [`Float`]. If the [`Float`] is nonzero, it has
    /// the specified precision. If rounding is needed, the specified rounding mode is used. An
    /// [`Ordering`] is also returned, indicating whether the returned value is less than, equal
    /// to, or greater than the original value.
    ///
    /// If you're only using [`RoundingMode::Nearest`], try using [`Float::from_unsigned_prec`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Examples
    /// See [here](super::from_primitive_int#from_unsigned_prec_round).
    #[inline]
    pub fn from_unsigned_prec_round<T: PrimitiveUnsigned>(
        x: T,
        prec: u64,
        rm: RoundingMode,
    ) -> (Float, Ordering)
    where
        Natural: From<T>,
    {
        Float::from_natural_prec_round(Natural::from(x), prec, rm)
    }

    /// Converts an unsigned primitive integer to a [`Float`]. If the [`Float`] is nonzero, it has
    /// the specified precision. An [`Ordering`] is also returned, indicating whether the returned
    /// value is less than, equal to, or greater than the original value.
    ///
    /// If you want the [`Float`]'s precision to be equal to the integer's number of significant
    /// bits, try just using `Float::from` instead.
    ///
    /// Rounding may occur, in which case [`RoundingMode::Nearest`] is used by default. To specify
    /// a rounding mode as well as a precision, try [`Float::from_unsigned_prec_round`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Examples
    /// See [here](super::from_primitive_int#from_unsigned_prec).
    #[inline]
    pub fn from_unsigned_prec<T: PrimitiveUnsigned>(x: T, prec: u64) -> (Float, Ordering)
    where
        Natural: From<T>,
    {
        Float::from_natural_prec(Natural::from(x), prec)
    }

    #[doc(hidden)]
    #[inline]
    pub fn from_signed_times_power_of_2<T: PrimitiveSigned>(x: T, pow: i64) -> Float
    where
        Integer: From<T>,
    {
        Float::from_integer_times_power_of_2(Integer::from(x), pow)
    }

    #[doc(hidden)]
    #[inline]
    pub fn from_signed_times_power_of_2_prec_round<T: PrimitiveSigned>(
        x: T,
        pow: i64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Float, Ordering)
    where
        Integer: From<T>,
    {
        Float::from_integer_times_power_of_2_prec_round(Integer::from(x), pow, prec, rm)
    }

    #[doc(hidden)]
    #[inline]
    pub fn from_signed_times_power_of_2_prec<T: PrimitiveSigned>(
        x: T,
        pow: i64,
        prec: u64,
    ) -> (Float, Ordering)
    where
        Integer: From<T>,
    {
        Float::from_integer_times_power_of_2_prec(Integer::from(x), pow, prec)
    }

    /// Converts a primitive signed integer to a [`Float`]. If the [`Float`] is nonzero, it has the
    /// specified precision. If rounding is needed, the specified rounding mode is used. An
    /// [`Ordering`] is also returned, indicating whether the returned value is less than, equal
    /// to, or greater than the original value.
    ///
    /// If you're only using [`RoundingMode::Nearest`], try using [`Float::from_signed_prec`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Examples
    /// See [here](super::from_primitive_int#from_signed_prec_round).
    #[inline]
    pub fn from_signed_prec_round<T: PrimitiveSigned>(
        x: T,
        prec: u64,
        rm: RoundingMode,
    ) -> (Float, Ordering)
    where
        Integer: From<T>,
    {
        Float::from_integer_prec_round(Integer::from(x), prec, rm)
    }

    /// Converts a signed primitive integer to a [`Float`]. If the [`Float`] is nonzero, it has the
    /// specified precision. An [`Ordering`] is also returned, indicating whether the returned
    /// value is less than, equal to, or greater than the original value.
    ///
    /// If you want the [`Float`]'s precision to be equal to the integer's number of significant
    /// bits, try just using `Float::from` instead.
    ///
    /// Rounding may occur, in which case [`RoundingMode::Nearest`] is used by default. To specify
    /// a rounding mode as well as a precision, try [`Float::from_signed_prec_round`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Examples
    /// See [here](super::from_primitive_int#from_signed_prec).
    #[inline]
    pub fn from_signed_prec<T: PrimitiveSigned>(x: T, prec: u64) -> (Float, Ordering)
    where
        Integer: From<T>,
    {
        Float::from_integer_prec(Integer::from(x), prec)
    }
}

macro_rules! impl_from_unsigned {
    ($t: ident) => {
        impl From<$t> for Float {
            /// Converts an unsigned primitive integer to a [`Float`].
            ///
            /// If the integer is nonzero, the precision of the [`Float`] is equal to the integer's
            /// number of significant bits. If you want to specify a different precision, try
            /// [`Float::from_unsigned_prec`]. This may require rounding, which uses
            /// [`RoundingMode::Nearest`] by default. To specify a rounding mode as well as a
            /// precision, try [`Float::from_unsigned_prec_round`].
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::from_primitive_int#from).
            #[inline]
            fn from(u: $t) -> Float {
                Float::from(Natural::from(u))
            }
        }
    };
}
apply_to_unsigneds!(impl_from_unsigned);

macro_rules! impl_from_signed {
    ($t: ident) => {
        impl From<$t> for Float {
            /// Converts a signed primitive integer to a [`Float`].
            ///
            /// If the integer is nonzero, the precision of the [`Float`] is equal to the integer's
            /// number of significant bits. If you want to specify a different precision, try
            /// [`Float::from_signed_prec`]. This may require rounding, which uses
            /// [`RoundingMode::Nearest`] by default. To specify a rounding mode as well as a
            /// precision, try [`Float::from_signed_prec_round`].
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::from_primitive_int#from).
            #[inline]
            fn from(i: $t) -> Float {
                Float::from(Integer::from(i))
            }
        }
    };
}
apply_to_signeds!(impl_from_signed);
