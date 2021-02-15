use crate::backend::resource::texture::Texture;
use anyhow::Result;

pub struct SwapChain {
    pub wgpu: wgpu::SwapChain,
    pub descriptor: wgpu::SwapChainDescriptor,
}

impl SwapChain {
    pub fn new(
        device: &wgpu::Device,
        surface: &wgpu::Surface,
        descriptor: wgpu::SwapChainDescriptor,
    ) -> Self {
        Self {
            wgpu: device.create_swap_chain(surface, &descriptor),
            descriptor: descriptor,
        }
    }
    pub fn resize(
        &mut self,
        device: &wgpu::Device,
        surface: &wgpu::Surface,
        width: u32,
        height: u32,
    ) {
        self.descriptor.width = width;
        self.descriptor.height = height;
        self.wgpu = device.create_swap_chain(surface, &self.descriptor);
    }
    pub fn get_current_frame(&mut self) -> Result<Texture, wgpu::SwapChainError> {
        let frame = self.wgpu.get_current_frame()?.output;
        Ok(Texture::from_swap_chain_texture(frame))
    }
}
