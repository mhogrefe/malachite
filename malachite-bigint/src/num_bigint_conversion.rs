use crate::{
    BigInt, BigUint,
    Sign::{self, Minus, NoSign, Plus},
};
use malachite::Natural;
use num_traits::ToPrimitive;

impl From<num_bigint::BigUint> for BigUint {
    #[inline]
    fn from(value: num_bigint::BigUint) -> Self {
        if let Some(x) = value.to_u64() {
            Self::from(x)
        } else {
            let limbs = value.to_u64_digits();
            Self(Natural::from_owned_limbs_asc(limbs))
        }
    }
}

impl From<BigUint> for num_bigint::BigUint {
    #[inline]
    fn from(value: BigUint) -> Self {
        if let Some(x) = value.to_u64() {
            Self::from(x)
        } else {
            let digits = value.to_u32_digits();
            Self::new(digits)
        }
    }
}

impl From<num_bigint::Sign> for Sign {
    #[inline]
    fn from(value: num_bigint::Sign) -> Self {
        match value {
            num_bigint::Sign::Minus => Minus,
            num_bigint::Sign::NoSign => NoSign,
            num_bigint::Sign::Plus => Plus,
        }
    }
}

impl From<Sign> for num_bigint::Sign {
    #[inline]
    fn from(value: Sign) -> Self {
        match value {
            Minus => num_bigint::Sign::Minus,
            NoSign => num_bigint::Sign::NoSign,
            Plus => num_bigint::Sign::Plus,
        }
    }
}

impl From<num_bigint::BigInt> for BigInt {
    #[inline]
    fn from(value: num_bigint::BigInt) -> Self {
        let (sign, abs) = value.into_parts();
        Self::from_biguint(sign.into(), abs.into())
    }
}

impl From<BigInt> for num_bigint::BigInt {
    #[inline]
    fn from(value: BigInt) -> Self {
        let (sign, abs) = value.into_parts();
        Self::from_biguint(sign.into(), abs.into())
    }
}

#[test]
fn num_bigint_conversion_test() {
    let tester = |val: BigInt| {
        let numval = num_bigint::BigInt::from(val.clone());
        let val2 = BigInt::from(numval);
        assert_eq!(val, val2);
    };

    tester(50723.into());
    tester(BigInt::from(-52321));
    tester(0.into());
    tester(BigInt::from(std::u64::MAX).pow(1000u32));
    tester(BigInt::from(std::i64::MIN).pow(1000u32));
}
