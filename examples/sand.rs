use ledge_engine::conf;
use ledge_engine::event;
use ledge_engine::interface::*;
use ledge_engine::graphics::image;
use ledge_engine::graphics;
use ledge_engine::error::GameResult;

use cgmath::Vector2;

#[derive(Clone)]
struct SandPixel {
    size: f32,
    image: image::Image,
}

impl SandPixel {
    pub fn new(ctx: &graphics::context::GraphicsContext, size: f32, color: graphics::Color) -> Self {
        Self {
            size,
            image: image::Image::from_color(ctx, color),
        }
    }
}

struct MainState {
    particles: Vec<Vec<Option<SandPixel>>>,
}

impl event::EventHandler for MainState {
    fn update(&mut self, interface: &mut Interface) -> GameResult {
        for i in 0..self.particles.len() {
            for j in 0..self.particles[i].len() {
                if j == self.particles[i].len()-1 {
                    continue;
                }
                if self.particles[i+1][j].is_none() {
                    self.particles[i+1][j] = self.particles[i][j].take();
                } else if self.particles[i+1][j+1].is_none() {
                    self.particles[i+1][j+1] = self.particles[i][j].take();
                } else if self.particles[i+1][j-1].is_none() {
                    self.particles[i+1][j-1] = self.particles[i][j].take();
                }
            }
        }

        Ok(())
    }
    
    fn draw(&mut self, interface: &mut Interface) -> GameResult {
        for i in 0..self.particles.len() {
            for j in 0..self.particles[i].len() {
                if let Some(pixel) = &mut self.particles[i][j] {
                    let mut draw_info = graphics::DrawInfo::new();
                    draw_info.scale(2./64.);
                    draw_info.translate(i as f32, j as f32, 10 as f32);
                    graphics::draw(&mut interface.graphics_context, &pixel.image, draw_info);
                }
            }
        }

        Ok(())
    }
}

impl MainState {
    pub fn new(ctx: &graphics::context::GraphicsContext) -> Self {
        let mut v = Vec::new();
        v.resize(64, Vec::new());
        for i in 0..63 {
            let mut nv = Vec::new();
            nv.resize(new_len: usize, value: T)
        }
        v[5][10] = Some(SandPixel::new(ctx, 2./64., graphics::Color::black()));
        Self{
            particles: v,
        }
    }
}

fn main() {
    let builder = InterfaceBuilder::new("sand", "author");
    let (interface, event_loop) = builder.build().unwrap();
    let state = MainState::new(&interface.graphics_context);
    event::run(interface, event_loop, state);
}