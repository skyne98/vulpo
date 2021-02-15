use crate::backend::resource::Resource;
use anyhow::*;
use image::GenericImageView;
use wgpu::{BindGroupEntry, BindGroupLayout, BindGroupLayoutEntry};

pub enum TextureSource {
    Texture {
        texture: wgpu::Texture,
        view: wgpu::TextureView,
    },
    SwapChainTexture {
        texture: wgpu::SwapChainTexture,
    },
}

pub struct Texture {
    pub source: TextureSource,
}

impl Texture {
    pub fn from_swap_chain_texture(texture: wgpu::SwapChainTexture) -> Self {
        Self {
            source: TextureSource::SwapChainTexture { texture },
        }
    }

    pub fn from_bytes(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        bytes: &[u8],
        label: &str,
    ) -> Result<Self> {
        let img = image::load_from_memory(bytes)?;
        Self::from_image(device, queue, &img, Some(label))
    }

    pub fn from_image(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        img: &image::DynamicImage,
        label: Option<&str>,
    ) -> Result<Self> {
        let rgba = img.as_rgba8().unwrap();
        let dimensions = img.dimensions();

        let size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth: 1,
        };
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label,
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::COPY_DST,
        });

        queue.write_texture(
            wgpu::TextureCopyView {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            rgba,
            wgpu::TextureDataLayout {
                offset: 0,
                bytes_per_row: 4 * dimensions.0,
                rows_per_image: dimensions.1,
            },
            size,
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        Ok(Self {
            source: TextureSource::Texture { texture, view },
        })
    }

    pub fn get_view(&self) -> &wgpu::TextureView {
        let view = match &self.source {
            TextureSource::Texture { texture, ref view } => view,
            TextureSource::SwapChainTexture { ref texture } => &texture.view,
        };

        view
    }
}

impl<'a> Resource<'a> for Texture {
    fn entry(
        &self,
        index: u32,
        visibility: wgpu::ShaderStage,
    ) -> (BindGroupLayoutEntry, BindGroupEntry) {
        let view = match &self.source {
            TextureSource::Texture { texture, ref view } => view,
            TextureSource::SwapChainTexture { ref texture } => &texture.view,
        };

        (
            wgpu::BindGroupLayoutEntry {
                binding: index,
                visibility,
                ty: wgpu::BindingType::Texture {
                    multisampled: false,
                    sample_type: wgpu::TextureSampleType::Float { filterable: false },
                    view_dimension: wgpu::TextureViewDimension::D2,
                },
                count: None,
            },
            wgpu::BindGroupEntry {
                binding: index,
                resource: wgpu::BindingResource::TextureView(view),
            },
        )
    }
}
