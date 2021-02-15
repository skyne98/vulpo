use crate::backend::resource::Resource;
use bytemuck::__core::marker::PhantomData;
use std::fmt::Debug;
use wgpu::util::DeviceExt;
use wgpu::{BindGroupEntry, BindGroupLayoutEntry};

pub struct Uniform<T: Debug + Copy + Clone + bytemuck::Pod + bytemuck::Zeroable> {
    wgpu: wgpu::Buffer,
    phantom: PhantomData<T>,
}

impl<T: Debug + Copy + Clone + bytemuck::Pod + bytemuck::Zeroable> Uniform<T> {
    pub fn new(device: &wgpu::Device, data: T) -> Self {
        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Uniform Buffer"),
            contents: bytemuck::cast_slice(&[data]),
            usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
        });

        Self {
            wgpu: uniform_buffer,
            phantom: PhantomData,
        }
    }

    pub fn set(&self, queue: &wgpu::Queue, data: T) {
        queue.write_buffer(&self.wgpu, 0, bytemuck::cast_slice(&[data]));
    }
}

impl<'a, T: Debug + Copy + Clone + bytemuck::Pod + bytemuck::Zeroable> Resource<'a> for Uniform<T> {
    fn entry(
        &self,
        index: u32,
        visibility: wgpu::ShaderStage,
    ) -> (BindGroupLayoutEntry, BindGroupEntry) {
        (
            wgpu::BindGroupLayoutEntry {
                binding: index,
                visibility,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            wgpu::BindGroupEntry {
                binding: index,
                resource: wgpu::BindingResource::Buffer {
                    buffer: &self.wgpu,
                    offset: 0,
                    size: None,
                },
            },
        )
    }
}
