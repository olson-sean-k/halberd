mod ncube;

use fool::BoolExt;

use crate::Spatial;

pub use crate::partition::ncube::NCube;

pub trait Subdivide: Sized {
    type Output: AsRef<[Self]> + IntoIterator<Item = Self>;

    fn subdivide(&self) -> Self::Output;
}

pub trait Partition: Spatial + Subdivide {
    fn contains(&self, point: &Self::Space) -> bool;

    fn index_unchecked(&self, point: &Self::Space) -> usize;

    #[allow(unstable_name_collisions)]
    fn index(&self, point: &Self::Space) -> Option<usize> {
        self.contains(point).then_some(self.index_unchecked(point))
    }
}
