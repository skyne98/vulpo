use crate::backend::pipeline::Pipeline;
use crate::backend::shader::CoreShaderSet;
use crate::backend::vertex::Vertex;
use wgpu::{PipelineLayout, RenderPipeline};

pub struct TexturePipeline {
    shaders: CoreShaderSet,
    texture_format: wgpu::TextureFormat,
    layout: Option<wgpu::PipelineLayout>,
    pipeline: Option<wgpu::RenderPipeline>,
}

impl TexturePipeline {
    pub fn new(device: &wgpu::Device, texture_format: wgpu::TextureFormat) -> Self {
        // Shaders
        let vs_module =
            device.create_shader_module(wgpu::include_spirv!("../../shaders/shader.vert.spv"));
        let fs_module =
            device.create_shader_module(wgpu::include_spirv!("../../shaders/shader.frag.spv"));
        let shader_set = CoreShaderSet {
            vertex: vs_module,
            fragment: fs_module,
        };

        TexturePipeline {
            shaders: shader_set,
            texture_format,
            layout: None,
            pipeline: None,
        }
    }
}

impl<'a> Pipeline<'a> for TexturePipeline {
    fn initialize(&mut self, device: &wgpu::Device, bind_group_layout: &wgpu::BindGroupLayout) {
        // Pipeline
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Texture Pipeline Layout"),
                bind_group_layouts: &[&bind_group_layout],
                push_constant_ranges: &[],
            });
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Texture Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex_stage: wgpu::ProgrammableStageDescriptor {
                module: &self.shaders.vertex,
                entry_point: "main",
            },
            fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
                module: &self.shaders.fragment,
                entry_point: "main",
            }),
            rasterization_state: Some(wgpu::RasterizationStateDescriptor {
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: wgpu::CullMode::Back,
                depth_bias: 0,
                depth_bias_slope_scale: 0.0,
                depth_bias_clamp: 0.0,
                clamp_depth: false,
            }),
            color_states: &[wgpu::ColorStateDescriptor {
                format: self.texture_format,
                color_blend: wgpu::BlendDescriptor::REPLACE,
                alpha_blend: wgpu::BlendDescriptor::REPLACE,
                write_mask: wgpu::ColorWrite::ALL,
            }],
            primitive_topology: wgpu::PrimitiveTopology::TriangleList,
            depth_stencil_state: None,
            vertex_state: wgpu::VertexStateDescriptor {
                index_format: wgpu::IndexFormat::Uint16,
                vertex_buffers: &[Vertex::desc()],
            },
            sample_count: 1,
            sample_mask: !0,
            alpha_to_coverage_enabled: false,
        });

        self.layout = Some(render_pipeline_layout);
        self.pipeline = Some(render_pipeline);
    }

    fn layout(&self) -> &Option<PipelineLayout> {
        &self.layout
    }

    fn pipeline(&self) -> &Option<RenderPipeline> {
        &self.pipeline
    }
}
