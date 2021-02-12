use crate::{
    ecs::component::Component,
    ecs::{Fetch, FetchMut},
    ecs::entity::Entities,
    ecs::entity::Entity,
    ecs::join::Joinable,
};
use std::{
    marker::PhantomData,
    ops::{Deref, DerefMut}
};

// Stores the bitset (used for joining) and the physical storage for the component.
pub struct TrackedStorage<C: Component> {
    pub bitset: LayeredBitMap,
    pub inner: C::Storage,
}

impl<C: Component> TrackedStorage<C> {
    pub fn new(storage: C::Storage) -> Self {
        Self {
            bitset: LayeredBitMap::new(),
            inner: storage,
        }
    }

    pub fn insert(&mut self, index: usize, value: C) {
        self.inner.insert(index, value);
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

pub trait DynamicStorage<T>: TryDefault {
    fn insert(&mut self, index: usize, value: T);
    fn get(&self, id: usize) -> &T;
}

impl<T> DynamicStorage<T> for VecStorage<T> {
    fn insert(&mut self, index: usize, value: T) {
        self.inner.push(value);
        // if index < self.inner.len() {
        //     println!("Inserting in dyn storage entity with id: {}", index);
        //     self.inner.insert(index, value); // TODO this seems like a bad way to do it.
        // } else if index == 0 && self.inner.len() == 0 {
        //     println!("Inserting in dyn storage when 0 entity with id: {}", index);
        //     self.inner.insert(index, value); // TODO this seems like a bad way to do it.
        // }
    }

    fn get(&self, id: usize) -> &T {
        self.inner.get(id).unwrap()
    }
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

// Inspired by hibitset and amethyst this is a hierarchial bitset that is used for speedy joining.
#[derive(Default)]
pub struct LayeredBitMap {
    pub layer1: Vec<u16>,
    pub layer0: Vec<u32>,
    max_index: usize,
}

impl LayeredBitMap {
    pub fn new() -> Self {
        Self {
            layer1: Vec::new(),
            layer0: Vec::new(),
            max_index: 0,
        }
    }

    pub fn insert(&mut self, index: usize) {
        if index > self.max_index {
            for _ in 0..((index / 32) + 1 - self.layer0.len()) {
                self.layer0.push(0);
            }
            for _ in 0..((index / 16) + 1 - self.layer1.len()) {
                self.layer1.push(0);
            }
            self.max_index = index;
        }
        self.layer0[index / 32] = self.layer0[index / 32] ^ 1 << (index % 32);
        self.layer1[index / 16 - 1] = self.layer1[index / 16 - 1] ^ 1 << (index % 16);
    }

    pub fn remove(&mut self, index: usize) {
        self.layer0[index / 32] = self.layer0[index / 32] ^ 1 << (index % 32);
    }

    pub fn check(&self, index: usize) -> bool {
        if index > self.max_index {
            return false;
        }
        if self.layer1[index / 16] & (1 << (index % 16)) != 0  {
            println!("layer1 check: true");
        }
        (self.layer0[index / 32] & (1 << (index % 32))) != 0
    }
}

// Currently only used for component storage reads. TODO impl
pub struct Storage<'a, T, D> {
    pub data: D,
    pub entities: Entities<'a>,
    pub phantom: PhantomData<T>,
}

impl<'a, T, D> Storage<'a, T, D>
where 
    D: DerefMut<Target = TrackedStorage<T>>,
    T: Component,
    {
    pub fn insert(&mut self, entity: Entity, value: T) {
        // println!("Inserting in storage entity with id: {}", entity.id());
        self.data.insert(entity.id(), value);
    }

    pub fn get(&self, e: Entity) -> Option<&T> {
        Some(self.data.inner.get(e.id()))
        // if self.data.bitset.contains(e.id()) && self.entities.is_alive(e) {
        //     Some(self.data.inner.get(e.id()))
        // } else {
        //     None
        // }
    }
}

impl<'a, 'b, T, D> Joinable for &'a Storage<'b, T, D>
where
    T: Component,
    D: Deref<Target = TrackedStorage<T>> 
{
    type Value = &'a T::Storage;
    type Type = &'a T;

    fn get_values(&self) -> Self::Value {
        &self.data.inner
    }
}

// pub trait SystemStorage {}

pub type ReadStorage<'a, T> = Storage<'a, T, Fetch<'a, TrackedStorage<T>>>;

pub type WriteStorage<'a, T> = Storage<'a, T, FetchMut<'a, TrackedStorage<T>>>;