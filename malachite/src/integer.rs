#[cfg(feature = "gmp")]
pub use malachite_gmp::integer::Integer;
#[cfg(feature = "native")]
pub use malachite_native::integer::Integer;
