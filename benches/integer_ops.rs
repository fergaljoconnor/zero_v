use criterion::{black_box, criterion_group, criterion_main, Criterion};
use zero_v::{compose, composed, zero_v_gen};

#[zero_v_gen]
trait IntOp {
    fn execute(&self, input: usize) -> usize;
}

struct Adder {
    value: usize,
}

impl Adder {
    #[inline]
    fn new(value: usize) -> Self {
        Self { value }
    }
}

impl IntOp for Adder {
    fn execute(&self, input: usize) -> usize {
        input + self.value
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
    fn execute(&self, input: usize) -> usize {
        input * self.value
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
    fn execute(&self, input: usize) -> usize {
        input >> self.value
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
    fn execute(&self, input: usize) -> usize {
        input << self.value
    }
}

struct ConstAdder<const VALUE: usize> {}

impl<const VALUE: usize> ConstAdder<VALUE> {
    fn new() -> Self {
        Self {}
    }
}

impl<const VALUE: usize> IntOp for ConstAdder<VALUE> {
    fn execute(&self, input: usize) -> usize {
        input + VALUE
    }
}

struct ConstMultiplier<const VALUE: usize> {}

impl<const VALUE: usize> ConstMultiplier<VALUE> {
    fn new() -> Self {
        Self {}
    }
}

impl<const VALUE: usize> IntOp for ConstMultiplier<VALUE> {
    fn execute(&self, input: usize) -> usize {
        input * VALUE
    }
}

struct ConstRShifter<const VALUE: usize> {}

impl<const VALUE: usize> ConstRShifter<VALUE> {
    fn new() -> Self {
        Self {}
    }
}

impl<const VALUE: usize> IntOp for ConstRShifter<VALUE> {
    fn execute(&self, input: usize) -> usize {
        input >> VALUE
    }
}

struct ConstLShifter<const VALUE: usize> {}

impl<const VALUE: usize> ConstLShifter<VALUE> {
    fn new() -> Self {
        Self {}
    }
}

impl<const VALUE: usize> IntOp for ConstLShifter<VALUE> {
    fn execute(&self, input: usize) -> usize {
        input >> VALUE
    }
}

#[composed(IntOp as IntOps)]
fn bench_composed(input: usize, ops: &IntOps) -> usize {
    ops.iter_execute(input).sum()
}

fn bench_trait_objects(input: usize, ops: &Vec<Box<dyn IntOp>>) -> usize {
    ops.iter().map(|op| op.execute(input)).sum()
}

fn bench_baseline(input: usize) -> usize {
    (input + 0)
        + (input << 1)
        + (input + 2)
        + (input * 3)
        + (input + 4)
        + (input * 5)
        + (input + 6)
        + (input * 7)
        + (input + 8)
        + (input * 9)
        + (input + 10)
        + (input >> 11)
        + (input + 12)
        + (input >> 13)
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Integer Ops");

    let ops_dyn: Vec<Box<dyn IntOp>> = vec![
        Box::new(Adder::new(0)),
        Box::new(LShifter::new(1)),
        Box::new(Adder::new(2)),
        Box::new(Multiplier::new(3)),
        Box::new(Adder::new(4)),
        Box::new(Multiplier::new(5)),
        Box::new(Adder::new(6)),
        Box::new(Multiplier::new(7)),
        Box::new(Adder::new(8)),
        Box::new(Multiplier::new(9)),
        Box::new(Adder::new(10)),
        Box::new(RShifter::new(11)),
        Box::new(Adder::new(12)),
        Box::new(RShifter::new(13)),
    ];

    let ops_dyn_const: Vec<Box<dyn IntOp>> = vec![
        Box::new(ConstAdder::<0>::new()),
        Box::new(ConstLShifter::<1>::new()),
        Box::new(ConstAdder::<2>::new()),
        Box::new(ConstMultiplier::<3>::new()),
        Box::new(ConstAdder::<4>::new()),
        Box::new(ConstMultiplier::<5>::new()),
        Box::new(ConstAdder::<6>::new()),
        Box::new(ConstMultiplier::<7>::new()),
        Box::new(ConstAdder::<8>::new()),
        Box::new(ConstMultiplier::<9>::new()),
        Box::new(ConstAdder::<10>::new()),
        Box::new(ConstRShifter::<11>::new()),
        Box::new(ConstAdder::<12>::new()),
        Box::new(ConstRShifter::<13>::new()),
    ];

    let ops = compose!(
        Adder::new(0),
        LShifter::new(1),
        Adder::new(2),
        Multiplier::new(3),
        Adder::new(4),
        Multiplier::new(5),
        Adder::new(6),
        Multiplier::new(7),
        Adder::new(8),
        Multiplier::new(9),
        Adder::new(10),
        RShifter::new(11),
        Adder::new(12),
        RShifter::new(13)
    );

    let ops_const = compose!(
        ConstAdder::<0>::new(),
        ConstLShifter::<1>::new(),
        ConstAdder::<2>::new(),
        ConstMultiplier::<3>::new(),
        ConstAdder::<4>::new(),
        ConstMultiplier::<5>::new(),
        ConstAdder::<6>::new(),
        ConstMultiplier::<7>::new(),
        ConstAdder::<8>::new(),
        ConstMultiplier::<9>::new(),
        ConstAdder::<10>::new(),
        ConstRShifter::<11>::new(),
        ConstAdder::<12>::new(),
        ConstRShifter::<13>::new()
    );

    group.bench_function("Static/Arg", |b| {
        b.iter(|| bench_composed(black_box(20), black_box(&ops)))
    });

    group.bench_function("Vtable/Arg", |b| {
        b.iter(|| bench_trait_objects(black_box(20), black_box(&ops_dyn)))
    });

    group.bench_function("Static/Const", |b| {
        b.iter(|| bench_composed(black_box(20), black_box(&ops_const)))
    });

    group.bench_function("Vtable/Const", |b| {
        b.iter(|| bench_trait_objects(black_box(20), black_box(&ops_dyn_const)))
    });

    group.bench_function("Baseline", |b| b.iter(|| bench_baseline(black_box(20))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
