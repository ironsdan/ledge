use std::any::Any;
use crate::ecs::storage::DynamicStorage;

pub trait Component: Any + Sized {
    type Storage: DynamicStorage<Self> + Any + Default;
}