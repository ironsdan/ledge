use std::cell::{Ref, RefCell, RefMut};
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;

use crate::graphics::{
    context::GraphicsContext, image::Image, BlendMode, Color, DrawInfo, Drawable, InstanceData,
    Rect, Vertex, QUAD_VERTICES,
};

pub trait DocumentElement: Drawable {
    fn draw_element(&self, ctx: &mut GraphicsContext, style: &DocumentElementStyles);
}

pub struct DocumentElementStyles {
    pub positioning: bool,
    pub position: (f32, f32, f32),
    pub background_color: Color,
    pub padding: (f32, f32),
    pub font_size: u32,
    pub letter_spacing: u32,
    pub line_height: u32,
    pub width: f32,
    pub height: f32,
}

impl DocumentElementStyles {
    fn combine(
        current: &DocumentElementStyles,
        parent: &DocumentElementStyles,
    ) -> DocumentElementStyles {
        DocumentElementStyles {
            positioning: current.positioning,
            position: (
                current.position.0 + parent.position.0,
                current.position.1 + parent.position.1,
                current.position.2 + parent.position.2,
            ),
            background_color: current.background_color,
            padding: current.padding,
            font_size: current.font_size,
            letter_spacing: current.letter_spacing,
            line_height: current.line_height,
            width: current.width,
            height: current.height,
        }
    }
}

impl Default for DocumentElementStyles {
    fn default() -> DocumentElementStyles {
        DocumentElementStyles {
            positioning: false,
            position: (0.0, 0.0, 0.0),
            background_color: Color::transparent(),
            padding: (0.0, 0.0),
            font_size: 16,
            letter_spacing: 0,
            line_height: 16,
            width: 1.0,
            height: 1.0,
        }
    }
}

pub struct DocumentNode {
    id: String,
    descendants: Vec<Rc<RefCell<DocumentNode>>>,
    pub style: DocumentElementStyles,
    pub inner: Box<dyn DocumentElement>,
}

impl DocumentNode {
    pub fn new(id: &str, element: Box<dyn DocumentElement>) -> Self {
        Self {
            id: id.to_string(),
            descendants: Vec::new(),
            style: DocumentElementStyles::default(),
            inner: element,
        }
    }

    pub fn descendants_mut(&mut self) -> &mut Vec<Rc<RefCell<DocumentNode>>> {
        &mut self.descendants
    }

    pub fn descendants(&self) -> &Vec<Rc<RefCell<DocumentNode>>> {
        &self.descendants
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn draw(&self, ctx: &mut GraphicsContext, parent_style: &DocumentElementStyles) {
        let final_style = DocumentElementStyles::combine(&self.style, parent_style);

        self.inner.draw_element(ctx, &final_style);

        for d in self.descendants.iter() {
            d.borrow().draw(ctx, &final_style);
        }
    }
}

pub struct DocumentContext<'a> {
    pub root: Rc<RefCell<DocumentNode>>,
    pub ids: HashMap<&'a str, Rc<RefCell<DocumentNode>>>,
}

impl<'a> DocumentContext<'a> {
    pub fn new() -> Self {
        let mut ids: HashMap<&str, Rc<RefCell<DocumentNode>>> = HashMap::new();

        let root = Rc::new(RefCell::new(DocumentNode::new(
            "root",
            Box::new(Div::new()),
        )));

        ids.insert("root", root.clone());

        Self {
            root: root,
            ids: ids,
        }
    }

    pub fn insert(&mut self, parent_id: &str, id: &'a str, value: Box<dyn DocumentElement>) {
        let element = Rc::new(RefCell::new(DocumentNode::new(id, value)));

        self.ids.insert(id, element.clone());

        self.ids
            .get(parent_id)
            .unwrap()
            .borrow_mut()
            .descendants_mut()
            .push(element);
    }

    pub fn select(&self, id: &str) -> Ref<DocumentNode> {
        self.ids.get(id).unwrap().borrow()
    }

    pub fn select_mut(&mut self, id: &str) -> RefMut<DocumentNode> {
        self.ids.get(id).unwrap().borrow_mut()
    }
}

