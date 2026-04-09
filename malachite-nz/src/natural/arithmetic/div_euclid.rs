use malachite_base::num::{
    arithmetic::traits::DivMod,
    basic::traits::{One, Zero},
};

use crate::natural::Natural;

pub trait DivEuclidean: Sized {
    fn div_rem_euclid(self, rhs: Self) -> (Self, Self);

    fn div_rem_euclid_ref(self, rhs: &Self) -> (Self, Self);
}

impl DivEuclidean for Natural {
    fn div_rem_euclid(self, rhs: Self) -> (Self, Self) {
        assert_ne!(rhs, Self::ZERO, "division by zero");

        let (mut q, mut r) = self.div_mod(&rhs);

        if r < Self::ZERO {
            let abs_rhs = &rhs;

            r += abs_rhs;

            if rhs > Self::ZERO {
                q -= Self::ONE;
            } else {
                q += Self::ONE;
            }
        }

        (q, r)
    }
    fn div_rem_euclid_ref(self, rhs: &Self) -> (Self, Self) {
        assert_ne!(*rhs, Self::ZERO, "division by zero");

        let (mut q, mut r) = self.div_mod(rhs);

        if r < Self::ZERO {
            let abs_rhs = rhs;

            r += abs_rhs;

            if *rhs > Self::ZERO {
                q -= Self::ONE;
            } else {
                q += Self::ONE;
            }
        }

        (q, r)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
#[allow(clippy::panic)]
#[allow(clippy::expect_used)]
mod test {
    use crate::natural::{Natural, arithmetic::div_euclid::DivEuclidean};

    #[test]
    fn t0() {
        {
            let q_init: u32 = 50;
            let r_init: u32 = 23;

            let q_mala = Natural::from(q_init);

            let r_mala = Natural::from(r_init);

            let ret = q_mala.div_rem_euclid(r_mala);

            assert_eq!(ret.0, Natural::from(2_u32));

            assert_eq!(ret.1, Natural::from(4_u32));
        }
    }
}
