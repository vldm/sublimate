use core::Core;
/// Trait Consume Core, and run event loop
pub trait Application:Sized {
   type Error;
   
   fn new(core: Core) -> Result<Self, Self::Error>;
   fn run(&mut self);
}
