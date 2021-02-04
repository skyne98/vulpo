pub mod texture;

use crate::backend::shader::CoreShaderSet;
use crate::backend::vertex::Vertex;

pub trait Pipeline<'a> {
    /// (Re-)initialize the pipeline. Usually you do it after changes in resources.
    fn initialize(&mut self, device: &wgpu::Device, bind_group_layout: &wgpu::BindGroupLayout);
    fn layout(&self) -> &Option<wgpu::PipelineLayout>;
    fn pipeline(&self) -> &Option<wgpu::RenderPipeline>;
}
