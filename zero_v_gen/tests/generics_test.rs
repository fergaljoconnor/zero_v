use zero_v::*;

#[zero_v(trait_types)]
trait MyTrait<'a, I, R>
where
    I: Copy,
{
    fn apply(&self, input: &'a I) -> R;
}

#[derive(Debug)]
struct Plus(usize);

impl<'a> MyTrait<'a, usize, usize> for Plus {
    fn apply(&self, input: &'a usize) -> usize {
        input + self.0
    }
}

#[derive(Debug)]
struct PlusLen<'a>(&'a str);

impl<'a> MyTrait<'a, usize, usize> for PlusLen<'a> {
    fn apply(&self, input: &'a usize) -> usize {
        input + self.0.len()
    }
}

#[zero_v(fn_generics, MyTrait as MyTraits)]
fn apply<'a, I, R>(t: &MyTraits, input: &'a I) -> Vec<R>
where
    I: Copy,
{
    t.iter_apply(input).collect()
}

#[test]
fn test_generic_trait() {
    let impls = compose!(Plus(1), PlusLen("a"));

    let results: Vec<usize> = apply(&impls, &100);
    assert_eq!(results, vec![101, 101]);
}

#[test]
fn test_generic_manual_iter() {
    let impls = compose!(Plus(1), PlusLen("ab"));

    let mapped: Vec<_> = (0..impls.len())
        .filter_map(|index| impls.apply_at_level(&100, index))
        .collect();

    assert_eq!(vec![101, 102], mapped);
}
