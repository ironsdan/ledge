// mod lib;
// use::lib::Rect;

struct CollisionWorld {
    collidables: Vec<Collidable>,
    entities: Vec<Collidable>,
    num_collidables: u16,
    
}

fn check_bodies(rect1: Rect, rect2: Rect) -> bool {
    let mut collision = false;
    
    if rect1.x < rect2.x + rect2.width() &&
       rect1.x + rect1.width() > rect2.x &&
       rect1.y < rect2.y + rect2.height() &&
       rect1.y + rect1.height() > rect2.y {
           collision = true;
    }

    return collision;
}

impl CollisionWorld {
    pub fn step(& mut self) {
        for entity in self.entities.iter_mut() {
            for collidable in self.collidables.iter() {
                // if check_bodies(entity.bounding_box, collidable.bounding_box) {
                //     entity.grounded = true;
                //     entity.bounding_box.y = collidable.bounding_box.y - entity.bounding_box.height() + 1.0;
                // } else {
                //     entity.grounded = false;
                // }
            }
        }
    }
}

struct Collidable {
    obj_type: CollidableObj,
    id: u32,
    name: String,
    pub bounding_box: Rect,
    pub grounded: bool,
}

struct BoundingBox {
    height: u32,
    width: u32,
    x: u32,
    y: u32,
}

pub enum CollidableObj {
    Solid,
    Platform,
    Entity,
}

pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub height: f32,
    pub width: f32,
}

impl Rect {
    pub fn width(&self) -> f32 {
        return self.width;
    }

    pub fn height(&self) -> f32 {
        return self.height;
    }
}