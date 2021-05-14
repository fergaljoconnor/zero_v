use super::composite::{NextNode, Node};
pub trait NestLevel {
    fn nest_level(&self) -> usize;
}

impl NestLevel for () {
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
