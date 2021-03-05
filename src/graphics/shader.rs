// use std::collections::HashMap;
// use crate::lib::*;
// use std::sync::Arc;
// use vulkano::{
//     framebuffer::{Subpass, RenderPassAbstract},
//     pipeline::{GraphicsPipeline, GraphicsPipelineAbstract},
//     OomError,
// };

// use vulkano::descriptor::descriptor::DescriptorDesc;
// use vulkano::descriptor::descriptor::ShaderStages;
// use vulkano::descriptor::pipeline_layout::PipelineLayoutDesc;
// use vulkano::descriptor::pipeline_layout::PipelineLayoutDescPcRange;
// use vulkano::device::Device;
// use vulkano::device::DeviceExtensions;
// use vulkano::format::Format;
// use vulkano::pipeline::shader::{
//     GraphicsShaderType, ShaderInterfaceDef, ShaderInterfaceDefEntry, ShaderModule,
// };
// use vulkano::swapchain;


// use std::borrow::Cow;
// use std::ffi::CStr;

// // This structure is to store multiple pipelines for different blend modes.
// pub struct PipelineObjectSet {
//     pipelines: HashMap<BlendMode, PipelineState>,
// }

// impl PipelineObjectSet {
//     pub fn new(cap: usize) -> Self {
//         Self {
//             pipelines: HashMap::with_capacity(cap),
//         }
//     }
// }

// pub struct PipelineState {
//     pub pipeline: Arc<dyn GraphicsPipelineAbstract + Send + Sync>,
// }

// impl PipelineState {
//     pub unsafe fn new(device: Arc<vulkano::device::Device>, 
//             render_pass: Arc<dyn RenderPassAbstract + Send + Sync>, 
//             (vs, fs): (ShaderModule, ShaderModule)) -> Self 
//     {
//         let pipeline = Arc::new(
//             GraphicsPipeline::start()
//                 .vertex_input_single_buffer::<Vertex>()
//                 .vertex_shader(vs.graphics_entry_point(
//                     CStr::from_bytes_with_nul_unchecked(b"main\0"),
//                     VertInput,
//                     VertOutput,
//                     VertLayout(ShaderStages {
//                         vertex: true,
//                         ..ShaderStages::none()
//                     }),
//                     GraphicsShaderType::Vertex,
//                 ), ())
//                 .triangle_strip()
//                 .viewports_dynamic_scissors_irrelevant(1)
//                 .fragment_shader(fs.graphics_entry_point(
//                     CStr::from_bytes_with_nul_unchecked(b"main\0"),
//                     FragInput,
//                     FragOutput,
//                     FragLayout(ShaderStages {
//                         fragment: true,
//                         ..ShaderStages::none()
//                     }),
//                     GraphicsShaderType::Fragment,
//                 ), ())
//                 .blend_alpha_blending()
//                 .render_pass(Subpass::from(render_pass.clone(), 0).unwrap())
//                 .build(device.clone())
//                 .unwrap()
//         ) as Arc<dyn GraphicsPipelineAbstract + Send + Sync>;
//         Self {
//             pipeline,
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
//         // There are things to consider when giving out entries:
//         // * There must be only one entry per one location, you can't have
//         //   `color' and `position' entries both at 0..1 locations.  They also
//         //   should not overlap.
//         // * Format of each element must be no larger than 128 bits.
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