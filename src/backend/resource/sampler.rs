use crate::backend::resource::Resource;
use anyhow::*;
use image::GenericImageView;
use wgpu::{BindGroupEntry, BindGroupLayout, BindGroupLayoutEntry};

pub struct Sampler {
    pub wgpu: wgpu::Sampler,
}

impl Sampler {
    pub fn new(device: &wgpu::Device, descriptor: &wgpu::SamplerDescriptor) -> Self {
        Self {
            wgpu: device.create_sampler(descriptor),
        }
    }

    pub fn pixel(device: &wgpu::Device) -> Self {
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        Self { wgpu: sampler }
    }
}

impl<'a> Resource<'a> for Sampler {
    fn entry(
        &self,
        index: u32,
        visibility: wgpu::ShaderStage,
    ) -> (BindGroupLayoutEntry, BindGroupEntry) {
        (
            wgpu::BindGroupLayoutEntry {
                binding: index,
                visibility,
                ty: wgpu::BindingType::Sampler {
                    comparison: false,
                    filtering: false,
                },
                count: None,
            },
            wgpu::BindGroupEntry {
                binding: index,
                resource: wgpu::BindingResource::Sampler(&self.wgpu),
            },
        )
    }
}
