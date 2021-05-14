#[derive(Debug, PartialEq)]
pub struct Composite<A: NextNode> {
    pub head: A,
}

impl<A: NextNode> Composite<A> {
    pub fn new(head: A) -> Self {
        Self { head }
    }
}

#[derive(Debug, PartialEq)]
pub struct Node<A, B: NextNode> {
    pub data: A,
    pub next: B,
}

pub trait NextNode {}

impl NextNode for () {}
impl<A, B: NextNode> Node<A, B> {
    pub fn new(data: A, next: B) -> Self {
        Self { data, next }
    }
}

impl<A> Node<A, ()> {
    pub fn base(data: A) -> Self {
        Self { data, next: () }
    }
}

impl<A, B: NextNode> NextNode for Node<A, B> {}

#[macro_export]
macro_rules! compose_nodes {
    () => {
        ()
    };
    ($val: expr) => {
       Node::base($val)
    };
    ($left: expr, $($right: expr), +) => {
        Node::new($left, compose_nodes!( $($right), +))
    };
}

#[macro_export]
macro_rules! compose {
    ($($right: expr), *) => {
        Composite::new(compose_nodes!( $($right), *))
    };
}

#[cfg(test)]
mod test {
    use super::{Composite, Node};
    #[test]
    fn output() {
        assert_eq!(compose!(), Composite::new(()));
        assert_eq!(compose!(0), Composite::new(Node::base(0)));
        assert_eq!(compose!(0, 1), Composite::new(Node::new(0, Node::base(1))));
        assert_eq!(
            compose!(0, 1, 2),
            Composite::new(Node::new(0, Node::new(1, Node::base(2))))
        );
    }
}
