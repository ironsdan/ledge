use crate::scene::*;
use crate::graphics::context::GraphicsContext;

pub struct SceneStack<C> {
    pub scenes: Vec<Box<dyn Scene<C>>>,
}

impl<C> SceneStack<C> {
    pub fn new() -> Self {
        Self {
            scenes: Vec::new(),
        }  
    }

    pub fn push(&mut self, scene: Box<dyn Scene<C>>) {
        self.scenes.push(scene)
    }

    // pub fn draw(&mut self, world: &mut World, context: &mut GraphicsContext) {
    //     for scene in self.scenes.iter_mut() {
    //         scene.draw(world, context).unwrap();
    //     }
    // }

    // pub fn update(&mut self, world: &mut World, interface: &mut Interface) {
    //     for scene in self.scenes.iter_mut() {
    //         scene.update(interface, world);
    //     }
    // }
}

pub enum SceneSwitch<C> {
    None,
    Push(Box<dyn Scene<C>>),
    Replace(Box<dyn Scene<C>>),
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
