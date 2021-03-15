use std::collections::HashMap;
use std::sync::Arc;
use vulkano::{
    framebuffer::{Subpass, RenderPassAbstract},
    pipeline::{GraphicsPipeline, GraphicsPipelineAbstract},
    OomError,
};

use vulkano::descriptor::descriptor::DescriptorDesc;
use vulkano::descriptor::descriptor::ShaderStages;
use vulkano::descriptor::pipeline_layout::PipelineLayoutDesc;
use vulkano::descriptor::pipeline_layout::PipelineLayoutDescPcRange;
use vulkano::device::Device;
use vulkano::device::DeviceExtensions;
use vulkano::format::Format;
use vulkano::pipeline::shader::{
    GraphicsShaderType, ShaderInterfaceDef, ShaderInterfaceDefEntry, ShaderModule,
};
use vulkano::swapchain;

use vulkano::pipeline::shader::GraphicsEntryPointAbstract;

use crate::graphics::{Vertex, BlendMode};

use std::borrow::Cow;
use std::ffi::CStr;

use crate::graphics::vs;
use crate::graphics;

pub struct Shaders {
    pub vs: vs::Shader
}

// impl Shaders {
//     pub fn load(device: Arc<Device>) -> Result<Self, OomError> {
//         Ok(Self {
//             vs: vs::Shader::load(device)?,
//         })
//     }
// }

// impl<'a, I, O, L> ShaderGeneric<'a, I, O, L> for Shaders {
//     fn entry_point(&self) -> vulkano::pipeline::shader::GraphicsEntryPoint<(), Box<ShaderInterfaceDef<Iter = VertInputIter>>, Box<ShaderInterfaceDef<Iter = FragInputIter>>, Box<PipelineLayoutDesc>> {
//         self.vs.main_entry_point()
//     }
// }

// pub trait ShaderGeneric<'a, I, O, L> {
//     fn entry_point(&self) -> vulkano::pipeline::shader::GraphicsEntryPoint<(), Box<ShaderInterfaceDef<Iter = VertInputIter>>, Box<ShaderInterfaceDef<Iter = FragInputIter>>, Box<PipelineLayoutDesc>>;
// }

// This structure is to store multiple pipelines for different blend modes.
pub struct PipelineObjectSet {
    pipelines: HashMap<BlendMode, PipelineObject>,
}

impl PipelineObjectSet {
    pub fn new(cap: usize) -> Self {
        Self {
            pipelines: HashMap::with_capacity(cap),
        }
    }

    pub fn insert(&mut self, blend_mode: BlendMode, pipeline: PipelineObject) {
        self.pipelines.insert(blend_mode, pipeline);
    }

    pub fn get(&self, blend_mode: &BlendMode) -> Option<&PipelineObject> {
        self.pipelines.get(blend_mode)
    }
}

// pub trait PipelineStateGeneric {

// }

pub struct PipelineObject {
    pub pipeline: Arc<dyn GraphicsPipelineAbstract + Send + Sync>,
}

impl PipelineObject {
    pub fn new(pipeline: Arc<dyn GraphicsPipelineAbstract + Send + Sync>) -> Self {
        Self {
            pipeline,
        }
    }
}

// impl  PipelineState {
//     pub fn new<I, O, L>(device: Arc<vulkano::device::Device>, 
//             render_pass: Arc<dyn RenderPassAbstract + Send + Sync>, 
//             vs_bytes: &[u8], fs_bytes: &[u8], vs_entry: Box<dyn ShaderGeneric<I, O, L>>) -> Self 
//     {
//         unsafe {
//             let vs: Arc<ShaderModule> = ShaderModule::new(device.clone(), vs_bytes).unwrap();
//             let fs: Arc<ShaderModule> = ShaderModule::new(device.clone(), fs_bytes).unwrap();

