use crate::component::Component;
use crate::world::Resource;
use crate::world::{Fetch, FetchMut};
use crate::entity::Entities;

use std::marker::PhantomData;

// pub trait CompStorage<T> {
// }

pub struct TrackedStorage<C: Component> {
    pub bitset: Bitset,
    pub inner: C::Storage,
}

pub struct VecStorage<T> {
    inner: Vec<T>,
}

impl<T> Default for VecStorage<T> {
    fn default() -> Self {
        Self{
            inner: Default::default()
        }
    }
}

pub trait TryDefault: Sized {
    /// Tries to create the default.
    fn try_default() -> Result<Self, String>;

    /// Calls `try_default` and panics on an error case.
    fn unwrap_default() -> Self {
        match Self::try_default() {
            Ok(x) => x,
            Err(e) => panic!("Failed to create a default value for storage ({:?})", e),
        }
    }
}

impl<T> TryDefault for T
where
    T: Default,
{
    fn try_default() -> Result<Self, String> {
        Ok(T::default())
    }
}

pub trait DynamicStorage<T>: TryDefault {

}

impl<T> DynamicStorage<T> for VecStorage<T> {

} 

impl<C: Component> Default for TrackedStorage<C>
where
    C::Storage: Default
{
    fn default() -> Self {
        Self {
            bitset: Default::default(),
            inner: Default::default(),
        }
    }
}

impl<C: Component> AnyStorage for TrackedStorage<C> {

}

#[derive(Default)]
pub struct Bitset {

}

impl Bitset {
    pub fn new() -> Self {
        Self {

        }
    }
}

pub trait AnyStorage {

}

pub struct Storage<'a, T, D> {
    pub data: D,
    entities: Fetch<'a, Entities>,
    phantom: PhantomData<T>,
}

pub type ReadStorage<'a, T> = Storage<'a, T, Fetch<'a, TrackedStorage<T>>>;

pub type WriteStorage<'a, T> = Storage<'a, T, FetchMut<'a, TrackedStorage<T>>>;