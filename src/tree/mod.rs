mod node;

use theon::space::FiniteDimensional;

use crate::partition::Partition;
use crate::tree::node::LinkTopology;
use crate::Spatial;

pub use crate::tree::node::{Branch, Leaf, Node, NodeTopology, Subdivided};

type Dimension<P> = <<P as Spatial>::Space as FiniteDimensional>::N;

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

pub struct Tree<P, T>
where
    Branch<P, T>: LinkTopology<Dimension<P>>,
    P: Partition,
    T: TreeData,
{
    root: Node<P, T>,
}

impl<P, T> Tree<P, T>
where
    Branch<P, T>: LinkTopology<Dimension<P>>,
    P: Partition,
    T: TreeData,
{
    pub fn as_root_node(&self) -> &Node<P, T> {
        &self.root
    }

    pub fn as_root_node_mut(&mut self) -> &mut Node<P, T> {
        &mut self.root
    }
}
