use vulkano::{
    command_buffer::{AutoCommandBufferBuilder, PrimaryAutoCommandBuffer},
    device::physical::{PhysicalDevice, PhysicalDeviceType},
    device::{Device, DeviceCreateInfo, DeviceExtensions, QueueCreateInfo},
    image::{view::{ImageView, ImageViewCreateInfo}, ImageUsage, SwapchainImage},
    instance::{Instance, InstanceCreateInfo},
    sampler::{Filter, Sampler, SamplerAddressMode, SamplerCreateInfo},
    swapchain::{self, Surface, PresentMode, AcquireError, Swapchain, SwapchainCreateInfo, SwapchainCreationError},
    sync::{self, FlushError, GpuFuture},
    Version,
    format::Format,
};

use vulkano_win::VkSurfaceBuild;
use winit::event_loop::EventLoop;
use winit::window::{Window, WindowBuilder};

use std::sync::Arc;

use anyhow::{Result, anyhow};

use crate::{
    conf::*, 
    graphics::shader::ShaderId, 
    graphics::*,
};

pub type FinalImageView = Arc<ImageView<SwapchainImage<Window>>>;

pub struct Renderer {
    pub queue: Arc<vulkano::device::Queue>,
    pub(crate) surface: Arc<vulkano::swapchain::Surface<winit::window::Window>>,
    pub device: Arc<vulkano::device::Device>,
    pub(crate) swapchain: Arc<vulkano::swapchain::Swapchain<winit::window::Window>>,
    pub image_views: Vec<FinalImageView>,
    pub(crate) image_num: usize,
    pub(crate) recreate_swapchain: bool,
    pub previous_frame_end: Option<Box<dyn vulkano::sync::GpuFuture>>,
    pub present_future: Option<Box<dyn vulkano::sync::GpuFuture>>,
    pub command_buffer: Option<AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>>,
    pub default_shader: ShaderId,
    pub render_passes: Vec<render_pass::RenderPass>,
    pub samplers: Vec<Arc<Sampler>>,
}

impl Renderer {
    pub fn new(_conf: Conf) -> (Self, winit::event_loop::EventLoop<()>) {
        let required_extensions = vulkano_win::required_extensions();
        let instance = Instance::new(InstanceCreateInfo {
            application_name: None,
            application_version: Version::V1_1,
            enabled_extensions: required_extensions,
            ..Default::default()
        })
        .unwrap();

        let event_loop = EventLoop::new();
        let surface = WindowBuilder::new()
            .build_vk_surface(&event_loop, instance.clone())
            .unwrap();

        let device_extensions = DeviceExtensions {
            khr_swapchain: true,
            ..DeviceExtensions::none()
        };
        let (physical_device, queue_family) = PhysicalDevice::enumerate(&instance)
            .filter(|&p| p.supported_extensions().is_superset_of(&device_extensions))
            .filter_map(|p| {
                p.queue_families()
                    .find(|&q| {
                        q.supports_graphics() && q.supports_surface(&surface).unwrap_or(false)
                    })
                    .map(|q| (p, q))
            })
            .min_by_key(|(p, _)| match p.properties().device_type {
                PhysicalDeviceType::DiscreteGpu => 0,
                PhysicalDeviceType::IntegratedGpu => 1,
                PhysicalDeviceType::VirtualGpu => 2,
                PhysicalDeviceType::Cpu => 3,
                PhysicalDeviceType::Other => 4,
            })
            .unwrap();

        println!(
            "Using device: {} (type: {:?})",
            physical_device.properties().device_name,
            physical_device.properties().device_type,
        );

        let (device, mut queues) = Device::new(
            physical_device,
            DeviceCreateInfo {
                enabled_extensions: physical_device
                    .required_extensions()
                    .union(&device_extensions),
                queue_create_infos: vec![QueueCreateInfo::family(queue_family)],
                ..Default::default()
            },
        )
        .unwrap();

        let queue = queues.next().unwrap();

        let (swapchain, images) = Self::create_swap_chain(
            surface.clone(),
            physical_device,
            device.clone(),
            PresentMode::Immediate
        ).unwrap();

        let default_future = Some(sync::now(device.clone()).boxed());

        let mut samplers = Vec::new();

        let default_sampler = Sampler::new(
            device.clone(),
            SamplerCreateInfo {
                mag_filter: Filter::Linear,
                min_filter: Filter::Linear,
                address_mode: [SamplerAddressMode::Repeat; 3],
                ..Default::default()
            },
        )
        .unwrap();

        samplers.push(default_sampler);

        return (Renderer {
            queue,
            surface,
            device,
            swapchain,
            image_num: 0,
            image_views: images,
            present_future: None,
            previous_frame_end: default_future,
            recreate_swapchain: false,
            command_buffer: None,
            default_shader: 0,
            samplers,
            render_passes: Vec::new(),
        }, event_loop);
    }
    
