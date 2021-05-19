// TODO: It's debatable whether this type actually adds any value. Maybe an
// iterator should start at level zero and every call to execute_at_level
// for a node should execute if the level is zero and if it isn't zero,
// take one from it and feed it into execute_at_level for the next node.
// This would probably simplify implementation for users by removing one 
// of the tangle of traits they need to deal with, so if the performance is
// good enough it's worth looking at.

use super::composite::{NextNode, Node};

/// Defines a trait which should return the nesting level of a node in a
/// composite (the unit type at the deepest level should have level zero
/// and each level should return the nesting level of the level below plus one.
pub trait NestLevel {
    fn nest_level(&self) -> usize;
}

impl NestLevel for () {
    // On my current hardware, this inline is  critical (it takes 85%
    // off runtime for the integer operations benchmarks).
    #[inline]
    fn nest_level(&self) -> usize {
        0
    }
}

impl<A, B: NextNode + NestLevel> NestLevel for Node<A, B> {
    fn nest_level(&self) -> usize {
        self.next.nest_level() + 1
    }
}
