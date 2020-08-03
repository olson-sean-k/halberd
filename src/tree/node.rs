use theon::space::FiniteDimensional;
use typenum::{NonZero, Unsigned, U2, U3};

use crate::partition::Partition;
use crate::tree::{Dimension, TreeData};

pub trait Topology<N>
where
    N: NonZero + Unsigned,
{
    type Link;
}

pub trait AsNodes<P, T>: Topology<Dimension<P>>
where
    Branch<P, T>: Topology<Dimension<P>>,
    P: Partition,
    T: TreeData,
{
    fn as_nodes(&self) -> &[Node<P, T>];

    fn as_nodes_mut(&mut self) -> &mut [Node<P, T>];
}

impl<P, T> AsNodes<P, T> for Branch<P, T>
where
    Branch<P, T>: Topology<Dimension<P>>,
    <Branch<P, T> as Topology<Dimension<P>>>::Link: AsRef<[Node<P, T>]> + AsMut<[Node<P, T>]>,
    P: Partition,
    T: TreeData,
{
    fn as_nodes(&self) -> &[Node<P, T>] {
        self.nodes.as_ref().as_ref()
    }

    fn as_nodes_mut(&mut self) -> &mut [Node<P, T>] {
        self.nodes.as_mut().as_mut()
    }
}

pub struct Branch<P, T>
where
    Self: Topology<Dimension<P>>,
    P: Partition,
    T: TreeData,
{
    pub data: T::Branch,
    nodes: Box<<Self as Topology<Dimension<P>>>::Link>,
}

impl<P, T> Topology<U2> for Branch<P, T>
where
    P: Partition,
    P::Space: FiniteDimensional<N = U2>,
    T: TreeData,
{
    type Link = [Node<P, T>; 4];
}

impl<P, T> Topology<U3> for Branch<P, T>
where
    P: Partition,
    P::Space: FiniteDimensional<N = U3>,
    T: TreeData,
{
    type Link = [Node<P, T>; 8];
}

pub struct Leaf<T>
where
    T: TreeData,
{
    pub data: T::Leaf,
}

pub enum NodeState<P, T>
where
    Branch<P, T>: Topology<Dimension<P>>,
    P: Partition,
    T: TreeData,
{
    Branch(Branch<P, T>),
    Leaf(Leaf<T>),
}

pub struct Node<P, T>
where
    Branch<P, T>: Topology<Dimension<P>>,
    P: Partition,
    T: TreeData,
{
    pub data: T::Node,
    state: NodeState<P, T>,
    partition: P,
}

impl<P, T> Node<P, T>
where
    Branch<P, T>: AsNodes<P, T>,
    P: Partition,
    T: TreeData,
{
    #[cfg(test)] // Sanity check on `AsNodes` constraint.
    fn test(&self) {
        match self.state {
            NodeState::Branch(ref branch) => for node in branch.as_nodes() {},
            _ => {}
        }
    }
}
