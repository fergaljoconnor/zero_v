use zero_v::compose;
use zero_v_gen::zero_v;

#[zero_v(trait_types)]
trait IntOp {
    fn execute_1(&self, input: usize) -> usize;
    fn execute_2(&self, input_1: usize, input_2: usize) -> usize;
}

struct Adder {
    value: usize,
}

impl Adder {
    fn new(value: usize) -> Self {
        Self { value }
    }
}

impl IntOp for Adder {
    fn execute_1(&self, input: usize) -> usize {
        input + self.value
    }
    fn execute_2(&self, input_1: usize, input_2: usize) -> usize {
        input_1 + input_2 + self.value
    }
}

struct Multiplier {
    value: usize,
}

impl Multiplier {
    fn new(value: usize) -> Self {
        Self { value }
    }
}

impl IntOp for Multiplier {
    fn execute_1(&self, input: usize) -> usize {
        input * self.value
    }
    fn execute_2(&self, input_1: usize, input_2: usize) -> usize {
        input_1 * input_2 * self.value
    }
}

struct RShifter {
    value: usize,
}

impl RShifter {
    fn new(value: usize) -> Self {
        Self { value }
    }
}

impl IntOp for RShifter {
    fn execute_1(&self, input: usize) -> usize {
        input >> self.value
    }
    fn execute_2(&self, input_1: usize, input_2: usize) -> usize {
        input_1 >> input_2 >> self.value
    }
}

struct LShifter {
    value: usize,
}

impl LShifter {
    fn new(value: usize) -> Self {
        Self { value }
    }
}

impl IntOp for LShifter {
    fn execute_1(&self, input: usize) -> usize {
        input << self.value
    }
    fn execute_2(&self, input_1: usize, input_2: usize) -> usize {
        input_1 << input_2 << self.value
    }
}

#[zero_v(fn_generics, IntOp as IntOps)]
fn execute_1(input: usize, ops: &IntOps) -> Vec<usize> {
    ops.iter_execute_1(input).collect()
}

#[test]
fn test_execute() {
    let ops = compose!(
        Adder::new(0),
        LShifter::new(1),
        Adder::new(2),
        Multiplier::new(3),
        RShifter::new(2)
    );

    let results = ops.iter_execute_1(20).collect::<Vec<_>>();
    assert_eq!(results, vec![20, 20 << 1, 22, 20 * 3, 20 >> 2]);

    let results = ops.iter_execute_2(9, 10).collect::<Vec<_>>();
    assert_eq!(
        results,
        vec![19, 9 << 10 << 1, 21, 9 * 10 * 3, 9 >> 10 >> 2]
    );
}

#[test]
fn test_fn_generics() {
    let ops = compose!(
        Adder::new(0),
        LShifter::new(1),
        Adder::new(2),
        Multiplier::new(3),
        RShifter::new(2)
    );

    let results = execute_1(10, &ops);
    assert_eq!(results, vec![10, 10 << 1, 10 + 2, 10 * 3, 10 >> 2]);
}
