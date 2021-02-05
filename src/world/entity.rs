use crate::world::World;

pub struct EntityBuilder<'a> {
    pub entity: Entity,
    pub world: &'a World
}

pub struct Entity {
    id: u32,
    generation: Generation,
}

pub struct Generation {}