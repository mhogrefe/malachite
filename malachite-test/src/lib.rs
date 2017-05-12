extern crate num;
extern crate rugint;

pub mod integer {
    pub mod conversion {
        pub mod assign_i32;
        pub mod assign_u32;
    }
    pub mod comparison {
        pub mod partial_cmp_i32;
        pub mod partial_cmp_u32;
        pub mod partial_eq_i32;
        pub mod partial_eq_u32;
        pub mod sign;
    }
}
pub mod natural {
    pub mod arithmetic {
        pub mod add_u32;
    }
    pub mod comparison {
        pub mod partial_cmp_u32;
        pub mod partial_eq_u32;
    }
    pub mod conversion {
        pub mod assign_u32;
    }
    pub mod logic {
        pub mod get_bit;
    }
}
