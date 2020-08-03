pub mod partition;
pub mod tree;

use decorum::Real;
use num::{Num, One};
use theon::space::{EuclideanSpace, FiniteDimensional};

trait Half {
    fn half(self) -> Self;
}

impl<T> Half for T
where
    T: Num + One + Real,
{
    fn half(self) -> Self {
        self / (Self::one() + Self::one())
    }
}

pub trait Spatial {
    type Space: EuclideanSpace + FiniteDimensional;
}