//             let vertex_shader_entry = vs.graphics_entry_point(
//                 CStr::from_bytes_with_nul_unchecked(b"main\0"),
//                 VertInput,
//                 VertOutput,
//                 VertLayout(ShaderStages {
//                     vertex: true,
//                     ..ShaderStages::none()
//                 }),
//                 GraphicsShaderType::Vertex,
//             );

//             let fragment_shader_entry = fs.graphics_entry_point(
//                 CStr::from_bytes_with_nul_unchecked(b"main\0"),
//                 FragInput,
//                 FragOutput,
//                 FragLayout(ShaderStages {
//                     fragment: true,
//                     ..ShaderStages::none()
//                 }),
//                 GraphicsShaderType::Fragment,
//             );

//             let pipeline = Arc::new(
//                 GraphicsPipeline::start()
//                     .vertex_input_single_buffer::<Vertex>()
//                     .vertex_shader(vertex_shader_entry, ())
//                     .triangle_strip()
//                     .viewports_dynamic_scissors_irrelevant(1)
//                     .fragment_shader(fragment_shader_entry, ())
//                     .blend_alpha_blending()
//                     .render_pass(Subpass::from(render_pass.clone(), 0).unwrap())
//                     .build(device.clone())
//                     .unwrap()
//             ) as Arc<dyn GraphicsPipelineAbstract + Send + Sync>;
//             Self {
//                 pipeline,
//             }
//         }
//     }
// }

// // This structure will tell Vulkan how input entries of our vertex shader look like
// #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
// struct VertInput;

// unsafe impl ShaderInterfaceDef for VertInput {
//     type Iter = VertInputIter;

//     fn elements(&self) -> VertInputIter {
//         VertInputIter(0)
//     }
// }

// #[derive(Debug, Copy, Clone)]
// struct VertInputIter(u16);

// impl Iterator for VertInputIter {
//     type Item = ShaderInterfaceDefEntry;

//     #[inline]
//     fn next(&mut self) -> Option<Self::Item> {
//         if self.0 == 0 {
//             self.0 += 1;
//             return Some(ShaderInterfaceDefEntry {
//                 location: 1..2,
//                 format: Format::R32G32B32Sfloat,
//                 name: Some(Cow::Borrowed("color")),
//             });
//         }
//         if self.0 == 1 {
//             self.0 += 1;
//             return Some(ShaderInterfaceDefEntry {
//                 location: 0..1,
//                 format: Format::R32G32Sfloat,
//                 name: Some(Cow::Borrowed("position")),
//             });
//         }
//         None
//     }

//     #[inline]
//     fn size_hint(&self) -> (usize, Option<usize>) {
//         // We must return exact number of entries left in iterator.
//         let len = (2 - self.0) as usize;
//         (len, Some(len))
//     }
// }

// impl ExactSizeIterator for VertInputIter {}

// #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
// struct VertOutput;

// unsafe impl ShaderInterfaceDef for VertOutput {
//     type Iter = VertOutputIter;

//     fn elements(&self) -> VertOutputIter {
//         VertOutputIter(0)
//     }
// }

// // This structure will tell Vulkan how output entries (those passed to next
// // stage) of our vertex shader look like.
// #[derive(Debug, Copy, Clone)]
// struct VertOutputIter(u16);

// impl Iterator for VertOutputIter {
//     type Item = ShaderInterfaceDefEntry;

//     #[inline]
//     fn next(&mut self) -> Option<Self::Item> {
//         if self.0 == 0 {
//             self.0 += 1;
//             return Some(ShaderInterfaceDefEntry {
//                 location: 0..1,
//                 format: Format::R32G32B32Sfloat,
//                 name: Some(Cow::Borrowed("v_color")),
//             });
//         }
//         None
//     }

//     #[inline]
//     fn size_hint(&self) -> (usize, Option<usize>) {
//         let len = (1 - self.0) as usize;
//         (len, Some(len))
//     }
// }

