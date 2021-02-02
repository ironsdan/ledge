pub struct World {
    resources: HashMap<ResourseId, Vec<Box<dyn Resource>>>
}

pub struct ResourseId {
    type: ResourceType,
    id: u32
}

pub trait Resourse {

}