    fn create_swap_chain(
        surface: Arc<Surface<Window>>,
        physical: PhysicalDevice,
        device: Arc<Device>,
        present_mode: PresentMode,
    ) -> Result<(Arc<Swapchain<Window>>, Vec<FinalImageView>)> {
        let caps = physical
            .surface_capabilities(&surface, Default::default())
            .unwrap();

        // Choosing the internal format that the images will have.
        let image_format = Some(
            physical
                .surface_formats(&surface, Default::default())
                .unwrap()[0]
                .0,
        );

        let (swapchain, images) = {
            Swapchain::new(
                device.clone(),
                surface.clone(),
                SwapchainCreateInfo {
                    min_image_count: caps.min_image_count,
                    image_format: image_format,
                    present_mode,
                    image_extent: surface.window().inner_size().into(),
                    image_usage: ImageUsage::color_attachment(),
                    composite_alpha: caps
                        .supported_composite_alpha
                        .iter()
                        .next()
                        .unwrap(),
        
                    ..Default::default()
                },
            ).unwrap()
        };

        let images = images
            .into_iter()
            .map(|image| {
                let info = ImageViewCreateInfo::from_image(&image);
                ImageView::new(image, info).unwrap()
            })
            .collect::<Vec<_>>();
        Ok((swapchain, images))
    }

    /// Handles setup of a new frame, called when the graphics pipeline is first created and
    /// at the end of every frame to start the next one.
    ///
    /// This is necessary because the swapchain could be out of date,
    /// as well as updating the image_num, optimality, and the swapcahin future.
    pub fn begin_frame(&mut self) -> Result<Box<dyn GpuFuture>> {
        self.previous_frame_end.as_mut().unwrap().cleanup_finished();

        if self.recreate_swapchain {
            let (new_swapchain, new_images) = match self.swapchain.recreate(SwapchainCreateInfo {
                image_extent: self.surface.window().inner_size().into(),
                ..self.swapchain.create_info()
            }) {
                Ok(r) => r,
                Err(SwapchainCreationError::ImageExtentNotSupported { .. }) => {return Err(anyhow!(AcquireError::OutOfDate))},
                Err(e) => panic!("Failed to recreate swapchain: {:?}", e),
            };

            let new_images = new_images
            .into_iter()
            .map(|image| {
                let info = ImageViewCreateInfo::from_image(&image);
                ImageView::new(image, info).unwrap()
            })
            .collect::<Vec<_>>();

            self.image_views = new_images;
            self.swapchain = new_swapchain;
            self.recreate_swapchain = false;
        }

        let (image_num, suboptimal, acquire_future) =
            match swapchain::acquire_next_image(self.swapchain.clone(), None) {
                Ok(r) => r,
                Err(AcquireError::OutOfDate) => {
                    self.recreate_swapchain = true;
                    return Err(anyhow!(AcquireError::OutOfDate));
                }
                Err(e) => panic!("Failed to acquire next image: {:?}", e),
            };

        if suboptimal {
            self.recreate_swapchain = true;
        }

        self.image_num = image_num;

        Ok(self.previous_frame_end.take().unwrap().join(acquire_future).boxed())
    }

    /// This function submits the command buffer to the queue and fences the operation,
    /// storing a future refering to the operation.
    ///
    /// This function must be run once at the end of all updates and draw calls in order for the frame to be sumbitted.
    pub fn end_frame(&mut self, after_future: Box<dyn GpuFuture>) {
        let future = after_future
            .then_swapchain_present(
                self.queue.clone(),
                self.swapchain.clone(),
                self.image_num,
            )
            .then_signal_fence_and_flush();

        match future {
            Ok(future) => {
                self.previous_frame_end = Some(future.boxed());
            }
            Err(FlushError::OutOfDate) => {
                self.recreate_swapchain = true;
                self.previous_frame_end = Some(sync::now(self.device.clone()).boxed());
            }
            Err(e) => {
                println!("Failed to flush future: {:?}", e);
                self.previous_frame_end = Some(sync::now(self.device.clone()).boxed());
            }
        };
    }

    pub fn final_image(&self) -> FinalImageView {
        self.image_views[self.image_num].clone()
    }

    pub fn output_format(&self) -> Format {
        self.image_views[self.image_num].format().unwrap()
    }
}
