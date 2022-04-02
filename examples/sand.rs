use ledge::event;
use ledge::input;
use ledge::interface::*;
use ledge::graphics::{self, image};
use ledge::error::GameResult;
use rand::{thread_rng, Rng};

#[derive(Clone)]
struct SandPixel {
    image: image::Image,
    updated: bool,
    draw_info: graphics::DrawInfo,
}

impl SandPixel {
    pub fn new(ctx: &graphics::context::GraphicsContext, size: usize, color: graphics::Color) -> Self {
        let mut draw_info = graphics::DrawInfo::new();
        draw_info.scale(1./(size as f32/2.));
        Self {
            image: image::Image::from_color(ctx, color),
            updated: false,
            draw_info,
        }
    }
}

struct MainState {
    particles: Vec<Vec<Option<SandPixel>>>,
    size: usize,
}

impl event::EventHandler for MainState {
    fn update(&mut self, interface: &mut Interface) -> GameResult {
        let mut updated = Vec::new();

        if let Some(button) = interface.mouse_context.current_pressed {
            let x = ((1. + interface.mouse_context.last_position.0) * (1./2.) * self.size as f64) as usize;
            let y = ((1. + interface.mouse_context.last_position.1) * (1./2.) * self.size as f64) as usize;
            
            if !(x > self.size-1 || y > self.size-1) && self.particles[x][y].is_none(){
                if button == input::mouse::MouseButton::Left {
                    self.particles[x][y] = Some(SandPixel::new(&interface.graphics_context, self.size, graphics::Color::rgba(194, 168, 128, 255)));
                }
            }
        }

        let mut rng = thread_rng();
        let n: u32 = rng.gen_range(0..10);

        for i in 0..self.particles.len() {
            if i >= self.particles.len() {
                break;
            }
            for j in 0..self.particles[i].len() {
                if j >= self.particles[i].len()-1 || self.particles[i][j].is_none() || updated.contains(&(i, j)) {
                    continue;
                }

                if self.particles[i][j+1].is_none() {
                    self.particles[i][j+1] = self.particles[i][j].take();        
                    updated.push((i,j+1));
                } else if i < self.size-1 && self.particles[i+1][j+1].is_none() && n > 6 {
                    self.particles[i+1][j+1] = self.particles[i][j].take();
                    updated.push((i+1,j+1));
                } else if i > 0 && self.particles[i-1][j+1].is_none() && n > 6  {
                    self.particles[i-1][j+1] = self.particles[i][j].take();
                    updated.push((i-1,j+1));
                }
            }
        }

        Ok(())
    }
    
    fn draw(&mut self, interface: &mut Interface) -> GameResult {
        let n = self.size as f32;

        for i in 0..self.particles.len() {
            for j in 0..self.particles[i].len() {
                if let Some(pixel) = &mut self.particles[i][j] {
                    pixel.draw_info.dest((i as f32 - n/2.) / (n/2.), ((j) as f32 - n/2.) / (n/2.), 0.);
                    graphics::draw(&mut interface.graphics_context, &pixel.image, pixel.draw_info.clone());
                }
            }
        }

        Ok(())
    }

    fn resize(&mut self, _width: u32, _height: u32) -> GameResult {
        Ok(())
    }
}

impl MainState {
    pub fn new(_ctx: &graphics::context::GraphicsContext, n: usize) -> Self {
        let mut v = Vec::new();
        for _ in 0..n {
            let mut nv: Vec<Option<SandPixel>> = Vec::new();
            nv.resize(n as usize, None);
            v.push(nv);
        }

        Self {
            particles: v,
            size: n,
        }
    }
}

fn main() {
    let builder = InterfaceBuilder::new("sand", "author");
    let (interface, event_loop) = builder.build().unwrap();
    let state = MainState::new(&interface.graphics_context, 256);
    event::run(interface, event_loop, state);
}