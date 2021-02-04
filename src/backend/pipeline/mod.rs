pub mod sprite;
pub mod texture;

use crate::backend::shader::ShaderSet;
use crate::backend::vertex::Vertex;

pub trait Pipeline {
    /// (Re-)initialize the pipeline. Usually you do it after changes in resources.
    fn initialize(&mut self, device: &wgpu::Device, bind_group_layouts: &[&wgpu::BindGroupLayout]);
    fn resize(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, width: u32, height: u32);
    fn layout(&self) -> &Option<wgpu::PipelineLayout>;
    fn pipeline(&self) -> &Option<wgpu::RenderPipeline>;
    fn vertex_buffer(&self) -> &Option<wgpu::Buffer>;
    fn index_buffer(&self) -> &Option<wgpu::Buffer>;
    fn index_number(&self) -> u32;
}
