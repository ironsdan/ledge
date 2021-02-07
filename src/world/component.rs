use std::any::Any;
use crate::storage::DynamicStorage;

pub trait Component: Any + Sized {
    type Storage: DynamicStorage<Self> + Any + Default;
}