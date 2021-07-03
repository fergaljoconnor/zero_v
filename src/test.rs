use crate as zero_v;
use crate::{compose, zero_v};

#[zero_v(trait_types)]
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

#[test]
fn can_compose() {
    let composite = compose!(Adder::<11>::new(), Adder::<12>::new(), Adder::<13>::new());
    let outputs: Vec<usize> = composite.iter_execute(0).collect();
    assert_eq!(outputs, vec![11, 12, 13]);
}


#[test]
fn can_iter_manually_by_level() {
    let composite = compose!(Adder::<11>::new(), Adder::<12>::new(), Adder::<13>::new());
    let mut outputs = Vec::new();
    for i in 0..composite.len() {
        outputs.push(composite.head.execute_at_level(0, i));
    }
}
