#[cfg(feature = "gmp")]
extern crate malachite_gmp;
#[cfg(feature = "native")]
extern crate malachite_native;
extern crate rand;

pub mod natural {
    #[cfg(feature = "gmp")]
    pub use malachite_gmp::natural::Natural;
    #[cfg(feature = "native")]
    pub use malachite_native::natural::Natural;
    pub mod random {
        pub mod random_natural_from_bits;
        pub mod random_natural_up_to_bits;
        pub mod random_natural_below;
    }
}
pub mod integer {
    #[cfg(feature = "gmp")]
    pub use malachite_gmp::integer::Integer;
    #[cfg(feature = "native")]
    pub use malachite_native::integer::Integer;
}
