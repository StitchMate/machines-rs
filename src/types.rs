pub type Condition<T> = fn(&T) -> bool;

pub type Action<T> = fn(&mut T) -> ();
