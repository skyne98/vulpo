pub mod sampler;
pub mod texture;
pub mod uniform;

pub trait Resource<'a> {
    fn entry(
        &self,
        index: u32,
        visibility: wgpu::ShaderStage,
    ) -> (wgpu::BindGroupLayoutEntry, wgpu::BindGroupEntry);
}

pub fn build_bind_group<'a>(
    device: &wgpu::Device,
    visibility: wgpu::ShaderStage,
    resources: Vec<&dyn Resource<'a>>,
) -> (wgpu::BindGroupLayout, wgpu::BindGroup) {
    let (group_layout_entries, group_entries): (Vec<_>, Vec<_>) = resources
        .iter()
        .enumerate()
        .map(|(index, res)| res.entry(index as u32, visibility))
        .unzip();

    let group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("bind_group_layout"),
        entries: &group_layout_entries,
    });
    let group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("bind_group"),
        layout: &group_layout,
        entries: &group_entries,
    });

    (group_layout, group)
}
