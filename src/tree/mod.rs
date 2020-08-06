mod array;

use fool::BoolExt;
use std::mem;
use theon::space::FiniteDimensional;
use theon::AsPosition;
use typenum::{NonZero, Unsigned, U2, U3};

use crate::partition::Partition;
use crate::tree::array::FromFn;
use crate::Spatial;

pub type Dimension<P> = <<P as Spatial>::Position as FiniteDimensional>::N;
pub type Link<P, T> = <Branch<P, T> as LinkTopology<Node<P, T>, Dimension<P>>>::Link;

pub trait TreeData {
    type Node: Default;
    type Leaf: AsPosition;
}

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

impl<P, T> Branch<P, T>
where
    Self: LinkTopology<Node<P, T>, Dimension<P>>,
    P: Partition,
    T: TreeData,
    T::Leaf: AsPosition<Position = P::Position>,
{
    pub fn nodes(&self) -> &Link<P, T> {
        self.nodes.as_ref()
    }
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
    fn recompute<F>(&mut self, f: F)
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

    fn insert(&mut self, data: T::Leaf) {
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

    pub fn topology(&self) -> NodeTopology<&Branch<P, T>, &Leaf<T>> {
        self.topology.to_ref()
    }

    pub fn partition(&self) -> &P {
        &self.partition
    }
}

pub struct Tree<P, T>
where
    Branch<P, T>: LinkTopology<Node<P, T>, Dimension<P>>,
    P: Partition,
    T: TreeData,
    T::Leaf: AsPosition<Position = P::Position>,
{
    root: Node<P, T>,
}

impl<P, T> Tree<P, T>
where
    Branch<P, T>: LinkTopology<Node<P, T>, Dimension<P>>,
    P: Partition,
    T: TreeData,
    T::Leaf: AsPosition<Position = P::Position>,
{
    pub fn mutate(self) -> Mutation<P, T> {
        self.into()
    }

    pub fn as_root_node(&self) -> &Node<P, T> {
        &self.root
    }
}

pub struct Mutation<P, T>
where
    Branch<P, T>: LinkTopology<Node<P, T>, Dimension<P>>,
    P: Partition,
    T: TreeData,
    T::Leaf: AsPosition<Position = P::Position>,
{
    tree: Tree<P, T>,
}

impl<P, T> Mutation<P, T>
where
    Branch<P, T>: LinkTopology<Node<P, T>, Dimension<P>>,
    P: Partition,
    T: TreeData,
    T::Leaf: AsPosition<Position = P::Position>,
{
    pub fn insert(&mut self, data: T::Leaf) -> Result<(), ()> {
        self.tree
            .root
            .partition
            .contains(data.as_position())
            .ok_or_else(|| ())?;
        self.tree.root.insert(data);
        Ok(())
    }

    pub fn commit(self) -> Tree<P, T>
    where
        T: TreeData<Node = ()>,
    {
        self.tree
    }

    pub fn commit_with<F>(self, f: F) -> Tree<P, T>
    where
        F: Fn(NodeTopology<(&T::Node, &T::Node), &T::Leaf>) -> T::Node,
    {
        let Mutation { mut tree } = self;
        tree.root.recompute(f);
        tree
    }
}

impl<P, T> From<Tree<P, T>> for Mutation<P, T>
where
    Branch<P, T>: LinkTopology<Node<P, T>, Dimension<P>>,
    P: Partition,
    T: TreeData,
    T::Leaf: AsPosition<Position = P::Position>,
{
    fn from(tree: Tree<P, T>) -> Self {
        Mutation { tree }
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
