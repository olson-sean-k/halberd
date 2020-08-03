use theon::space::FiniteDimensional;
use typenum::{NonZero, Unsigned, U2, U3};

use crate::{Partition, Spatial};

type Dimension<P> = <<P as Spatial>::Space as FiniteDimensional>::N;

trait Reborrow {
    type Target;

    fn reborrow(&self) -> &Self::Target;
}

trait ReborrowMut: Reborrow {
    fn reborrow_mut(&mut self) -> &mut Self::Target;
}

impl<'a, T> Reborrow for &'a T {
    type Target = T;

    fn reborrow(&self) -> &Self::Target {
        self
    }
}

impl<'a, T> Reborrow for &'a mut T {
    type Target = T;

    fn reborrow(&self) -> &Self::Target {
        &*self
    }
}

impl<'a, T> ReborrowMut for &'a mut T {
    fn reborrow_mut(&mut self) -> &mut Self::Target {
        self
    }
}

pub trait TreeData {
    type Node;
    type Branch;
    type Leaf;
}

impl TreeData for () {
    type Node = ();
    type Branch = ();
    type Leaf = ();
}

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
    pub state: NodeState<P, T>,
    pub data: T::Node,
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
