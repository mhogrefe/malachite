macro_rules! apply_to_unsigneds {
    ($m:tt{_$(,$args:ident)*}) => {
        $m!(u8 $(,$args)*);
        $m!(u16 $(,$args)*);
        $m!(u32 $(,$args)*);
        $m!(u64 $(,$args)*);
        $m!(usize $(,$args)*);
        $m!(u128 $(,$args)*);
    };
    ($m:tt{$arg0:tt,_$(,$args:tt)*}) => {
        $m!($arg0, u8 $(,$args)*);
        $m!($arg0, u16 $(,$args)*);
        $m!($arg0, u32 $(,$args)*);
        $m!($arg0, u64 $(,$args)*);
        $m!($arg0, usize $(,$args)*);
        $m!($arg0, u128 $(,$args)*);
    }
}

macro_rules! apply_to_signeds {
    ($m:tt{_$(,$args:ident)*}) => {
        $m!(i8 $(,$args)*);
        $m!(i16 $(,$args)*);
        $m!(i32 $(,$args)*);
        $m!(i64 $(,$args)*);
        $m!(isize $(,$args)*);
        $m!(i128 $(,$args)*);
    };
    ($m:tt{$arg0:tt,_$(,$args:tt)*}) => {
        $m!($arg0, i8 $(,$args)*);
        $m!($arg0, i16 $(,$args)*);
        $m!($arg0, i32 $(,$args)*);
        $m!($arg0, i64 $(,$args)*);
        $m!($arg0, isize $(,$args)*);
        $m!($arg0, i128 $(,$args)*);
    }
}

macro_rules! apply_to_primitives {
    ($m:tt{_$(,$args:ident)*}) => {
        apply_to_signeds!($m{_$(,$args)*});
        apply_to_unsigneds!($m{_$(,$args)*});
    };
    ($m:tt{$arg0:tt,_$(,$args:tt)*}) => {
        apply_to_signeds!($m{$arg0, _$(,$args)*});
        apply_to_unsigneds!($m{$arg0, _$(,$args)*});
    };
}

macro_rules! forward_from {
    ($lhs:ty, $rhs:ty) => {
        impl From<$rhs> for $lhs {
            #[inline]
            fn from(value: $rhs) -> Self {
                Self(<_ as From<_>>::from(value))
            }
        }
    };
}

macro_rules! forward_try_from {
    ($lhs:ty, $rhs:ty) => {
        impl TryFrom<$rhs> for $lhs {
            type Error = TryFromBigIntError<()>;

            #[inline]
            fn try_from(value: $rhs) -> Result<Self, Self::Error> {
                <_ as TryFrom<_>>::try_from(value)
                    .map_err(|_| Self::Error::new(()))
                    .map(Self)
            }
        }
    };
}

macro_rules! forward_try_into {
    ($res:ty, $t:ty) => {
        impl TryFrom<$res> for $t {
            type Error = TryFromBigIntError<$res>;

            #[inline]
            fn try_from(value: $res) -> Result<Self, Self::Error> {
                <$t>::try_from(&value.0).map_err(|_| Self::Error::new(value))
            }
        }

        impl TryFrom<&$res> for $t {
            type Error = TryFromBigIntError<()>;

            #[inline]
            fn try_from(value: &$res) -> Result<Self, Self::Error> {
                <$t>::try_from(&value.0).map_err(|_| Self::Error::new(()))
            }
        }
    };
}

macro_rules! forward_unary_op {
    ($struct:tt, $trait:tt, $fn:ident) => {
        impl $trait for $struct {
            type Output = $struct;

            #[inline]
            fn $fn(mut self) -> Self::Output {
                self.0 = $trait::$fn(self.0);
                self
            }
        }

        impl $trait for &$struct {
            type Output = $struct;

            #[inline]
            fn $fn(self) -> Self::Output {
                $struct($trait::$fn(&self.0))
            }
        }
    };
}

