use std::ops::{Deref, DerefMut};

use crate::partition::Partition;
use crate::tree::borrow::{Reborrow, ReborrowInto, ReborrowMut};
use crate::tree::node::{AsSubdivisions, Branch, ClosedNode, Node, Topology};
use crate::tree::{Dimension, TreeData};

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

impl<B, P, T> NodeView<B>
where
    B: Reborrow<Target = Node<P, T>>,
    Branch<P, T>: AsSubdivisions<P, T>,
    P: Partition,
    T: TreeData,
{
    pub fn subdivisions<'a>(
        &'a self,
    ) -> Option<impl ExactSizeIterator<Item = NodeView<&'a Node<P, T>>>>
    where
        P: 'a,
        T: 'a,
    {
        self.as_branch().map(|branch| {
            branch
                .as_subdivisions()
                .iter()
                .map(|node| NodeView { node })
        })
    }
}

impl<B, P, T> NodeView<B>
where
    B: ReborrowMut<Target = Node<P, T>>,
    Branch<P, T>: AsSubdivisions<P, T>,
    P: Partition,
    T: TreeData,
{
    pub fn subdivisions_mut<'a>(
        &'a mut self,
    ) -> Option<impl ExactSizeIterator<Item = NodeView<&'a mut Node<P, T>>>>
    where
        P: 'a,
        T: 'a,
    {
        self.as_branch_mut().map(|branch| {
            branch
                .as_subdivisions_mut()
                .iter_mut()
                .map(|node| NodeView { node })
        })
    }
}

impl<'a, P, T> NodeView<&'a mut Node<P, T>>
where
    Branch<P, T>: AsSubdivisions<P, T>,
    P: Partition,
    T: TreeData,
{
    pub fn into_subdivisions_mut(
        self,
    ) -> Option<impl ExactSizeIterator<Item = NodeView<&'a mut Node<P, T>>>> {
        let NodeView { node } = self;
        node.as_branch_mut().map(|branch| {
            branch
                .as_subdivisions_mut()
                .iter_mut()
                .map(|node| NodeView { node })
        })
    }
}

impl<B, P, T> Deref for NodeView<B>
where
    B: Reborrow<Target = Node<P, T>>,
    Branch<P, T>: Topology<Dimension<P>>,
    P: Partition,
    T: TreeData,
{
    type Target = Node<P, T>;

    fn deref(&self) -> &Self::Target {
        self.node.reborrow()
    }
}

impl<B, P, T> DerefMut for NodeView<B>
where
    B: ReborrowMut<Target = Node<P, T>>,
    Branch<P, T>: Topology<Dimension<P>>,
    P: Partition,
    T: TreeData,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.node.reborrow_mut()
    }
}

pub struct NodeOrphan;
