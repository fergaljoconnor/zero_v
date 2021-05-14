use criterion::{black_box, criterion_group, criterion_main, Criterion};
use zero_v::{compose, compose_nodes, Composite, NestLevel, NextNode, Node};

trait IntOp {
    fn execute(&self, input: usize) -> usize;
}

trait IntOpAtLevel {
    fn execute_at_level(&self, input: usize, level: usize) -> Option<usize>;
}

trait IterIntOps<NodeType: NextNode + IntOpAtLevel> {
    fn iter_execute(&self, input: usize) -> CompositeIterator<'_, NodeType>;
}

impl IntOpAtLevel for () {
    fn execute_at_level(&self, _input: usize, _level: usize) -> Option<usize> {
        None
    }
}

impl<A: IntOp, B: NextNode + IntOpAtLevel + NestLevel> IntOpAtLevel for Node<A, B> {
    fn execute_at_level(&self, input: usize, level: usize) -> Option<usize> {
        if level == self.nest_level() {
            Some(self.data.execute(input))
        } else {
            self.next.execute_at_level(input, level)
        }
    }
}

impl<Nodes: NextNode + IntOpAtLevel + NestLevel> IterIntOps<Nodes> for Composite<Nodes> {
    fn iter_execute(&self, input: usize) -> CompositeIterator<'_, Nodes> {
        CompositeIterator::new(&self.head, input, self.head.nest_level())
    }
}

struct CompositeIterator<'a, Nodes: NextNode + IntOpAtLevel> {
    level: usize,
    input: usize,
    parent: &'a Nodes,
}

impl<'a, Nodes: NextNode + IntOpAtLevel> CompositeIterator<'a, Nodes> {
    fn new(parent: &'a Nodes, input: usize, max_level: usize) -> Self {
        Self {
            parent,
            input,
            level: max_level,
        }
    }
}

impl<'a, Nodes: NextNode + IntOpAtLevel> Iterator for CompositeIterator<'a, Nodes> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.parent.execute_at_level(self.input, self.level);
        if self.level > 0 {
            self.level -= 1
        };
        result
    }
}

struct Adder<const VALUE: usize> {}

impl<const VALUE: usize> Adder<VALUE> {
    fn new() -> Self {
        Self {}
    }
}

impl<const VALUE: usize> IntOp for Adder<VALUE> {
    #[inline]
    fn execute(&self, input: usize) -> usize {
        input >> VALUE
    }
}

fn bench_composed<NodeType, Composed>(input: usize, composed: &Composed) -> usize
where
    NodeType: IntOpAtLevel + NextNode,
    Composed: IterIntOps<NodeType>,
{
    composed.iter_execute(input).sum()
}

fn bench_trait_objects(input: usize, ops: &Vec<Box<dyn IntOp>>) -> usize {
    ops.iter().map(|op| op.execute(input)).sum()
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Adders");

    let adders_dyn: Vec<Box<dyn IntOp>> = vec![
        Box::new(Adder::<0>::new()),
        Box::new(Adder::<1>::new()),
        Box::new(Adder::<2>::new()),
        Box::new(Adder::<3>::new()),
        Box::new(Adder::<4>::new()),
        Box::new(Adder::<5>::new()),
        Box::new(Adder::<6>::new()),
        Box::new(Adder::<7>::new()),
        Box::new(Adder::<8>::new()),
        Box::new(Adder::<9>::new()),
        Box::new(Adder::<10>::new()),
        Box::new(Adder::<11>::new()),
        Box::new(Adder::<12>::new()),
        Box::new(Adder::<13>::new()),
    ];

    let adders = compose!(
        Adder::<0>::new(),
        Adder::<1>::new(),
        Adder::<2>::new(),
        Adder::<3>::new(),
        Adder::<4>::new(),
        Adder::<5>::new(),
        Adder::<6>::new(),
        Adder::<7>::new(),
        Adder::<8>::new(),
        Adder::<9>::new(),
        Adder::<10>::new(),
        Adder::<11>::new(),
        Adder::<12>::new(),
        Adder::<13>::new()
    );

    group.bench_function("static", |b| b.iter(|| bench_composed(black_box(20), &adders)));
    group.bench_function("dynamic", |b| {
        b.iter(|| bench_trait_objects(black_box(20), &adders_dyn))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