macro_rules! impl_binary_op {
    ($lhs:ty, $rhs:ty, $output:ty, $trait:tt, $fn:ident, $expr:expr) => {
        impl $trait<$rhs> for $lhs {
            type Output = $output;

            #[inline]
            fn $fn(self, rhs: $rhs) -> Self::Output {
                $expr(self, rhs)
            }
        }
    };
}

macro_rules! impl_assign_op {
    ($lhs:ty, $rhs:ty, $trait:tt, $fn:ident, $expr:expr) => {
        impl $trait<$rhs> for $lhs {
            #[inline]
            fn $fn(&mut self, rhs: $rhs) {
                $expr(self, rhs)
            }
        }
    };
}

macro_rules! forward_binary_self {
    ($struct:ty, $trait:tt, $fn:ident) => {
        impl_binary_op!(
            $struct,
            $struct,
            $struct,
            $trait,
            $fn,
            |lhs: $struct, rhs: $struct| { $trait::$fn(lhs.0, rhs.0).into() }
        );
        impl_binary_op!(
            &$struct,
            $struct,
            $struct,
            $trait,
            $fn,
            |lhs: &$struct, rhs: $struct| { $trait::$fn(&lhs.0, rhs.0).into() }
        );
        impl_binary_op!(
            $struct,
            &$struct,
            $struct,
            $trait,
            $fn,
            |lhs: $struct, rhs: &$struct| { $trait::$fn(lhs.0, &rhs.0).into() }
        );
        impl_binary_op!(
            &$struct,
            &$struct,
            $struct,
            $trait,
            $fn,
            |lhs: &$struct, rhs: &$struct| { $trait::$fn(&lhs.0, &rhs.0).into() }
        );
    };
}

macro_rules! forward_binary_right_primitive {
    ($lhs:ty, $rhs:ty, $trait:tt, $fn:ident) => {
        impl_binary_op!($lhs, $rhs, $lhs, $trait, $fn, |lhs: $lhs, rhs: $rhs| {
            $trait::$fn(lhs.0, rhs).into()
        });
        impl_binary_op!(&$lhs, $rhs, $lhs, $trait, $fn, |lhs: &$lhs, rhs: $rhs| {
            $trait::$fn(&lhs.0, rhs).into()
        });
        impl_binary_op!($lhs, &$rhs, $lhs, $trait, $fn, |lhs: $lhs, rhs: &$rhs| {
            $trait::$fn(lhs.0, *rhs).into()
        });
        impl_binary_op!(&$lhs, &$rhs, $lhs, $trait, $fn, |lhs: &$lhs, rhs: &$rhs| {
            $trait::$fn(&lhs.0, *rhs).into()
        });
    };
}

macro_rules! forward_binary_right_primitive_into {
    ($lhs:ty, $rhs:ty, $trait:tt, $fn:ident) => {
        impl_binary_op!($lhs, $rhs, $lhs, $trait, $fn, |lhs: $lhs, rhs: $rhs| {
            $trait::$fn(lhs.0, <$lhs>::from(rhs).0).into()
        });
        impl_binary_op!(&$lhs, $rhs, $lhs, $trait, $fn, |lhs: &$lhs, rhs: $rhs| {
            $trait::$fn(&lhs.0, <$lhs>::from(rhs).0).into()
        });
        impl_binary_op!($lhs, &$rhs, $lhs, $trait, $fn, |lhs: $lhs, rhs: &$rhs| {
            $trait::$fn(lhs.0, <$lhs>::from(*rhs).0).into()
        });
        impl_binary_op!(&$lhs, &$rhs, $lhs, $trait, $fn, |lhs: &$lhs, rhs: &$rhs| {
            $trait::$fn(&lhs.0, <$lhs>::from(*rhs).0).into()
        });
    };
}

