pub mod sprite;
pub mod texture;




pub trait Pipeline {
    /// (Re-)initialize the pipeline. Usually you do it after changes in resources.
    fn initialize<
        F0: Fn(&wgpu::Device, &wgpu::Queue) -> Vec<(wgpu::BindGroupLayout, wgpu::BindGroup)>,
    >(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        bind_group_builder: F0,
    );
    fn resize(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, width: u32, height: u32);
    fn layout(&self) -> &Option<wgpu::PipelineLayout>;
    fn pipeline(&self) -> &Option<wgpu::RenderPipeline>;
    fn vertex_buffer(&self) -> &Option<wgpu::Buffer>;
    fn index_buffer(&self) -> &Option<wgpu::Buffer>;
    fn index_number(&self) -> u32;
    fn groups(&self) -> &Option<Vec<wgpu::BindGroup>>;
}
