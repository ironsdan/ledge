use crate::scene::*;
use crate::graphics::context::GraphicsContext;
use crate::ecs;

pub struct SpaceStack<C> {
    pub scenes: Vec<Box<dyn Space<C>>>,
}

impl<C> SpaceStack<C> {
    pub fn new() -> Self {
        Self {
            scenes: Vec::new(),
        }  
    }

    pub fn push(&mut self, scene: Box<Space<C>>) {
        self.scenes.push(scene)
    }

    pub fn draw(&mut self, world: &mut World, context: &mut GraphicsContext) {
        for scene in self.scenes.iter_mut() {
            scene.draw(world, context).unwrap();
        }
    }

    pub fn update(&mut self) {

    }
}

pub enum SpaceSwitch<C> {
    None,
    Push(Box<Space<C>>),
    Replace(Box<Space<C>>),
    Pop,
}

impl<C> SpaceSwitch<C> {
    pub fn replace<S>(scene: S) -> Self
    where
        S: Space<C> + 'static,
    {
        SpaceSwitch::Replace(Box::new(scene))
    }

    pub fn push<S>(scene: S) -> Self
    where
        S: Space<C> + 'static,
    {
        SpaceSwitch::Push(Box::new(scene))
    }
}
