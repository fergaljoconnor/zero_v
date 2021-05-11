#![macro_use]

mod composite;
mod nest;
mod linked;

#[derive(Debug, PartialEq)]
struct Collection<A, B> {
    left: A,
    right: B,
}


impl<A, B> Collection<A, B> {
    fn new(left: A, right: B) -> Self {
        Collection { left, right }
    }

    fn compose<T>(self, new: T) -> Collection<Self, T> {
        Collection::new(self, new)
    }
}