impl<'a> Drawable for DocumentContext<'a> {
    fn draw(&self, ctx: &mut GraphicsContext, _info: DrawInfo) {
        self.root
            .borrow()
            .draw(ctx, &DocumentElementStyles::default());
    }
}

pub struct Div {}

impl Div {
    pub fn new() -> Self {
        Self {}
    }
}

impl DocumentElement for Div {
    fn draw_element(&self, ctx: &mut GraphicsContext, style: &DocumentElementStyles) {
        let image = Image::from_color(ctx, Color::white());
        let mut info = DrawInfo::default();

        info.color(style.background_color);
        info.dest(style.position.0, style.position.1, style.position.2);

        let verts: [Vertex; 4] = Rect {
            x: 0.0,
            y: 0.0,
            w: style.width,
            h: style.height,
        }
        .into();

        ctx.update_vertex_data(verts.to_vec());

        ctx.pipe_data
            .sampled_image(0, image.inner().clone(), ctx.samplers[0].clone());

        self.draw(ctx, info.into());
    }
}

impl Drawable for Div {
    fn draw(&self, ctx: &mut GraphicsContext, info: DrawInfo) {
        ctx.update_instance_properties(Arc::new(vec![info.into()]));
        ctx.set_blend_mode(BlendMode::Alpha);
        ctx.draw();
    }
}

pub struct Text {
    font: Arc<Font>,
    inner: String,
}

impl Text {
    pub fn with_font(font: Arc<Font>) -> Self {
        Self {
            font: font,
            inner: "".to_string(),
        }
    }

    pub fn text(mut self, text: String) -> Self {
        self.inner = text;
        self
    }
}

impl Drawable for Text {
    fn draw(&self, ctx: &mut GraphicsContext, _info: DrawInfo) {
        ctx.update_vertex_data(QUAD_VERTICES.to_vec());

        // Add texture to pipe data
        ctx.pipe_data.sampled_image(
            0,
            self.font.sheet().inner().clone(),
            ctx.samplers[0].clone(),
        );

        // Set blend mode
        ctx.set_blend_mode(BlendMode::Alpha);

        // call ctx draw with none
        ctx.draw();
    }
}

impl DocumentElement for Text {
    fn draw_element(&self, ctx: &mut GraphicsContext, style: &DocumentElementStyles) {
        let mut v = Vec::new();
        let mut i = 0;
        let mut j = 0;
        for r in self.inner.chars() {
            if r == ' ' {
                i += 1;
                continue;
            }

            if r == '\n' {
                j += 1;
                i = 0;
                continue;
            }

            let coords = self.font.map(&r);
            let mut info = DrawInfo::with_rect(Rect {
                x: coords.0 / self.font.width,
                y: coords.1 / self.font.height,
                w: 1. / self.font.width,
                h: 1. / self.font.height,
            });

            let ruin_size = style.font_size as f32 / 600.0;
            let ruin_spacing = style.letter_spacing as f32 / 800.0;
            let ruin_separation = (i as f32) * (ruin_size + ruin_spacing);

            let line_spacing = style.line_height as f32 / 600.0;
            let line_separation = (j as f32) * (ruin_size + line_spacing) + line_spacing;

            info.translate(
                ruin_separation + style.position.0,
                line_separation + style.position.1,
                0.0,
            );
            info.scale(ruin_size);

            let data: InstanceData = info.into();
            v.push(data);
            i += 1;
        }

        ctx.update_instance_properties(Arc::new(v));

        self.draw(ctx, DrawInfo::default());
    }
}

pub struct Font {
    image: Image,
    width: f32,
    height: f32,
}

impl Font {
    pub fn map(&self, r: &char) -> (f32, f32) {
        let i: u32 = (*r).into();
        let x = (i - 64 - 1) % self.width as u32;
        let y = (i - 64 - 1) / self.width as u32;
        (x as f32, y as f32)
    }

    pub fn sheet(&self) -> &Image {
        &self.image
    }

    pub fn new(i: Image, w: u32, h: u32) -> Self {
        Self {
            image: i,
            width: w as f32,
            height: h as f32,
        }
    }
}
