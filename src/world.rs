use std::collections::HashMap;
use std::any::{Any, TypeId};

use crate::component::Component;


pub struct World {
    resources: HashMap<ResourceId, Box<dyn Resource>>
}

impl World {
    pub fn new() -> Self {
        Self {
            resources: HashMap::new(),
        }
    }

    pub fn register<C>(&mut self) 
    where
        C: Component
    {
        // self.insert()
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
        self.resources.insert(resource_id, Box::new(resource));
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
        if let Some(b) = self.resources.get(&resource_type_id).map(|b| b.as_any().downcast_ref::<T>()) {
            return b;
        }
        None
    }

    pub fn fetch_mut<T>(&mut self) -> &mut T 
    where
        T: Resource
    {
        self.try_fetch_mut::<T>().unwrap()
    }

    pub fn try_fetch_mut<T>(&mut self) -> Option<&mut T> 
    where
        T: Resource
    {
        let resource_type_id = ResourceId::new::<T>();
        if let Some(b) = self.resources.get_mut(&resource_type_id).map(|b| b.as_any_mut().downcast_mut::<T>()) {
            return b;
        }
        None
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

    pub fn check_type_id<T: Resource>(&self) -> bool{
        let test_id = ResourceId::new::<T>();
        return test_id.type_id == self.type_id;
    }
}

pub trait Resource: Any + 'static {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub enum ResourceType {
    
}