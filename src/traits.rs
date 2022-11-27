use std::fmt::Debug;

pub trait TState<T>
where
    T: Debug,
{
    fn entry(&mut self, _context: &mut T) {}
    fn exit(&mut self, _context: &mut T) {}
    fn update(&mut self, _context: &mut T) {}
}
