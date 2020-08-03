use std::ops::Deref;

use crate::partition::Partition;
use crate::tree::borrow::{Reborrow, ReborrowInto};
use crate::tree::node::{Branch, Node, Topology};
use crate::tree::{Dimension, TreeData};

pub trait ClosedNode {
    type Partition: Partition;
    type Data: TreeData;
}

pub trait AsNode<P, T>: ClosedNode<Partition = P, Data = T>
where
    Branch<P, T>: Topology<Dimension<P>>,
    P: Partition,
    T: TreeData,
{
    fn as_node(&self) -> &Node<P, T>;
}

pub trait AsNodeMut<P, T>: AsNode<P, T>
where
    Branch<P, T>: Topology<Dimension<P>>,
    P: Partition,
    T: TreeData,
{
    fn as_node_mut(&mut self) -> &mut Node<P, T>;
}

pub struct NodeView<B>
where
    B: Reborrow,
    B::Target: ClosedNode,
{
    node: B,
}

impl<B, N> NodeView<B>
where
    B: Reborrow<Target = N>,
    N: ClosedNode,
{
    pub fn to_ref(&self) -> NodeView<&N> {
        NodeView {
            node: self.node.reborrow(),
        }
    }
}

impl<'a, B, N> NodeView<B>
where
    B: ReborrowInto<'a, Target = N>,
    N: ClosedNode,
{
    pub fn into_ref(self) -> NodeView<&'a N> {
        NodeView {
            node: self.node.reborrow_into(),
        }
    }
}

impl<B, N, P, T> NodeView<B>
where
    B: Reborrow<Target = N>,
    N: AsNode<P, T> + ClosedNode<Partition = P, Data = T>,
    Branch<P, T>: Topology<Dimension<P>>,
    P: Partition,
    T: TreeData,
{
}

impl<B, N> Deref for NodeView<B>
where
    B: Reborrow<Target = N>,
    N: ClosedNode,
{
    type Target = N;

    fn deref(&self) -> &Self::Target {
        self.node.reborrow()
    }
}

pub struct NodeOrphan;
