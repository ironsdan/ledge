use vulkano::buffer::BufferAccess;
use vulkano::buffer::{BufferUsage, CpuAccessibleBuffer};
use vulkano::device::Device;

use std::sync::Arc;

pub struct BufferAttribute<T> {
    pub inner: std::sync::Arc<vulkano::buffer::CpuAccessibleBuffer<T>>,
}

impl<T : 'static + Copy> BufferAttribute<T> {
    pub fn from_data(data: T, device: Arc<Device>) -> Self {
        let cpu_buffer = CpuAccessibleBuffer::from_data(
            device.clone(), 
            BufferUsage::all(), 
            false,
            data,
        ).unwrap();
        
        Self {
            inner: cpu_buffer
        }
    }
}