use std::any::Any;

pub trait Component: Any + Sized {
    type Storage;
}