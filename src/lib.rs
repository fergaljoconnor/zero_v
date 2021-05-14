#![macro_use]
///Zero_v is a crate for defining behavior over collections of objects
///implementing some trait with members defined at compile time without
///dynamic polymorphism. This can be handy in
///contexts where vtable overhead matters but being able to vary the
///code for this collection easily is a problem.
///
///For example, when you want to make your type/function generic over some
///collection defined by the library user, a dynamic way to do it might be:
///
///```rust
///struct Event {
///}
///
///trait Plugin {
///  fn handle_event(&self, event: &Event);
///}
///
///struct EventLogger {
///  plugins: Vec<Box<dyn Plugin>>
///}
///
///impl EventLogger {
///  fn log_event(&self, event: &Event) {
///      for plugin in &self.plugins {
///        plugin.handle_event(event)
///      };
///  }
///}
///
///```

mod composite;
mod nest;
#[cfg(test)]
mod test;

pub use composite::{Composite, Node, NextNode};
pub use nest::NestLevel;
