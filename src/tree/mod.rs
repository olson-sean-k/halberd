use theon::space::{EuclideanSpace, FiniteDimensional};
use typenum::{NonZero, Unsigned, U2, U3};

use crate::Partition;

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

pub trait AsNodes<S, P, T>: Topology<S::N>
where
    Branch<S, P, T>: Topology<S::N>,
    S: EuclideanSpace + FiniteDimensional,
    P: Partition<S>,
    T: TreeData,
{
    fn as_nodes(&self) -> &[Node<S, P, T>];

    fn as_nodes_mut(&mut self) -> &mut [Node<S, P, T>];
}

impl<S, P, T> AsNodes<S, P, T> for Branch<S, P, T>
where
    Branch<S, P, T>: Topology<S::N>,
    <Branch<S, P, T> as Topology<S::N>>::Link: AsRef<[Node<S, P, T>]> + AsMut<[Node<S, P, T>]>,
    S: EuclideanSpace + FiniteDimensional,
    P: Partition<S>,
    T: TreeData,
{
    fn as_nodes(&self) -> &[Node<S, P, T>] {
        self.nodes.as_ref().as_ref()
    }

    fn as_nodes_mut(&mut self) -> &mut [Node<S, P, T>] {
        self.nodes.as_mut().as_mut()
    }
}

pub struct Branch<S, P, T>
where
    Self: Topology<S::N>,
    S: EuclideanSpace + FiniteDimensional,
    T: TreeData,
{
    pub data: T::Branch,
    nodes: Box<<Self as Topology<S::N>>::Link>,
}

impl<S, P, T> Topology<U2> for Branch<S, P, T>
where
    S: EuclideanSpace + FiniteDimensional<N = U2>,
    P: Partition<S>,
    T: TreeData,
{
    type Link = [Node<S, P, T>; 4];
}

impl<S, P, T> Topology<U3> for Branch<S, P, T>
where
    S: EuclideanSpace + FiniteDimensional<N = U3>,
    P: Partition<S>,
    T: TreeData,
{
    type Link = [Node<S, P, T>; 8];
}

pub struct Leaf<T>
where
    T: TreeData,
{
    pub data: T::Leaf,
}

pub enum NodeState<S, P, T>
where
    Branch<S, P, T>: Topology<S::N>,
    S: EuclideanSpace + FiniteDimensional,
    P: Partition<S>,
    T: TreeData,
{
    Branch(Branch<S, P, T>),
    Leaf(Leaf<T>),
}

pub struct Node<S, P, T>
where
    Branch<S, P, T>: Topology<S::N>,
    S: EuclideanSpace + FiniteDimensional,
    P: Partition<S>,
    T: TreeData,
{
    pub state: NodeState<S, P, T>,
    pub data: T::Node,
    partition: P,
}

impl<S, P, T> Node<S, P, T>
where
    Branch<S, P, T>: AsNodes<S, P, T>,
    S: EuclideanSpace + FiniteDimensional,
    P: Partition<S>,
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
