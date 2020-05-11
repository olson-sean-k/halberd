mod ncube;
mod tree;

use num::{Num, One};

pub use ncube::NCube;

trait Half {
    fn half(self) -> Self;
}

impl<T> Half for T
where
    T: Num + One,
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
    fn contains(&self, _: &S) -> bool;

    // TODO: Should this API handle elements that are not contained by the
    //       partition? Should this function return `Option<usize>`?
    fn index(&self, _: &S) -> usize;
}
