use theon::space::FiniteDimensional;
use typenum::{NonZero, Unsigned, U2, U3};

use crate::partition::Partition;
use crate::tree::{Dimension, TreeData};

pub trait LinkTopology<N>
where
    N: NonZero + Unsigned,
{
    type Link;
}

pub trait Subdivided<P, T>: LinkTopology<Dimension<P>>
where
    Branch<P, T>: LinkTopology<Dimension<P>>,
    P: Partition,
    T: TreeData,
{
    fn as_subdivision_slice(&self) -> &[Node<P, T>];

    fn as_subdivision_slice_mut(&mut self) -> &mut [Node<P, T>];
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
    <Branch<P, T> as LinkTopology<Dimension<P>>>::Link: AsRef<[Node<P, T>]> + AsMut<[Node<P, T>]>,
    P: Partition,
    T: TreeData,
{
    fn as_subdivision_slice(&self) -> &[Node<P, T>] {
        self.nodes.as_ref().as_ref()
    }

    fn as_subdivision_slice_mut(&mut self) -> &mut [Node<P, T>] {
        self.nodes.as_mut().as_mut()
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
    pub data: T::Node,
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
