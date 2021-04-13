use crate::graphics::*;
use crate::graphics::context::GraphicsContext;
use crate::scene::*;
use crate::scene::stack::*;
use crate::timer::*;

pub struct LevelBuilder {
    pub(crate) inner: Level,
    pub(crate) is_built: bool,
}

impl LevelBuilder {
    pub fn new() -> Self {
        Self {
            inner: Level::default(),
            is_built: false,
        }
    }

    pub fn from_conf(&mut self) {}

    pub fn with(&mut self) {}

    // pub fn with_entity(mut self, entity: Entity) -> Self {
    //     self.inner.entities.push(entity);
    //     self
    // }

    pub fn from_conf_file(&mut self) {}

    pub fn build(self) -> Level {
        self.inner
    }
}

#[derive(Default, Clone)]
pub struct Level {
}

// impl Scene for LevelScene {
    
    // fn update(&mut self, interface: &mut Interface, world: &mut World) -> SceneSwitch<World> {
    //     SceneSwitch::None
    // }

    // fn draw(&mut self, world: &mut World, context: &mut GraphicsContext) -> Result<(), > {
    //     Ok(())
    // }

    // fn input(&mut self, _gameworld: &mut World, _started: bool) {

    // }

    // fn current_scene(&self) -> bool {
    //     true
    // }
// }

