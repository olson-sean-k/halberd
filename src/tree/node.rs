use theon::space::FiniteDimensional;
use typenum::{NonZero, Unsigned, U2, U3};

use crate::partition::Partition;
use crate::tree::{Dimension, TreeData};

pub type Link<P, T> = <Branch<P, T> as LinkTopology<Dimension<P>>>::Link;

pub trait LinkTopology<N>
where
    N: NonZero + Unsigned,
{
    type Link;
}

pub trait Subdivided<P, T>: LinkTopology<Dimension<P>>
where
    Branch<P, T>: LinkTopology<Dimension<P>>,
    Link<P, T>: AsRef<[Node<P, T>]> + AsMut<[Node<P, T>]>,
    P: Partition,
    T: TreeData,
{
    fn nodes(&self) -> &Link<P, T>;

    fn nodes_mut(&mut self) -> &mut Link<P, T>;
}

pub struct Branch<P, T>
where
    Self: LinkTopology<Dimension<P>>,
    P: Partition,
    T: TreeData,
{
    nodes: Box<<Self as LinkTopology<Dimension<P>>>::Link>,
}

impl<P, T> LinkTopology<U2> for Branch<P, T>
where
    P: Partition,
    P::Space: FiniteDimensional<N = U2>,
    T: TreeData,
{
    type Link = [Node<P, T>; 4];
}

impl<P, T> LinkTopology<U3> for Branch<P, T>
where
    P: Partition,
    P::Space: FiniteDimensional<N = U3>,
    T: TreeData,
{
    type Link = [Node<P, T>; 8];
}

impl<P, T> Subdivided<P, T> for Branch<P, T>
where
    Branch<P, T>: LinkTopology<Dimension<P>>,
    Link<P, T>: AsRef<[Node<P, T>]> + AsMut<[Node<P, T>]>,
    P: Partition,
    T: TreeData,
{
    fn nodes(&self) -> &Link<P, T> {
        self.nodes.as_ref()
    }

    fn nodes_mut(&mut self) -> &mut Link<P, T> {
        self.nodes.as_mut()
    }
}

pub struct Leaf<T>
where
    T: TreeData,
{
    data: Option<T::Leaf>,
}

pub enum NodeTopology<B, L> {
    Branch(B),
    Leaf(L),
}

impl<B, L> NodeTopology<B, L> {
    pub fn into_branch(self) -> Option<B> {
        if let NodeTopology::Branch(branch) = self {
            Some(branch)
        }
        else {
            None
        }
    }

    pub fn into_leaf(self) -> Option<L> {
        if let NodeTopology::Leaf(leaf) = self {
            Some(leaf)
        }
        else {
            None
        }
    }

    fn to_ref(&self) -> NodeTopology<&B, &L> {
        match self {
            NodeTopology::Branch(ref branch) => NodeTopology::Branch(branch),
            NodeTopology::Leaf(ref leaf) => NodeTopology::Leaf(leaf),
        }
    }
}

pub struct Node<P, T>
where
    Branch<P, T>: LinkTopology<Dimension<P>>,
    P: Partition,
    T: TreeData,
{
    data: T::Node,
    topology: NodeTopology<Branch<P, T>, Leaf<T>>,
    partition: P,
}

impl<P, T> Node<P, T>
where
    Branch<P, T>: LinkTopology<Dimension<P>>,
    P: Partition,
    T: TreeData,
{
    pub fn partition(&self) -> &P {
        &self.partition
    }

    pub fn topology(&self) -> NodeTopology<&Branch<P, T>, &Leaf<T>> {
        self.topology.to_ref()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use theon::integration::nalgebra;

    use decorum::R64;
    use nalgebra::Point3;

    use crate::partition::NCube;

    type E3 = Point3<R64>;

    // TODO: Provide features to implement `TreeData` for Euclidean spaces.
    impl TreeData for E3 {
        type Node = ();
        type Leaf = E3;
    }

    // Sanity check. `rustc` can determine the `Link` type from `P` and `T`.
    impl Node<NCube<E3>, E3> {
        fn _test(&self) {
            if let Some(branch) = self.topology().into_branch() {
                let [_, _, ..] = branch.nodes();
                for _ in branch.nodes().as_ref() {}
            }
        }
    }
}
