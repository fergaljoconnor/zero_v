use std::marker::PhantomData;

/*
Level is a wrapper around an array index. The purpose is to make
zero_v function execute_at_level function signatures robust to users
mixing up argument order in the case that their function signature
takes a usize argument. It also guarantees that the index you're passing in
maps to a valid level for your composite.

For example, lets imagine you have a trait that has a function with signature
do_x(&self, input: usize). Then the derived do_x_at_level function
would have signature do_x_at_level(&self, input: usize, level: usize).
This is the problem. There is nothing at the type system level preventing
the library user from mixing these two arguments up since they both have the
same type. Level is designed to act as a slot-in replacement, so you end up
with signature do_x_at_level(&self, input: usize, level: Level) and the
compiler can protect you if you ever mix the order up.

Making the level generic over your type solves another problem. If level
wasn't specific to the type it will be used on, then you could take a level
from composite A and use it on composite B. If the level from composite
A didn't exist in composite B, you could get a value of None (if using Option
returns) or a panic (if unwrapping). To prevent this, the only way to
get a level for a composite is to call iter_levels() on an instance. This means
that, as long as the levels returned by iter_level for a composite are always
valid for that type, you can safely unwrap your return value/ are guaranteed
there is an element at that level.
*/

/// The level of an object in a collection
#[derive(Clone, Debug, PartialEq)]
#[repr(transparent)]
pub struct Level<T> {
    ///The index of the target object
    value: usize,
    phantom: PhantomData<T>,
}

impl<T> Level<T> {
    pub(crate) fn new(value: usize) -> Self {
        Self {
            value,
            phantom: PhantomData {},
        }
    }
    pub fn value(&self) -> usize {
        self.value
    }
}
