use round::RoundingMode;
use std::cmp::Ordering;

pub trait AbsAssign {
    fn abs_assign(&mut self);
}

pub trait Assign<Rhs = Self> {
    fn assign(&mut self, rhs: Rhs);
}

pub trait NegAssign {
    fn neg_assign(&mut self);
}

pub trait NotAssign {
    fn not_assign(&mut self);
}

pub trait AddMulAssign<B, C> {
    // Equivalent to self += b * c
    fn add_mul_assign(&mut self, b: B, c: C);
}

pub trait AddMul<B, C> {
    type Output;

    // Equivalent to self + b * c
    fn add_mul(self, b: B, c: C) -> Self::Output;
}

pub trait SubMulAssign<B, C> {
    // Equivalent to self -= b * c
    fn sub_mul_assign(&mut self, b: B, c: C);
}

pub trait SubMul<B, C> {
    type Output;

    // Equivalent to self - b * c
    fn sub_mul(self, b: B, c: C) -> Self::Output;
}

pub trait PartialOrdAbs<Rhs: ?Sized = Self>: PartialEq<Rhs> {
    fn partial_cmp_abs(&self, other: &Rhs) -> Option<Ordering>;

    fn lt_abs(&self, other: &Rhs) -> bool {
        match self.partial_cmp_abs(other) {
            Some(Ordering::Less) => true,
            _ => false,
        }
    }

    fn le_abs(&self, other: &Rhs) -> bool {
        match self.partial_cmp_abs(other) {
            Some(Ordering::Less) | Some(Ordering::Equal) => true,
            _ => false,
        }
    }

    fn gt_abs(&self, other: &Rhs) -> bool {
        match self.partial_cmp_abs(other) {
            Some(Ordering::Greater) => true,
            _ => false,
        }
    }

    fn ge_abs(&self, other: &Rhs) -> bool {
        match self.partial_cmp_abs(other) {
            Some(Ordering::Greater) | Some(Ordering::Equal) => true,
            _ => false,
        }
    }
}

pub trait OrdAbs: Eq + PartialOrdAbs<Self> {
    fn cmp_abs(&self, other: &Self) -> Ordering;
}

pub trait Zero {
    const ZERO: Self;
}

pub trait One {
    const ONE: Self;
}

pub trait Two {
    const TWO: Self;
}

pub trait NegativeOne {
    const NEGATIVE_ONE: Self;
}

macro_rules! impl01u {
    ($t: ty) => {
        impl Zero for $t {
            const ZERO: $t = 0;
        }

        impl One for $t {
            const ONE: $t = 1;
        }

        impl Two for $t {
            const TWO: $t = 2;
        }
    }
}

macro_rules! impl01i {
    ($t: ty) => {
        impl01u!($t);

        impl NegativeOne for $t {
            const NEGATIVE_ONE: $t = -1;
        }
    }
}

macro_rules! impl01f {
    ($t: ty) => {
        impl Zero for $t {
            const ZERO: $t = 0.0;
        }

        impl One for $t {
            const ONE: $t = 1.0;
        }

        impl Two for $t {
            const TWO: $t = 2.0;
        }

        impl NegativeOne for $t {
            const NEGATIVE_ONE: $t = -1.0;
        }
    }
}

impl01u!(u8);
impl01u!(u16);
impl01u!(u32);
impl01u!(u64);
impl01u!(usize);

impl01i!(i8);
impl01i!(i16);
impl01i!(i32);
impl01i!(i64);
impl01i!(isize);

impl01f!(f32);
impl01f!(f64);

pub trait ShlRound<RHS> {
    type Output;

    fn shl_round(self, rhs: RHS, rm: RoundingMode) -> Self::Output;
}

pub trait ShrRound<RHS> {
    type Output;

    fn shr_round(self, rhs: RHS, rm: RoundingMode) -> Self::Output;
}

pub trait ShlRoundAssign<Rhs = Self> {
    fn shl_round_assign(&mut self, rhs: Rhs, rm: RoundingMode);
}

pub trait ShrRoundAssign<Rhs = Self> {
    fn shr_round_assign(&mut self, rhs: Rhs, rm: RoundingMode);
}
