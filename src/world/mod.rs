pub mod entity;
pub mod component;
pub mod storage;
pub mod system;

use crate::world::component::Component;
use crate::world::storage::TrackedStorage;

use std::collections::HashMap;
use std::any::{TypeId};
use std::cell::RefCell;
use std::marker::PhantomData;
use std::collections::hash_map::Entry;
use std::cell::Ref;
use std::cell::RefMut;
use std::ops::{Deref, DerefMut};
use mopa::Any;

mod __resource_mopafy_scope {
    use mopa::mopafy;
    use super::Resource;
    mopafy!(Resource);
}

pub struct World {
    resources: HashMap<ResourceId, RefCell<Box<dyn Resource>>>,
}

impl World {
    pub fn new() -> Self {
        Self {
            resources: HashMap::new(),
        }
    }

    pub fn register<C: Component>(&mut self)
    where
        C::Storage: Default
    {
        self.register_with_storage::<_, C>(Default::default);
    }

    pub fn register_with_storage<F, C>(&mut self, storage: F)
    where
        F: FnOnce() -> C::Storage,
        C: Component,
    {
        println!("Registering Component with Type Id: {:?}", ResourceId::new::<C>().type_id);
        self.resources.insert(ResourceId::new::<C>(), RefCell::new(Box::new(TrackedStorage::<C>::new(storage()))));
    }

    pub fn fetch<R: Resource>(&self) -> Fetch<R> {
        Fetch {
            inner: self.resources.get(&ResourceId::new::<R>()).unwrap().borrow(),
            phantom: PhantomData
        }
    }

    pub fn fetch_mut<R: Resource>(&mut self) -> FetchMut<R> {
        FetchMut {
            inner: self.resources.get(&ResourceId::new::<R>()).unwrap().borrow_mut(),
            phantom: PhantomData
        }
    }

    pub fn entry<R: Resource>(&mut self) -> ResEntry<R> {
        create_entry::<R>(self.resources.entry(ResourceId::new::<R>()))
    }

    pub fn insert<R: Resource>(&mut self, resource: R) {
        self.insert_by_id(ResourceId::new::<R>(), resource)
    }

    pub fn insert_by_id<R: Resource>(&mut self, resource_id: ResourceId, resource: R) {
        resource_id.assert_type_id::<R>();
        println!("Inserting resource with Type Id: {:?}", ResourceId::new::<R>().type_id);
        self.resources.insert(resource_id, RefCell::new(Box::new(resource)));
    }

    pub fn remove<R: Resource>(&mut self) {
        self.remove_by_id::<R>(ResourceId::new::<R>());
    }

    pub fn remove_by_id<R: Resource>(&mut self, resource_id: ResourceId) {
        resource_id.assert_type_id::<R>();
        self.resources.remove(&resource_id);
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ResourceId {
    type_id: TypeId,
}

impl ResourceId {
    pub fn new<T: Resource>() -> Self {
        Self {
            type_id: TypeId::of::<T>()
        }
    }

    pub fn assert_type_id<T: Resource>(&self) {
        let test_id = ResourceId::new::<T>();
        assert_eq!(test_id.type_id, self.type_id);
    }

    pub fn check_type_id<T: Resource>(&self) -> bool {
        let test_id = ResourceId::new::<T>();
        return test_id.type_id == self.type_id;
    }
}

pub trait Resource: Any + 'static {}

impl<T> Resource for T where T: Any {}

// impl Downcast for dyn Resource {}

pub struct ResEntry<'a, T: 'a> {
    pub inner: Entry<'a, ResourceId, RefCell<Box<dyn Resource>>>,
    phantom: PhantomData<T>,
}

pub fn create_entry<T>(entry: Entry<ResourceId, RefCell<Box<dyn Resource>>>) -> ResEntry<T> {
    ResEntry {
        inner: entry,
        phantom: PhantomData,
    }
}

pub struct Fetch<'a, T: 'a> {
    pub inner: Ref<'a, Box<dyn Resource>>,
    pub phantom: PhantomData<&'a T>,
}

pub struct FetchMut<'a, T: 'a> {
    pub inner: RefMut<'a, Box<dyn Resource>>,
    pub phantom: PhantomData<&'a mut T>,
}

impl<'a, T> Deref for Fetch<'a, T>
where
    T: Resource,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe{ self.inner.downcast_ref_unchecked() }
    }
}

impl<'a, T> Deref for FetchMut<'a, T>
where
    T: Resource,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe{ self.inner.downcast_ref_unchecked() }
    }
}

impl<'a, T> DerefMut for FetchMut<'a, T>
where
    T: Resource,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe{ self.inner.downcast_mut_unchecked() }
    }
}