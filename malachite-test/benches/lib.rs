#![feature(test)]

extern crate malachite_gmp;
extern crate malachite_native;
extern crate malachite_test;
extern crate num;
extern crate rugint;

extern crate test;

pub mod integer {
    pub mod basic {
        pub mod new;
    }
    pub mod conversion {
        pub mod clone;
        pub mod from_i32;
        pub mod from_u32;
    }
    pub mod comparison {
        pub mod partial_eq_i32;
        pub mod partial_eq_u32;
        pub mod sign;
    }
    pub mod logic {
        pub mod significant_bits;
    }
}
pub mod natural {
    pub mod basic {
        pub mod new;
    }
    pub mod comparison {
        pub mod partial_eq_u32;
    }
    pub mod conversion {
        pub mod clone;
        pub mod from_u32;
    }
    pub mod logic {
        pub mod limb_count;
        pub mod limbs_le;
        pub mod significant_bits;
    }
}
