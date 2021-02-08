pub trait System<'a> {
    type SystemData;
    fn run(&mut self, data: Self::SystemData);
}