use std::mem;
use theon::AsPosition;

use crate::partition::Partition;
use crate::tree::TreeData;

pub struct Branch<P, T>
where
    P: Partition,
    T: TreeData,
    T::Leaf: AsPosition<Position = P::Position>,
{
    nodes: Vec<Node<P, T>>,
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

pub enum Topology<B, L> {
    Branch(B),
    Leaf(L),
}

impl<B, L> Topology<B, L> {
    pub fn into_branch(self) -> Option<B> {
        if let Topology::Branch(branch) = self {
            Some(branch)
        }
        else {
            None
        }
    }

    pub fn into_leaf(self) -> Option<L> {
        if let Topology::Leaf(leaf) = self {
            Some(leaf)
        }
        else {
            None
        }
    }

    fn to_ref(&self) -> Topology<&B, &L> {
        match self {
            Topology::Branch(ref branch) => Topology::Branch(branch),
            Topology::Leaf(ref leaf) => Topology::Leaf(leaf),
        }
    }
}

impl<P, T> Topology<Branch<P, T>, Leaf<T>>
where
    P: Partition,
    T: TreeData,
    T::Leaf: AsPosition<Position = P::Position>,
{
    fn empty() -> Self {
        Topology::Leaf(Leaf { data: None })
    }
}

pub struct Node<P, T>
where
    P: Partition,
    T: TreeData,
    T::Leaf: AsPosition<Position = P::Position>,
{
    data: T::Node,
    topology: Topology<Branch<P, T>, Leaf<T>>,
    partition: P,
}

impl<P, T> Node<P, T>
where
    P: Partition,
    T: TreeData,
    T::Leaf: AsPosition<Position = P::Position>,
{
    pub(in crate::tree) fn empty(partition: P, data: T::Node) -> Self {
        Node {
            data,
            topology: Topology::empty(),
            partition,
        }
    }

    pub(in crate::tree) fn recompute<F>(&mut self, f: F)
    where
        F: Fn(Topology<(&T::Node, &T::Node), &T::Leaf>) -> T::Node,
    {
        self.data = match self.topology {
            Topology::Leaf(Leaf { data: None }) => Default::default(),
            Topology::Leaf(Leaf {
                data: Some(ref data),
            }) => f(Topology::Leaf(data)),
            Topology::Branch(Branch { ref mut nodes }) => {
                for node in nodes.iter_mut() {
                    node.recompute(|data| f(data));
                }
                nodes
                    .iter()
                    .map(|node| &node.data)
                    .fold(Default::default(), |base, next| {
                        f(Topology::Branch((&base, next)))
                    })
            }
        };
    }

    pub(in crate::tree) fn insert(&mut self, data: T::Leaf) {
        let dispatch = |nodes: &mut Vec<Node<P, T>>, partition: &P, data: T::Leaf| {
            nodes[partition.index_unchecked(data.as_position())].insert(data);
        };
        let mut topology = Topology::empty();
        mem::swap(&mut topology, &mut self.topology);
        self.topology = match topology {
            Topology::Leaf(Leaf { data: None }) => Topology::Leaf(Leaf { data: Some(data) }),
            Topology::Leaf(Leaf {
                data: Some(repartition),
            }) => {
                let mut nodes = self
                    .partition
                    .subdivide()
                    .into_iter()
                    .map(|partition| Node {
                        data: Default::default(),
                        topology: Topology::empty(),
                        partition,
                    })
                    .collect();
                dispatch(&mut nodes, &self.partition, data);
                dispatch(&mut nodes, &self.partition, repartition);
                Topology::Branch(Branch { nodes })
            }
            Topology::Branch(Branch { mut nodes }) => {
                dispatch(&mut nodes, &self.partition, data);
                Topology::Branch(Branch { nodes })
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
