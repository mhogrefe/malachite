macro_rules! forward_unary_op {
    ($struct:tt, $trait:tt, $fn:ident) => {
        impl $trait for $struct {
            type Output = $struct;

            #[inline]
            fn $fn(mut self) -> Self::Output {
                self.0 = $trait::$fn(self.0);
                self
            }
        }

        impl $trait for &$struct {
            type Output = $struct;

            #[inline]
            fn $fn(self) -> Self::Output {
                $struct($trait::$fn(&self.0))
            }
        }
    };
}

macro_rules! forward_binary_op {
    ($struct:tt, $trait:tt, $fn:ident) => {
        impl $trait<$struct> for $struct {
            type Output = $struct;

            #[inline]
            fn $fn(self, rhs: $struct) -> Self::Output {
                $struct($trait::$fn(self.0, rhs.0))
            }
        }

        impl $trait<&$struct> for $struct {
            type Output = $struct;

            #[inline]
            fn $fn(self, rhs: &$struct) -> Self::Output {
                $struct($trait::$fn(self.0, &rhs.0))
            }
        }

        impl $trait<$struct> for &$struct {
            type Output = $struct;

            #[inline]
            fn $fn(self, rhs: $struct) -> Self::Output {
                $struct($trait::$fn(&self.0, rhs.0))
            }
        }

        impl $trait<&$struct> for &$struct {
            type Output = $struct;

            #[inline]
            fn $fn(self, rhs: &$struct) -> Self::Output {
                $struct($trait::$fn(&self.0, &rhs.0))
            }
        }
    };
}
