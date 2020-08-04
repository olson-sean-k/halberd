mod node;

use theon::space::FiniteDimensional;
use theon::AsPosition;

use crate::partition::Partition;
use crate::tree::node::LinkTopology;
use crate::Spatial;

pub use crate::tree::node::{Branch, Leaf, Link, Node, NodeTopology, Subdivided};

pub type Dimension<P> = <<P as Spatial>::Space as FiniteDimensional>::N;

pub trait TreeData {
    type Node;
    type Leaf: AsPosition;
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
