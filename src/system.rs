use crate::world::*;

pub trait System<'a> {
    type SystemData: DynSystemData<'a>;
    fn run(&mut self, data: Self::SystemData);

    // fn running_time(&self) -> RunningTime;
    // fn accessor(&'b self) -> AccessorCow<'a, 'b, Self>;
    // fn setup(&mut self, world: &mut World);
    // fn dispose(self, world: &mut World);
}

pub trait DynSystemData<'a> {

}