use std::collections::HashMap;

pub struct World {
    resources: HashMap<ResourseId, Vec<Box<dyn Resource>>>
}

pub struct ResourseId {
    res_type: ResourceType,
    id: u32
}

pub trait Resource {

}

pub enum ResourceType {
    
}