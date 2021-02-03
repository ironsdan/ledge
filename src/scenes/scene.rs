pub enum SceneSwitch<C, Ev> {
    None,
    Push(Box<Scene<C, Ev>>),
    Replace(Box<Scene<C, Ev>>),
    Pop,
}

pub trait Scene<C, E> {
    fn update(&mut self, gameworld: &mut C, ctx: &mut ggez::Context) -> SceneSwitch<C, Ev>;
    fn draw(&mut self, gameworld: &mut C, ctx: &mut ggez::Context) -> ggez::GameResult<()>;
    fn input(&mut self, gameworld: &mut C, event: Ev, started: bool);
}

pub struct SceneStack<C, Ev> {
    pub world: C,
    scenes: Vec<Box<Scene<C, E>>>,
}

