/*
The current structure of the composite system is
similar to a linked list but which each node containing a type instead
of an object. So effectively, we build a list of types at compile
time, terminating with the unit type, and use that list to build a nested
data structure where each level of nesting is responsible for one of the
types in that list (the most deeply nested level only containing the unit
type).

To understand how this is handy,lets go through the usage path, from here
to the application writer.

1. The zero_v library: Uses this to define generic collections of objects.
2. The intermediate library: Uses the nesting level and recursion to define an
   iterator/ some other output over any collection of objects implementing
   their trait which executes one or more functions on each object at each
   level and combines the outputs in some way.
3. The application author: Defines the specific collection of objects at
   compile time and uses the behavior defined by the intermediate library
   to get some result or to plug that collection into some type exposed
   by the intermediate library.
*/

/// A type representing a collection of zero or more objects.
#[derive(Debug, PartialEq)]
pub struct Composite<A: NextNode> {
    /// Can be of any type implementing the NextNode trait. Typically this will
    /// be a node whose `next` field implements NextNode (representing a
    /// collection of one or more nested nodes) or the unit type
    /// (representing an empty composite).
    pub head: A,
}

impl<A: NextNode> Composite<A> {
    /// Generates a new Composite
    ///
    /// # Arguments
    ///
    /// * `head` - The first node in the data structure or the unit type.
    pub fn new(head: A) -> Self {
        Self { head }
    }
}

/// Represents a collection of one or more objects.
#[derive(Debug, PartialEq)]
pub struct Node<A, B: NextNode> {
    /// The object held in this node
    pub data: A,
    /// Next is any type implementing the NextNode trait. Typically this will
    /// be a node whose next field also implements NextNode (representing a
    /// collection of one or more nested node) or the unit type
    /// (representing an empty composite).
    pub next: B,
}

impl<A, B: NextNode> Node<A, B> {
    /// Build a new node
    ///
    /// # Arguments
    ///
    /// * `data` - The object held in this node in the composite
    /// * `next` - The next node in the data structure.
    pub fn new(data: A, next: B) -> Self {
        Self { data, next }
    }
}

impl<A> Node<A, ()> {
    /// Build a new Node where the next field is the unit type.
    pub fn base(data: A) -> Self {
        Self { data, next: () }
    }
}

/// A Marker trait for types which can be nested in a node's next field
/// or Composite's head field. Implemented for the unit type
/// or a Node whose next field implements NextNode.
// Basically what we're doing here is building something similar to an option
// using traits. Obviously, an enum does not suit this situation since an enum
// is used in cases where we don't know whether the object will be of variant
// X or Y at compile time. In this case, we don't know this information while
// writing this library, but the library user will know the exact type of
// NextNode at compile time.
pub trait NextNode {}
impl NextNode for () {}
impl<A, B: NextNode> NextNode for Node<A, B> {}

/// Takes a list of objects and uses them to build a nested node object
/// with one of the original objects contained in the data field of each node.
///
/// # Example usage
/// ```
/// use zero_v::{compose_nodes, Node};
///
/// let nodes = compose_nodes!(1, 2);
/// assert_eq!(nodes, Node::new(1, Node::new(2, ())));
/// ```
#[macro_export]
macro_rules! compose_nodes {
    () => {
        ()
    };
    ($val: expr) => {
       $crate::Node::base($val)
    };
    ($left: expr, $($right: expr), +) => {
        $crate::Node::new($left, $crate::compose_nodes!( $($right), +))
    };
}

/// Takes a list of objects and uses them to build a composite
/// with one of the original objects contained in the data field of each node
/// (or a single unit type if the list is empty).
///
/// # Example usage
/// ```
/// use zero_v::{compose, Composite, Node};
///
/// let nodes = compose!(1, 2);
/// assert_eq!(nodes, Composite::new(Node::new(1, Node::base(2))));
/// ```
#[macro_export]
macro_rules! compose {
    ($($right: expr), *) => {
        $crate::Composite::new($crate::compose_nodes!( $($right), *))
    };
}

#[cfg(test)]
mod test {
    use super::{Composite, Node};
    #[test]
    fn can_build_composites_with_compose_macro() {
        assert_eq!(compose!(), Composite::new(()));
        assert_eq!(compose!(0), Composite::new(Node::base(0)));
        assert_eq!(compose!(0, 1), Composite::new(Node::new(0, Node::base(1))));
        assert_eq!(
            compose!(0, 1, 2),
            Composite::new(Node::new(0, Node::new(1, Node::base(2))))
        );
    }
}