macro_rules! forward_binary_left_primitive_into {
    ($lhs:ty, $rhs:ty, $trait:tt, $fn:ident) => {
        impl_binary_op!($lhs, $rhs, $rhs, $trait, $fn, |lhs: $lhs, rhs: $rhs| {
            $trait::$fn(<$rhs>::from(lhs).0, rhs.0).into()
        });
        impl_binary_op!(&$lhs, $rhs, $rhs, $trait, $fn, |lhs: &$lhs, rhs: $rhs| {
            $trait::$fn(<$rhs>::from(*lhs).0, rhs.0).into()
        });
        impl_binary_op!($lhs, &$rhs, $rhs, $trait, $fn, |lhs: $lhs, rhs: &$rhs| {
            $trait::$fn(<$rhs>::from(lhs).0, &rhs.0).into()
        });
        impl_binary_op!(&$lhs, &$rhs, $rhs, $trait, $fn, |lhs: &$lhs, rhs: &$rhs| {
            $trait::$fn(<$rhs>::from(*lhs).0, &rhs.0).into()
        });
    };
}

macro_rules! forward_assign_self {
    ($struct:ty, $trait:tt, $fn:ident) => {
        impl_assign_op!(
            $struct,
            $struct,
            $trait,
            $fn,
            |lhs: &mut $struct, rhs: $struct| { $trait::$fn(&mut lhs.0, rhs.0).into() }
        );
        impl_assign_op!(
            $struct,
            &$struct,
            $trait,
            $fn,
            |lhs: &mut $struct, rhs: &$struct| { $trait::$fn(&mut lhs.0, &rhs.0).into() }
        );
    };
}

macro_rules! forward_assign_primitive {
    ($lhs:ty, $rhs:ty, $trait:tt, $fn:ident) => {
        impl_assign_op!($lhs, $rhs, $trait, $fn, |lhs: &mut $lhs, rhs: $rhs| {
            $trait::$fn(&mut lhs.0, rhs).into()
        });
        impl_assign_op!($lhs, &$rhs, $trait, $fn, |lhs: &mut $lhs, rhs: &$rhs| {
            $trait::$fn(&mut lhs.0, *rhs).into()
        });
    };
}

macro_rules! forward_assign_primitive_into {
    ($lhs:ty, $rhs:ty, $trait:tt, $fn:ident) => {
        impl_assign_op!($lhs, $rhs, $trait, $fn, |lhs: &mut $lhs, rhs: $rhs| {
            $trait::$fn(&mut lhs.0, <$lhs>::from(rhs).0).into()
        });
        impl_assign_op!($lhs, &$rhs, $trait, $fn, |lhs: &mut $lhs, rhs: &$rhs| {
            $trait::$fn(&mut lhs.0, <$lhs>::from(*rhs).0).into()
        });
    };
}

macro_rules! forward_pow_primitive {
    ($lhs:ty, $rhs:ty) => {
        impl_binary_op!($lhs, $rhs, $lhs, Pow, pow, |lhs: $lhs, rhs: $rhs| {
            <_ as malachite::num::arithmetic::traits::Pow<u64>>::pow(lhs.0, rhs as _).into()
        });
        impl_binary_op!(&$lhs, $rhs, $lhs, Pow, pow, |lhs: &$lhs, rhs: $rhs| {
            <_ as malachite::num::arithmetic::traits::Pow<u64>>::pow(&lhs.0, rhs as _).into()
        });
        impl_binary_op!($lhs, &$rhs, $lhs, Pow, pow, |lhs: $lhs, rhs: &$rhs| {
            <_ as malachite::num::arithmetic::traits::Pow<u64>>::pow(lhs.0, *rhs as _).into()
        });
        impl_binary_op!(&$lhs, &$rhs, $lhs, Pow, pow, |lhs: &$lhs, rhs: &$rhs| {
            <_ as malachite::num::arithmetic::traits::Pow<u64>>::pow(&lhs.0, *rhs as _).into()
        });
    };
}

