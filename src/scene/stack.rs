use crate::scene::*;
use crate::ecs;

pub struct SceneStack<C> {
    scenes: Vec<Box<dyn Scene<C>>>,
}

impl<C> SceneStack<C> {
    pub fn new() -> Self {
        Self {
            scenes: Vec::new(),
        }  
    }

    pub fn push(&mut self, scene: Box<Scene<C>>) {
        self.scenes.push(scene)
    }
}

pub enum SceneSwitch<C> {
    None,
    Push(Box<Scene<C>>),
    Replace(Box<Scene<C>>),
    Pop,
}

impl<C> SceneSwitch<C> {
    pub fn replace<S>(scene: S) -> Self
    where
        S: Scene<C> + 'static,
    {
        SceneSwitch::Replace(Box::new(scene))
    }

    pub fn push<S>(scene: S) -> Self
    where
        S: Scene<C> + 'static,
    {
        SceneSwitch::Push(Box::new(scene))
    }
}