// impl ExactSizeIterator for VertOutputIter {}

// // This structure describes layout of this stage.
// #[derive(Debug, Copy, Clone)]
// struct VertLayout(ShaderStages);
// unsafe impl PipelineLayoutDesc for VertLayout {
//     // Number of descriptor sets it takes.
//     fn num_sets(&self) -> usize {
//         0
//     }
//     // Number of entries (bindings) in each set.
//     fn num_bindings_in_set(&self, _set: usize) -> Option<usize> {
//         None
//     }
//     // Descriptor descriptions.
//     fn descriptor(&self, _set: usize, _binding: usize) -> Option<DescriptorDesc> {
//         None
//     }
//     // Number of push constants ranges (think: number of push constants).
//     fn num_push_constants_ranges(&self) -> usize {
//         0
//     }
//     // Each push constant range in memory.
//     fn push_constants_range(&self, _num: usize) -> Option<PipelineLayoutDescPcRange> {
//         None
//     }
// }

// // Same as with our vertex shader, but for fragment one instead.
// #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
// struct FragInput;
// unsafe impl ShaderInterfaceDef for FragInput {
//     type Iter = FragInputIter;

//     fn elements(&self) -> FragInputIter {
//         FragInputIter(0)
//     }
// }
// #[derive(Debug, Copy, Clone)]
// struct FragInputIter(u16);

// impl Iterator for FragInputIter {
//     type Item = ShaderInterfaceDefEntry;

//     #[inline]
//     fn next(&mut self) -> Option<Self::Item> {
//         if self.0 == 0 {
//             self.0 += 1;
//             return Some(ShaderInterfaceDefEntry {
//                 location: 0..1,
//                 format: Format::R32G32B32Sfloat,
//                 name: Some(Cow::Borrowed("v_color")),
//             });
//         }
//         None
//     }

//     #[inline]
//     fn size_hint(&self) -> (usize, Option<usize>) {
//         let len = (1 - self.0) as usize;
//         (len, Some(len))
//     }
// }

// impl ExactSizeIterator for FragInputIter {}

// #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
// struct FragOutput;
// unsafe impl ShaderInterfaceDef for FragOutput {
//     type Iter = FragOutputIter;

//     fn elements(&self) -> FragOutputIter {
//         FragOutputIter(0)
//     }
// }

// #[derive(Debug, Copy, Clone)]
// struct FragOutputIter(u16);

// impl Iterator for FragOutputIter {
//     type Item = ShaderInterfaceDefEntry;

//     #[inline]
//     fn next(&mut self) -> Option<Self::Item> {
//         // Note that color fragment color entry will be determined
//         // automatically by Vulkano.
//         if self.0 == 0 {
//             self.0 += 1;
//             return Some(ShaderInterfaceDefEntry {
//                 location: 0..1,
//                 format: Format::R32G32B32A32Sfloat,
//                 name: Some(Cow::Borrowed("f_color")),
//             });
//         }
//         None
//     }
//     #[inline]
//     fn size_hint(&self) -> (usize, Option<usize>) {
//         let len = (1 - self.0) as usize;
//         (len, Some(len))
//     }
// }

// impl ExactSizeIterator for FragOutputIter {}

// // Layout same as with vertex shader.
// #[derive(Debug, Copy, Clone)]
// struct FragLayout(ShaderStages);
// unsafe impl PipelineLayoutDesc for FragLayout {
//     fn num_sets(&self) -> usize {
//         0
//     }
//     fn num_bindings_in_set(&self, _set: usize) -> Option<usize> {
//         None
//     }
//     fn descriptor(&self, _set: usize, _binding: usize) -> Option<DescriptorDesc> {
//         None
//     }
//     fn num_push_constants_ranges(&self) -> usize {
//         0
//     }
//     fn push_constants_range(&self, _num: usize) -> Option<PipelineLayoutDescPcRange> {
//         None
//     }
// }