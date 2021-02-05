use std::any::Any;

pub trait Component: Any {
    type Storage;
}