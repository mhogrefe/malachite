#[cfg(not(feature = "std"))]
#[macro_export]
macro_rules! fma {
    ($a: expr, $b: expr, $c: expr) => {{ libm::fma($a, $b, $c) }};
}

#[cfg(feature = "std")]
#[macro_export]
macro_rules! fma {
    ($a: expr, $b: expr, $c: expr) => {{ $a.mul_add($b, $c) }};
}

pub use fma;

#[cfg(not(feature = "std"))]
#[macro_export]
macro_rules! round_even {
    ($a: expr) => {{ libm::roundeven($a) }};
}

#[cfg(feature = "std")]
#[macro_export]
macro_rules! round_even {
    ($a: expr) => {{ $a.round_ties_even() }};
}

pub use round_even;
