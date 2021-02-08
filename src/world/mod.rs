pub mod entity;
pub mod component;
pub mod storage;
pub mod system;

use crate::world::{
    component::Component,
    storage::{
        TrackedStorage,
        ReadStorage,
        WriteStorage,
    },
    entity::{
        Entities,
        EntityController,
        EntityBuilder,
    }
};
use std::{
    collections::HashMap,
    any::TypeId,
    cell::{
        RefCell,
        Ref,
        RefMut,
    },
    marker::PhantomData,
    collections::hash_map::Entry,
    ops::{Deref, DerefMut}
};
use mopa::Any;

mod __resource_mopafy_scope {
    use mopa::mopafy;
    use super::Resource;
    mopafy!(Resource);
}

// Resource is a very important trait, it is implemented for everything.
pub trait Resource: Any + 'static {}

impl<T> Resource for T where T: Any {}

// Fetch and FetchMut are used to fetch resources from the world.
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


// World: is a resource management tool, combined with an interface for ECS.
pub struct World {
    resources: HashMap<ResourceId, RefCell<Box<dyn Resource>>>,
}

impl World {
    pub fn new() -> Self {
        let mut world = Self::default();
        world.insert(EntityController::default());
        
        world
    }

    // Register is used to add component storage to the world.
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
        // println!("Registering Component with Type Id: {:?}", ResourceId::new::<C>().type_id);
        self.resources.insert(ResourceId::new::<C>(), RefCell::new(Box::new(TrackedStorage::<C>::new(storage()))));
    }

    // Creates the aforementioned Fetch and FetchMut objects.
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

    // Convienience functions for reading/writing the TrackedStorage of a component.
    pub fn read_comp_storage<R>(&self) -> ReadStorage<R>
    where
    R: Resource + Component
    {
        ReadStorage {
            data: Fetch { 
                inner: self.resources.get(&ResourceId::new::<R>()).unwrap().borrow(), 
                phantom: PhantomData,
            },
            entities: Fetch {
                inner: self.entities().inner,
                phantom: PhantomData,
            },
            phantom: PhantomData
        }
    }

    pub fn write_comp_storage<R>(&self) -> WriteStorage<R>
    where
    R: Resource + Component
    {
        WriteStorage {
            data: FetchMut { 
                inner: self.resources.get(&ResourceId::new::<R>()).unwrap().borrow_mut(), 
                phantom: PhantomData,
            },
            entities: Fetch {
                inner: self.entities().inner,
                phantom: PhantomData,
            },
            phantom: PhantomData
        }
    }

    // Creates a wrapper around the raw hash_map::Entry.
    pub fn entry<R: Resource>(&mut self) -> ResEntry<R> {
        create_entry::<R>(self.resources.entry(ResourceId::new::<R>()))
    }

    // insert and remove are used to create/destroy entries in the HashMap of resources.
    pub fn insert<R: Resource>(&mut self, resource: R) {
        self.insert_by_id(ResourceId::new::<R>(), resource)
    }

    pub fn insert_by_id<R: Resource>(&mut self, resource_id: ResourceId, resource: R) {
        resource_id.assert_type_id::<R>();
        // println!("Inserting resource with Type Id: {:?}", ResourceId::new::<R>().type_id);
        self.resources.insert(resource_id, RefCell::new(Box::new(resource)));
    }

    pub fn remove<R: Resource>(&mut self) {
        self.remove_by_id::<R>(ResourceId::new::<R>());
    }

    pub fn remove_by_id<R: Resource>(&mut self, resource_id: ResourceId) {
        resource_id.assert_type_id::<R>();
        self.resources.remove(&resource_id);
    }

    // Convenience function for getting all the entities.
    pub fn entities(&self) -> Fetch<Entities> {
        self.fetch::<Entities>()
    }

    pub fn create_entity(&mut self) -> EntityBuilder {
        let entity = self.fetch_mut::<EntityController>().create_entity();
        EntityBuilder::new(entity, self)
    }
}

impl Default for World {
    fn default() -> Self {
        Self {
            resources: HashMap::new(),
        }
    }
}


// Exactly the same as a TypeId, in the future it may have other uses.
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

// Wrapper for hash_map::Entry.
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