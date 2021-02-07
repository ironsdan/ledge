use crate::world::*;

pub trait System<'a> {
    type SystemData;
    fn run(&mut self, data: Self::SystemData);
}

// pub trait DynSystemData<'a> {
//     type Accessor: Accessor;

//     // fn setup(accessor: &Self::Accessor, world: &mut World);
//     fn fetch(accessor: &Self::Accessor, world: &'a World) -> Self;
// }

// pub trait Accessor {

// }