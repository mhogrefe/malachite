use malachite_base::num::basic::traits::Zero;

use natural::InnerNatural::{Large, Small};
use natural::Natural;
use platform::Limb;

// Returns the length of `xs`, excluding trailing zeros.
fn limbs_significant_length(xs: &[Limb]) -> usize {
    xs.iter()
        .enumerate()
        .rev()
        .find(|&(_, &x)| x != 0)
        .map_or(0, |(i, _)| i + 1)
}

impl Natural {
    /// Converts a slice of limbs to a `Natural`, in ascending order, so that less significant limbs
    /// have lower indices in the input slice.
    ///
    /// This function borrows `xs`. If taking ownership of `xs` is possible, `from_owned_limbs_asc`
    /// is more efficient.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `xs.len()`
    ///
    /// This function is more efficient than `Natural::from_limbs_desc`.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from_limbs_asc(&[]).to_string(), "0");
    /// assert_eq!(Natural::from_limbs_asc(&[123]).to_string(), "123");
    /// // 10^12 = 232 * 2^32 + 3567587328
    /// assert_eq!(Natural::from_limbs_asc(&[3567587328, 232]).to_string(), "1000000000000");
    /// ```
    pub fn from_limbs_asc(xs: &[Limb]) -> Natural {
        let significant_length = limbs_significant_length(xs);
        match significant_length {
            0 => Natural::ZERO,
            1 => Natural(Small(xs[0])),
            _ => Natural(Large(xs[..significant_length].to_vec())),
        }
    }

    /// Converts a slice of limbs to a `Natural`, in descending order, so that less significant
    /// limbs have higher indices in the input slice.
    ///
    /// This function borrows `xs`. If taking ownership of `xs` is possible, `from_owned_limbs_desc`
    /// is more efficient.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `xs.len()`
    ///
    /// This function is less efficient than `Natural::from_limbs_asc`.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from_limbs_desc(&[]).to_string(), "0");
    /// assert_eq!(Natural::from_limbs_desc(&[123]).to_string(), "123");
    /// // 10^12 = 232 * 2^32 + 3567587328
    /// assert_eq!(Natural::from_limbs_desc(&[232, 3567587328]).to_string(), "1000000000000");
    /// ```
    pub fn from_limbs_desc(xs: &[Limb]) -> Natural {
        Natural::from_owned_limbs_asc(xs.iter().cloned().rev().collect())
    }

    /// Converts a `Vec` of limbs to a `Natural`, in ascending order, so that less significant limbs
    /// have lower indices in the input `Vec`.
    ///
    /// This function takes ownership of `xs`. If it's necessary to borrow `xs` instead, use
    /// `from_limbs_asc`.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// where n = `xs.len()`
    ///
    /// This function is more efficient than `Natural::from_limbs_desc`.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from_owned_limbs_asc(vec![]).to_string(), "0");
    /// assert_eq!(Natural::from_owned_limbs_asc(vec![123]).to_string(), "123");
    /// // 10^12 = 232 * 2^32 + 3567587328
    /// assert_eq!(Natural::from_owned_limbs_asc(vec![3567587328, 232]).to_string(),
    ///     "1000000000000");
    /// ```
    pub fn from_owned_limbs_asc(mut xs: Vec<Limb>) -> Natural {
        let significant_length = limbs_significant_length(&xs);
        match significant_length {
            0 => Natural::ZERO,
            1 => Natural(Small(xs[0])),
            _ => {
                xs.truncate(significant_length);
                Natural(Large(xs))
            }
        }
    }

    /// Converts a `Vec` of limbs to a `Natural`, in descending order, so that less significant
    /// limbs have higher indices in the input `Vec`.
    ///
    /// This function takes ownership of `xs`. If it's necessary to borrow `xs` instead, use
    /// `from_limbs_desc`.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// where n = `xs.len()`
    ///
    /// This function is less efficient than `Natural::from_limbs_asc`.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from_owned_limbs_desc(vec![]).to_string(), "0");
    /// assert_eq!(Natural::from_owned_limbs_desc(vec![123]).to_string(), "123");
    /// // 10^12 = 232 * 2^32 + 3567587328
    /// assert_eq!(Natural::from_owned_limbs_desc(vec![232, 3567587328]).to_string(),
    ///     "1000000000000");
    /// ```
    pub fn from_owned_limbs_desc(mut xs: Vec<Limb>) -> Natural {
        xs.reverse();
        Natural::from_owned_limbs_asc(xs)
    }
}
