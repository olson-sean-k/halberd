mod borrow;
mod node;
mod view;

use theon::space::FiniteDimensional;

use crate::partition::Partition;
use crate::tree::node::Topology;
use crate::Spatial;

pub use crate::tree::node::{Branch, Leaf, Node, NodeState};
pub use crate::tree::view::{NodeOrphan, NodeView};

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
    Branch<P, T>: Topology<Dimension<P>>,
    P: Partition,
    T: TreeData,
{
    root: Node<P, T>,
}
