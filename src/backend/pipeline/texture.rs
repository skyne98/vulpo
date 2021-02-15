use crate::backend::pipeline::Pipeline;
use crate::backend::shader::ShaderSet;
use crate::backend::vertex::Vertex;
use wgpu::util::DeviceExt;
use wgpu::{BindGroup, Buffer, PipelineLayout, RenderPipeline};

// TEXTURE RANGE: (0.0) - (1.0)
// FRAME RANGE: (-1.0) - (1.0)

const VERTICES: &[Vertex] = &[
    Vertex {
        position: [-1.0, 1.0, 1.0],
        tex_coords: [0.0, 0.0],
    }, // A
    Vertex {
        position: [-1.0, -1.0, 1.0],
        tex_coords: [0.0, 1.0],
    }, // B
    Vertex {
        position: [1.0, -1.0, 1.0],
        tex_coords: [1.0, 1.0],
    }, // C
    Vertex {
        position: [1.0, 1.0, 1.0],
        tex_coords: [1.0, 0.0],
    }, // D
];

const INDICES: &[u16] = &[0, 1, 3, 3, 1, 2];

pub struct TexturePipeline {
    shaders: ShaderSet,
    texture_format: wgpu::TextureFormat,
    layout: Option<wgpu::PipelineLayout>,
    pipeline: Option<wgpu::RenderPipeline>,
    vertex_buffer: Option<wgpu::Buffer>,
    index_buffer: Option<wgpu::Buffer>,
    bind_groups: Option<Vec<wgpu::BindGroup>>,
}

impl TexturePipeline {
    pub fn new(device: &wgpu::Device, texture_format: wgpu::TextureFormat) -> Self {
        // Shaders
        let vs_module =
            device.create_shader_module(&wgpu::include_spirv!("../../shaders/texture.vert.spv"));
        let fs_module =
            device.create_shader_module(&wgpu::include_spirv!("../../shaders/texture.frag.spv"));
        let shader_set = ShaderSet {
            vertex: vs_module,
            fragment: fs_module,
        };

        TexturePipeline {
            shaders: shader_set,
            texture_format,
            layout: None,
            pipeline: None,
            vertex_buffer: None,
            index_buffer: None,
            bind_groups: None,
        }
    }
}

impl Pipeline for TexturePipeline {
    fn initialize<
        F0: Fn(&wgpu::Device, &wgpu::Queue) -> Vec<(wgpu::BindGroupLayout, wgpu::BindGroup)>,
    >(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        bind_group_builder: F0,
    ) {
        // Vertex buffer
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: wgpu::BufferUsage::VERTEX,
        });
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(INDICES),
            usage: wgpu::BufferUsage::INDEX,
        });

        // Bind groups
        let (bind_group_layouts, bind_groups): (Vec<_>, Vec<_>) =
            bind_group_builder(&device, &queue).into_iter().unzip();

        // Pipeline
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Texture Pipeline Layout"),
                bind_group_layouts: &bind_group_layouts.iter().collect::<Vec<_>>(),
                push_constant_ranges: &[],
            });
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Texture Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &self.shaders.vertex,
                entry_point: "main",
                buffers: &[Vertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &self.shaders.fragment,
                entry_point: "main",
                targets: &[wgpu::ColorTargetState {
                    format: self.texture_format,
                    color_blend: wgpu::BlendState::REPLACE,
                    alpha_blend: wgpu::BlendState::REPLACE,
                    write_mask: wgpu::ColorWrite::ALL,
                }],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: wgpu::CullMode::Back,
                polygon_mode: wgpu::PolygonMode::Fill,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
        });

        self.layout = Some(render_pipeline_layout);
        self.pipeline = Some(render_pipeline);
        self.vertex_buffer = Some(vertex_buffer);
        self.index_buffer = Some(index_buffer);
        self.bind_groups = Some(bind_groups);
    }

    fn resize(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, width: u32, height: u32) {}
    fn layout(&self) -> &Option<PipelineLayout> {
        &self.layout
    }
    fn pipeline(&self) -> &Option<RenderPipeline> {
        &self.pipeline
    }
    fn vertex_buffer(&self) -> &Option<wgpu::Buffer> {
        &self.vertex_buffer
    }
    fn index_buffer(&self) -> &Option<wgpu::Buffer> {
        &self.index_buffer
    }
    fn index_number(&self) -> u32 {
        INDICES.len() as u32
    }
    fn groups(&self) -> &Option<Vec<BindGroup>> {
        &self.bind_groups
    }
}
