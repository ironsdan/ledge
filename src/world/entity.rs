use crate::world::World;
use crate::component::Component;
use crate::storage::{
    WriteStorage,
    SystemData,
};

pub struct EntityBuilder<'a> {
    pub entity: Entity,
    pub world: &'a World,
    pub built: bool,
}

impl<'a> EntityBuilder<'a> {
    pub fn with<C: Component>(self, component: C) -> Self {
        {
            let mut storage: WriteStorage<C> = SystemData::fetch(&self.world);
            storage.insert(self.entity, c).unwrap();
        }
        self
    }

    pub fn build(mut self) -> Entity {
        self.built = true;
        self.entity
    }
}

pub struct Entity {
    id: u32,
    generation: Generation,
}

pub struct Entities {
    
}

pub struct Generation {}