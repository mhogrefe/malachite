use num::basic::integers::PrimitiveInteger;

/// Generates an iterator's values, but with the highest bit set.
#[derive(Clone, Debug)]
pub struct RandomHighestBitSetValues<I: Iterator>
where
    I::Item: PrimitiveInteger,
{
    pub(crate) xs: I,
    pub(crate) mask: I::Item,
}

impl<I: Iterator> Iterator for RandomHighestBitSetValues<I>
where
    I::Item: PrimitiveInteger,
{
    type Item = I::Item;

    #[inline]
    fn next(&mut self) -> Option<I::Item> {
        self.xs.next().map(|x| x | self.mask)
    }
}
