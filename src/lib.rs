#![macro_use]
//!Zero_v is an experiment in defining behavior over a collections of
//!objects implementing some trait without dynamic polymorphism.
//!
//!It can be used in contexts where:
//! * Library users will always know the composition of the collection of types at compile time.
//! * Library users should be able to alter collection composition easily.
//! * You're okay with adding a significant chunk of boilerplate to your library internals.
//! * Vtable overhead matters
//!
//!For example, lets imagine you've written an event logging library that
//!allows users to extend it with plugins that alter the event before logging.
//!Using dynamic polymorphism/ vtables, client code might look something like:
//!
//!```ignore
//!let plugins: Vec<Box<dyn Plugin>> = Vec![
//!    Box::new(TimestampReformatter::new()),
//!    Box::new(HostMachineFieldAdder::new()),
//!    Box::new(UserFieldAdder::new()),
//!];
//!
//!let mut logger = EventLogger::with_plugins(plugins);
//!
//!let events = EventStream::new();
//!for event in events.listen() {logger.log_event()};
//!```
//!This is a fine way to approach the problem in most cases. It's easy for
//!clients to set up and you rarely care about the overhead of the virtual
//!calls.
//!
//!But if you do care about overhead, a zero_v version of the above looks
//!like:
//!
//!```ignore
//!use zero_v::{compose, compose_nodes};
//!let plugins = compose!(
//!    TimestampReformatter::new(),
//!    HostMachineFieldAdder::new(),
//!    UserFieldAdder::new()
//!);
//!
//!let mut logger = EventLogger::with_plugins(plugins);
//!
//!let events = EventStream::new();
//!for event in events.listen() {logger.log_event()};/
//!```
//!
//!To the client the only real difference here is the use of the compose macro,
//!dropping the boxing for each plugin in the collection
//!and the extra zero_v imports. But internally, your type is now generic over
//!a type defined without the use of boxing or vtables, which encourages the
//!compiler to monomorphise the plugin use and remove the virtual
//!function calls.
//!
//!## Implementing Zero_v for your type
//!
//! To enable zero_v, you'll need to add a pretty large chunk of boilerplate
//! to your library. This code walks you through it step by step
//! for a simple example.
//!
//!```
//!use zero_v::{Composite, NestLevel, NextNode, Node};
//!
//!// This is the trait we want members of client collections to implement.
//!trait IntOp {
//!    fn execute(&self, input: usize) -> usize;
//!}
//!
//!// First, you'll need a level execution trait. It will have one method
//!// which extends the signature of your trait's core function with an extra
//!// paremeter of type usize (called level here) and wraps the output in an
//!// option (these changes will allow us to return the outputs of the function
//!// from an iterator over the collection.
//!trait IntOpAtLevel {
//!    fn execute_at_level(&self, input: usize, level: usize) -> Option<usize>;
//!}
//!
//!// You'll need to implement this level execution trait for two types,
//!// The first type is Node<A, B> where A implements your basic trait and B
//!// implements the level execution trait and NestLevel. For this type, just
//!// copy the body of the function below, updating the contents of the if/else
//!// blocks with the signature of your trait's function.
//!impl<A: IntOp, B: NextNode + IntOpAtLevel + NestLevel> IntOpAtLevel for Node<A, B> {
//!    fn execute_at_level(&self, input: usize, level: usize) -> Option<usize> {
//!        if level == self.nest_level() {
//!            Some(self.data.execute(input))
//!        } else {
//!            self.next.execute_at_level(input, level)
//!        }
//!    }
//!}
//!// The second type is the unit type. For this implementation, just return None.
//!impl IntOpAtLevel for () {
//!    fn execute_at_level(&self, _input: usize, _level: usize) -> Option<usize> {
//!        None
//!    }
//!}
//!
//!// Next you'll need to create an iterator type for collections implementing
//!// your trait. The iterator will have one field for each argument to your
//!// trait's function, along with a level field and a parent reference to
//!// a type implementing your level execution trait and NextNode.
//!struct CompositeIterator<'a, Nodes: NextNode + IntOpAtLevel> {
//!    level: usize,
//!    input: usize,
//!    parent: &'a Nodes,
//!}
//!
//!// Giving your iterator a constructor is optional.
//!impl<'a, Nodes: NextNode + IntOpAtLevel> CompositeIterator<'a, Nodes> {
//!    fn new(parent: &'a Nodes, input: usize, max_level: usize) -> Self {
//!        Self {
//!            parent,
//!            input,
//!            level: max_level,
//!        }
//!    }
//!}
//!
//!// You'll need to implement Iterator for the iterator you just defined.
//!// The item type will be the return type of your function. For next, just
//!// copy the body of next below, replacing execute_at_level with the 
//!// signature of your execute_at_level function.
//!impl<'a, Nodes: NextNode + IntOpAtLevel> Iterator for CompositeIterator<'a, Nodes> {
//!    type Item = usize;
//!
//!    fn next(&mut self) -> Option<Self::Item> {
//!        let result = self.parent.execute_at_level(self.input, self.level);
//!        if self.level > 0 {
//!            self.level -= 1
//!        };
//!        result
//!    }
//!}
//!
//!// Almost done. Now you'll need to define a trait returning your iterator
//!// type. 
//!trait IterExecute<Nodes: NextNode + IntOpAtLevel + NestLevel> {
//!    fn iter_execute(&self, input: usize) -> CompositeIterator<'_, Nodes>;
//!}
//!
//!// Finally, implement your iterator return trait on a composite over Nodes
//!// bound by NextNode, NestLevel and your level execution trait which returns
//!// your iterator.
//!impl<Nodes: NextNode + IntOpAtLevel + NestLevel>IterExecute<Nodes> for Composite<Nodes> {
//!    fn iter_execute(&self, input: usize) -> CompositeIterator<'_, Nodes> {
//!        CompositeIterator::new(&self.head, input, self.head.nest_level())
//!    }
//!}
//!```

mod composite;
mod nest;
#[cfg(test)]
mod test;

pub use composite::{Composite, Node, NextNode};
pub use nest::NestLevel;
