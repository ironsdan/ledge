use crate::ecs::World;
use crate::ecs::component::Component;
use crate::ecs::storage::{
    // WriteStorage,
    Bitset,
    // SystemData,
};

pub struct EntityBuilder<'a> {
    pub entity: Entity,
    pub world: &'a World,
    pub built: bool,
}

impl<'a> EntityBuilder<'a> {
    pub fn new(entity: Entity, world: &'a World) -> Self {
        Self {
            entity,
            world,
            built: false,
        }
    }

    pub fn with<C: Component>(self, component: C) -> Self {
        {
            // let mut storage: WriteStorage<C> = SystemData::fetch(&self.world);
            // storage.insert(self.entity, c).unwrap();
        }
        self
    }

    pub fn build(mut self) -> Entity {
        self.built = true;
        self.entity
    }
}

// The resource in the world that stores all entities.
// pub struct EntitiesMaster {
//     pub controller: EntityController,
// }

// The controller for every entity keeps track of operational information.
pub struct EntityController {
    max_id: usize,
    generations: Vec<Generation>,
    alive: Bitset,
    killed: Bitset,
}

impl EntityController {
    pub fn new() -> Self {
        Self {
            max_id: 0,
            generations: Vec::new(),
            alive: Bitset::new(),
            killed: Bitset::new(),
        }
    }

    pub fn next_id(&mut self) -> usize {
        let next_id = self.max_id;
        self.max_id += 1;
        next_id
    }

    pub fn create_entity(&mut self) -> Entity {
        let id = self.next_id();
        let generation = self.generations[id];
        Entity {
            id,
            generation
        }
    }
}

impl Default for EntityController {
    fn default() -> Self {
        EntityController {
            max_id: 0,
            generations: Vec::new(),
            alive: Bitset::new(),
            killed: Bitset::new(),
        }
    }
}

// The user seen entity object.
pub struct Entity {
    id: usize,
    generation: Generation,
}

pub struct Entities {
    
}

// impl Resource for Entities

#[derive(Clone, Copy, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub struct Generation {}