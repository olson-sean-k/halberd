use std::mem;
use theon::space::FiniteDimensional;
use theon::AsPosition;
use typenum::{NonZero, Unsigned, U2, U3};

use crate::partition::Partition;
use crate::tree::array::FromFn;
use crate::tree::TreeData;
use crate::Spatial;

pub type Dimension<P> = <<P as Spatial>::Position as FiniteDimensional>::N;
pub type Link<P, T> = <Branch<P, T> as LinkTopology<Node<P, T>, Dimension<P>>>::Link;

pub trait LinkTopology<T, N>
where
    N: NonZero + Unsigned,
{
    type Link: AsMut<[T]> + AsRef<[T]> + FromFn<T>;
}

pub struct Branch<P, T>
where
    Self: LinkTopology<Node<P, T>, Dimension<P>>,
    P: Partition,
    T: TreeData,
    T::Leaf: AsPosition<Position = P::Position>,
{
    nodes: Box<Link<P, T>>,
}

impl<P, T> LinkTopology<Node<P, T>, U2> for Branch<P, T>
where
    P: Partition,
    P::Position: FiniteDimensional<N = U2>,
    T: TreeData,
    T::Leaf: AsPosition<Position = P::Position>,
{
    type Link = [Node<P, T>; 4];
}

impl<P, T> LinkTopology<Node<P, T>, U3> for Branch<P, T>
where
    P: Partition,
    P::Position: FiniteDimensional<N = U3>,
    T: TreeData,
    T::Leaf: AsPosition<Position = P::Position>,
{
    type Link = [Node<P, T>; 8];
}

pub struct Leaf<T>
where
    T: TreeData,
{
    data: Option<T::Leaf>,
}

impl<T> Leaf<T>
where
    T: TreeData,
{
    pub fn get(&self) -> Option<&T::Leaf> {
        self.data.as_ref()
    }
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

impl<P, T> NodeTopology<Branch<P, T>, Leaf<T>>
where
    Branch<P, T>: LinkTopology<Node<P, T>, Dimension<P>>,
    P: Partition,
    T: TreeData,
    T::Leaf: AsPosition<Position = P::Position>,
{
    fn empty() -> Self {
        NodeTopology::Leaf(Leaf { data: None })
    }
}

pub struct Node<P, T>
where
    Branch<P, T>: LinkTopology<Node<P, T>, Dimension<P>>,
    P: Partition,
    T: TreeData,
    T::Leaf: AsPosition<Position = P::Position>,
{
    data: T::Node,
    topology: NodeTopology<Branch<P, T>, Leaf<T>>,
    partition: P,
}

impl<P, T> Node<P, T>
where
    Branch<P, T>: LinkTopology<Node<P, T>, Dimension<P>>,
    P: Partition,
    T: TreeData,
    T::Leaf: AsPosition<Position = P::Position>,
{
    pub(in crate::tree) fn empty(partition: P, data: T::Node) -> Self {
        Node {
            data,
            topology: NodeTopology::empty(),
            partition,
        }
    }

    pub(in crate::tree) fn recompute<F>(&mut self, f: F)
    where
        F: Fn(NodeTopology<(&T::Node, &T::Node), &T::Leaf>) -> T::Node,
    {
        self.data = match self.topology {
            NodeTopology::Leaf(Leaf { data: None }) => Default::default(),
            NodeTopology::Leaf(Leaf {
                data: Some(ref data),
            }) => f(NodeTopology::Leaf(data)),
            NodeTopology::Branch(Branch { ref mut nodes }) => {
                for node in nodes.as_mut().as_mut() {
                    node.recompute(|data| f(data));
                }
                nodes
                    .as_ref()
                    .as_ref()
                    .iter()
                    .map(|node| &node.data)
                    .fold(Default::default(), |base, next| {
                        f(NodeTopology::Branch((&base, next)))
                    })
            }
        };
    }

    pub(in crate::tree) fn insert(&mut self, data: T::Leaf) {
        let dispatch = |nodes: &mut Link<P, T>, partition: &P, data: T::Leaf| {
            let nodes = nodes.as_mut().as_mut();
            nodes[partition.index_unchecked(data.as_position())].insert(data);
        };
        let mut topology = NodeTopology::empty();
        mem::swap(&mut topology, &mut self.topology);
        self.topology = match topology {
            NodeTopology::Leaf(Leaf { data: None }) => {
                NodeTopology::Leaf(Leaf { data: Some(data) })
            }
            NodeTopology::Leaf(Leaf {
                data: Some(repartition),
            }) => {
                let mut nodes = Box::new(Link::<P, T>::from_iter(
                    self.partition
                        .subdivide()
                        .into_iter()
                        .map(|partition| Node {
                            data: Default::default(),
                            topology: NodeTopology::empty(),
                            partition,
                        }),
                ));
                dispatch(&mut nodes, &self.partition, data);
                dispatch(&mut nodes, &self.partition, repartition);
                NodeTopology::Branch(Branch { nodes })
            }
            NodeTopology::Branch(Branch { mut nodes }) => {
                dispatch(&mut nodes, &self.partition, data);
                NodeTopology::Branch(Branch { nodes })
            }
        };
    }

    pub fn get(&self) -> &T::Node {
        &self.data
    }

    pub fn as_leaf(&self) -> Option<&Leaf<T>> {
        self.topology.to_ref().into_leaf()
    }

    pub fn partition(&self) -> &P {
        &self.partition
    }
}