macro_rules! forward_pow_biguint {
    ($lhs:ty) => {
        impl_binary_op!($lhs, BigUint, $lhs, Pow, pow, |lhs: $lhs, rhs: BigUint| {
            let exp = malachite::num::conversion::traits::SaturatingFrom::saturating_from(&rhs.0);
            <_ as malachite::num::arithmetic::traits::Pow<u64>>::pow(lhs.0, exp).into()
        });
        impl_binary_op!(
            &$lhs,
            BigUint,
            $lhs,
            Pow,
            pow,
            |lhs: &$lhs, rhs: BigUint| {
                let exp =
                    malachite::num::conversion::traits::SaturatingFrom::saturating_from(&rhs.0);
                <_ as malachite::num::arithmetic::traits::Pow<u64>>::pow(&lhs.0, exp).into()
            }
        );
        impl_binary_op!(
            $lhs,
            &BigUint,
            $lhs,
            Pow,
            pow,
            |lhs: $lhs, rhs: &BigUint| {
                let exp =
                    malachite::num::conversion::traits::SaturatingFrom::saturating_from(&rhs.0);
                <_ as malachite::num::arithmetic::traits::Pow<u64>>::pow(lhs.0, exp).into()
            }
        );
        impl_binary_op!(
            &$lhs,
            &BigUint,
            $lhs,
            Pow,
            pow,
            |lhs: &$lhs, rhs: &BigUint| {
                let exp =
                    malachite::num::conversion::traits::SaturatingFrom::saturating_from(&rhs.0);
                <_ as malachite::num::arithmetic::traits::Pow<u64>>::pow(&lhs.0, exp).into()
            }
        );
    };
}

macro_rules! impl_sum_iter_type {
    ($res:ty) => {
        impl<T> Sum<T> for $res
        where
            $res: Add<T, Output = $res>,
        {
            #[inline]
            fn sum<I>(iter: I) -> Self
            where
                I: Iterator<Item = T>,
            {
                iter.fold(Zero::zero(), <$res>::add)
            }
        }
    };
}

macro_rules! impl_product_iter_type {
    ($res:ty) => {
        impl<T> Product<T> for $res
        where
            $res: Mul<T, Output = $res>,
        {
            #[inline]
            fn product<I>(iter: I) -> Self
            where
                I: Iterator<Item = T>,
            {
                iter.fold(One::one(), <$res>::mul)
            }
        }
    };
}

macro_rules! impl_from_primitive_fn_infallible {
    ($t:ty) => {
        paste! {
            #[inline]
            fn [<from_ $t>](n: $t) -> Option<Self> {
                Some(Self::from(n))
            }
        }
    };
}

macro_rules! impl_from_primitive_fn_try_from {
    ($t:ty) => {
        paste! {
            #[inline]
            fn [<from_ $t>](n: $t) -> Option<Self> {
                Self::try_from(n).ok()
            }
        }
    };
}

macro_rules! impl_from_primitive_fn_float {
    ($t:ty) => {
        paste! {
            #[inline]
            fn [<from_ $t>](n: $t) -> Option<Self> {
                if !n.is_finite() {
                    return None;
                }
                Some(Self(n.rounding_into(RoundingMode::Down).0))
            }
        }
    };
}

macro_rules! impl_primitive_convert {
    ($res:ty, $t:ty) => {
        paste! {
            impl [<To $res>] for $t {
                #[inline]
                fn [<to_ $res:lower>](&self) -> Option<$res> {
                    $res::[<from_ $t>](*self)
                }
            }
        }
    };
}

macro_rules! impl_to_primitive_fn_try_into {
    ($t:ty) => {
        paste! {
            #[inline]
            fn [<to_ $t>](&self)-> Option<$t> {
                self.try_into().ok()
            }
        }
    };
}

macro_rules! impl_to_primitive_fn_float {
    ($t:ty) => {
        paste! {
            #[inline]
            fn [<to_ $t>](&self) -> Option<$t> {
                match (&self.0).rounding_into(RoundingMode::Nearest) {
                    // returned value is $t::MAX but still less than the original
                    (val, std::cmp::Ordering::Less) if val == $t::MAX => None,
                    // returned value is $t::MIN but still greater than the original
                    (val, std::cmp::Ordering::Greater) if val == $t::MIN => None,
                    (val, _) => Some(val),
                }
            }
        }
    };
}
