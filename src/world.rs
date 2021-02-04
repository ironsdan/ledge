use std::collections::HashMap;
use std::collections::hash_map::Iter;
use std::any::{Any, TypeId};


pub struct World {
    resources: HashMap<ResourceId, Box<dyn Resource>>
}

impl World {
    pub fn new() -> Self {
        Self {
            resources: HashMap::new(),
        }
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

}

pub enum ResourceType {
    
}