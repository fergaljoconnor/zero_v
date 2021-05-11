#[derive(Debug)]
pub struct Composite<A: NextNode> {
    pub head: A,
}

impl<A: NextNode> Composite<A> {
    pub fn new(head: A) -> Self {
        Self { head }
    }
}

#[derive(Debug)]
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


#[test]
fn output() {
    let composed = compose!();
    println!("{:?}", composed);
}
