use crate::{
    ecs::component::Component,
    ecs::{Fetch, FetchMut},
    ecs::entity::Entities,
};
use std::{marker::PhantomData};

// Stores the bitset (used for joining) and the physical storage for the component.
pub struct TrackedStorage<C: Component> {
    pub bitset: Bitset,
    pub inner: C::Storage,
}

impl<C: Component> TrackedStorage<C> {
    pub fn new(storage: C::Storage) -> Self {
        Self {
            bitset: Bitset::new(),
            inner: storage,
        }
    }
}

// Simple vec wrapper, added because in the future there will be other storage types as well.
pub struct VecStorage<T> {
    pub inner: Vec<T>,
}

impl<T> Default for VecStorage<T> {
    fn default() -> Self {
        Self{
            inner: Default::default()
        }
    }
}

// Found this online helps with defaults for some of the types.
pub trait TryDefault: Sized {
    fn try_default() -> Result<Self, String>;

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

pub trait DynamicStorage<T>: TryDefault {}

impl<T> DynamicStorage<T> for VecStorage<T> {} 

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

// Inspired by hibitset and amethyst this is a hierarchial bitset that is used for speedy joining.
#[derive(Default)]
pub struct Bitset {}

impl Bitset {
    pub fn new() -> Self {
        Self {}
    }
}

// Currently only used for component storage reads. TODO impl
pub struct Storage<'a, T, D> {
    pub data: D,
    pub entities: Fetch<'a, Entities>,
    pub phantom: PhantomData<T>,
}

// pub struct SystemData {

// }

// impl SystemData {
//     // pub fn fetch<T: Component>() -> WriteStorage<T> {

//     // }
// }

pub type ReadStorage<'a, T> = Storage<'a, T, Fetch<'a, TrackedStorage<T>>>;

pub type WriteStorage<'a, T> = Storage<'a, T, FetchMut<'a, TrackedStorage<T>>>;