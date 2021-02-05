use std::collections::HashMap;
use std::any::{Any, TypeId};
use std::cell::RefCell;
use std::marker::PhantomData;
use crate::component::Component;
use std::collections::hash_map::Entry;
use std::borrow::BorrowMut;
use std::cell::RefMut;

pub struct World {
    resources: HashMap<ResourceId, RefCell<Box<dyn Resource>>>
}

impl World {
    pub fn new() -> Self {
        Self {
            resources: HashMap::new(),
        }
    }

    pub fn register<T: Component>(&mut self) 
    where
        T::Storage: Default,
    {
        
        self.register_with_storage::<_, T>(Default::default);
    }

    pub fn register_with_storage<F, T>(&mut self, storage: F)
    where
        F: FnOnce() -> T::Storage,
        T: Component,
    {
        // self.entry().or_insert_with(move || CompStorage::<T>::new(storage()));
    }

    pub fn insert<R>(&mut self, resource: R)
    where 
        R: Resource,
    {
        self.insert_by_id(ResourceId::new::<R>(), resource)
    }

    pub fn insert_by_id<R>(&mut self, resource_id: ResourceId, resource: R)
    where 
        R: Resource,
    {
        resource_id.assert_type_id::<R>();
        self.resources.insert(resource_id, RefCell::new(Box::new(resource)));
    }

    pub fn remove<R>(&mut self, resource: R)
    where 
        R: Resource,
    {
        self.remove_by_id::<R>(ResourceId::new::<R>());
    }

    pub fn remove_by_id<R>(&mut self, resource_id: ResourceId)
    where 
        R: Resource, 
    {
        resource_id.assert_type_id::<R>();
        self.resources.remove(&resource_id);
    }

    pub fn entry<R>(&mut self) -> ResEntry<R> 
    where 
        R: Resource
    {
        create_entry(self.resources.entry(ResourceId::new::<R>()))
    }

    pub fn fetch<T>(&self) -> &T 
    where
        T: Resource
    {
        self.try_fetch::<T>().unwrap()
    }

    pub fn try_fetch<T>(&self) -> Option<&T> 
    where
        T: Resource
    {
        let resource_type_id = ResourceId::new::<T>();
        if let Some(b) = self.resources.get(&resource_type_id).map(|b| b.borrow().as_any().downcast_ref::<T>()) {
            return b;
        }
        None
    }

    // pub fn fetch_mut<T>(&mut self) -> &mut T 
    // where
    //     T: Resource
    // {
    //     self.try_fetch_mut::<T>().unwrap()
    // }

    // pub fn try_fetch_mut<T>(&mut self) -> Option<&mut T> 
    // where
    //     T: Resource
    // {
    //     let resource_type_id = ResourceId::new::<T>();
    //     if let Some(b) = self.resources.get_mut(&resource_type_id).map(|b| b.borrow_mut().as_any_mut().downcast_mut::<T>()) {
    //         return b;
    //     }
    //     None
    // }
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

    pub fn check_type_id<T: Resource>(&self) -> bool{
        let test_id = ResourceId::new::<T>();
        return test_id.type_id == self.type_id;
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ComponentId {
    type_id: TypeId,
}

pub trait Resource: Any + 'static {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub struct ResEntry<'a, T: 'a> {
    inner: Entry<'a, ResourceId, RefCell<Box<dyn Resource>>>,
    phantom: PhantomData<T>,
}

pub fn create_entry<T>(entry: Entry<ResourceId, RefCell<Box<dyn Resource>>>) -> ResEntry<T> {
    ResEntry {
        inner: entry,
        phantom: PhantomData,
    }
}

impl<'a, T> ResEntry<'a, T> 
where
    T: Resource + 'a
{
    // pub fn or_insert(self, v: T) -> &'a mut Box<T> {
    //     self.or_insert_with(move || v)
    // }

    // pub fn or_insert_with<F>(self, f: F) -> &'a mut Box<T>
    // where
    //     F: FnOnce() -> T,
    // {
    //     let value = self
    //         .inner
    //         .or_insert_with(move || RefCell::new(Box::new(f())));
    //     let inner = RefMut::map(value.borrow_mut(), Box::as_mut);

    //     inner
    // }
}

pub struct CompStorage<C: Component> {
    bitset: Bitset,
    storage: C::Storage,
}

impl<'a, C: Component> CompStorage<C> {
    pub fn new(storage: C::Storage) -> Self {
        Self {
            bitset: Bitset::new(),
            storage
        }
    }
}

impl<'a, C: Component> Resource for CompStorage<C> {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

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