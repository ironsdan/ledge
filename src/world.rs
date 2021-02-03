use std::collections::HashMap;


pub struct World {
    resources: HashMap<ResourceId, Vec<Box<dyn Resource>>>
}

impl World {
    pub fn insert<R>(&mut self, resource: R)
        where R: Resource {
            self.insert_by_id(ResourceId::new<R>(), resource)
    }

    pub fn insert_by_id<R>(&mut self, resource_id: ResourceId, resource: R)
        where R: Resource {

    }

    pub fn remove<R>(&mut self, resource: R)
    where R: Resource {
        
    }

    pub fn remove_by_id<R>(&mut self, resource_id: ResourceId, resource: R)
        where R: Resource {
            
    }
}

pub struct ResourceId {
    res_type: ResourceType,
    id: u32
}

pub trait Resource {

}

pub enum ResourceType {
    
}