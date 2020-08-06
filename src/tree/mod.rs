mod node;

use fool::BoolExt as _;
use theon::AsPosition;

use crate::partition::Partition;

pub use crate::tree::node::{Leaf, Node, Topology};

pub trait TreeData {
    type Node: Default;
    type Leaf: AsPosition;
}

pub struct Tree<P, T>
where
    P: Partition,
    T: TreeData,
    T::Leaf: AsPosition<Position = P::Position>,
{
    root: Node<P, T>,
}

impl<P, T> Tree<P, T>
where
    P: Partition,
    T: TreeData,
    T::Leaf: AsPosition<Position = P::Position>,
{
    pub fn empty(partition: P) -> Self {
        Tree {
            root: Node::empty(partition, Default::default()),
        }
    }

    pub fn from_iter<I>(partition: P, input: I) -> Result<Self, ()>
    where
        T: TreeData<Node = ()>,
        I: IntoIterator<Item = T::Leaf>,
    {
        let mut mutation = Mutation::from(Tree::empty(partition));
        mutation.append(input)?;
        Ok(mutation.commit())
    }

    pub fn from_iter_with<I, F>(partition: P, input: I, f: F) -> Result<Self, ()>
    where
        I: IntoIterator<Item = T::Leaf>,
        F: Fn(Topology<(&T::Node, &T::Node), &T::Leaf>) -> T::Node,
    {
        let mut mutation = Mutation::from(Tree::empty(partition));
        mutation.append(input)?;
        Ok(mutation.commit_with(f))
    }

    pub fn as_root_node(&self) -> &Node<P, T> {
        &self.root
    }
}

pub struct Mutation<P, T>
where
    P: Partition,
    T: TreeData,
    T::Leaf: AsPosition<Position = P::Position>,
{
    tree: Tree<P, T>,
}

impl<P, T> Mutation<P, T>
where
    P: Partition,
    T: TreeData,
    T::Leaf: AsPosition<Position = P::Position>,
{
    pub fn insert(&mut self, data: T::Leaf) -> Result<(), ()> {
        self.tree
            .root
            .partition()
            .contains(data.as_position())
            .ok_or_else(|| ())?;
        self.tree.root.insert(data);
        Ok(())
    }

    pub fn append<I>(&mut self, input: I) -> Result<(), ()>
    where
        I: IntoIterator<Item = T::Leaf>,
    {
        input.into_iter().map(|data| self.insert(data)).collect()
    }

    pub fn commit(self) -> Tree<P, T>
    where
        T: TreeData<Node = ()>,
    {
        self.tree
    }

    pub fn commit_with<F>(self, f: F) -> Tree<P, T>
    where
        F: Fn(Topology<(&T::Node, &T::Node), &T::Leaf>) -> T::Node,
    {
        let Mutation { mut tree } = self;
        tree.root.recompute(f);
        tree
    }
}

impl<P, T> From<Tree<P, T>> for Mutation<P, T>
where
    P: Partition,
    T: TreeData,
    T::Leaf: AsPosition<Position = P::Position>,
{
    fn from(tree: Tree<P, T>) -> Self {
        Mutation { tree }
    }
}
