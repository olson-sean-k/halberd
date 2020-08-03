pub mod ncube;
pub mod tree;

use decorum::Real;
use fool::BoolExt;
use num::{Num, One};

pub use ncube::NCube;

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

pub trait Subdivide: Sized {
    type Output: AsRef<[Self]> + IntoIterator<Item = Self>;

    fn subdivide(&self) -> Self::Output;
}

pub trait Partition<S>: Subdivide {
    fn contains(&self, point: &S) -> bool;

    fn index_unchecked(&self, point: &S) -> usize;

    #[allow(unstable_name_collisions)]
    fn index(&self, point: &S) -> Option<usize> {
        self.contains(point).then_some(self.index_unchecked(point))
    }
}
