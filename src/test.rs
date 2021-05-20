use crate::{compose, compose_nodes};
use crate::composite::{Composite, NextNode, Node};

trait IntOp {
    fn execute(&self, input: usize) -> usize;
}

struct Adder<const VALUE: usize> {}

impl<const VALUE: usize> Adder<VALUE> {
    fn new() -> Self {
        Self {}
    }
}

impl<const VALUE: usize> IntOp for Adder<VALUE> {
    fn execute(&self, input: usize) -> usize {
        input + VALUE
    }
}

trait IntOpAtLevel {
    fn execute_at_level(&self, input: usize, level: usize) -> Option<usize>;
}

impl<A: IntOp, B: NextNode + IntOpAtLevel> IntOpAtLevel for Node<A, B> {
    fn execute_at_level(&self, input: usize, level: usize) -> Option<usize> {
        if level == 0 {
            Some(self.data.execute(input))
        } else {
            self.next.execute_at_level(input, level - 1)
        }
    }
}

impl IntOpAtLevel for () {
    fn execute_at_level(&self, _input: usize, _level: usize) -> Option<usize> {
        None
    }
}

struct CompositeIterator<'a, Nodes: NextNode + IntOpAtLevel> {
    level: usize,
    input: usize,
    parent: &'a Nodes,
}

impl<'a, Nodes: NextNode + IntOpAtLevel> CompositeIterator<'a, Nodes> {
    fn new(parent: &'a Nodes, input: usize) -> Self {
        Self {
            parent,
            input,
            level: 0,
        }
    }
}

impl<'a, Nodes: NextNode + IntOpAtLevel> Iterator for CompositeIterator<'a, Nodes> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.parent.execute_at_level(self.input, self.level);
        self.level += 1;
        result
    }
}

trait IterExecute<Nodes: NextNode + IntOpAtLevel> {
   fn iter_execute(&self, input: usize) -> CompositeIterator<'_, Nodes>;
}

impl<Nodes: NextNode + IntOpAtLevel> IterExecute<Nodes> for Composite<Nodes> {
   fn iter_execute(&self, input: usize) -> CompositeIterator<'_, Nodes> {
       CompositeIterator::new(&self.head, input)
   }
}

#[test]
fn can_compose() {
    let composite = compose!(Adder::<11>::new(), Adder::<12>::new(), Adder::<13>::new());
    let outputs: Vec<usize> = composite.iter_execute(0).collect();
    assert_eq!(outputs, vec![11, 12, 13]);
}
