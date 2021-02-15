use crate::backend::pipeline::Pipeline;
use crate::backend::resource::build_bind_group;
use crate::backend::resource::uniform::Uniform;
use crate::backend::shader::ShaderSet;
use crate::backend::sprite::Sprites;
use crate::backend::vertex::Vertex;
use wgpu::util::DeviceExt;
use wgpu::{BindGroup, PipelineLayout, RenderPipeline};

// TEXTURE RANGE: (0.0) - (1.0)
// FRAME RANGE: (-1.0) - (1.0)

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Global {
    pub ortho: ultraviolet::Mat4,
    pub transform: ultraviolet::Mat4,
}

pub struct SpritePipeline {
    shaders: ShaderSet,
    texture_format: wgpu::TextureFormat,
    layout: Option<wgpu::PipelineLayout>,
    pipeline: Option<wgpu::RenderPipeline>,
    vertex_buffer: Option<wgpu::Buffer>,
    index_buffer: Option<wgpu::Buffer>,
    global_buffer: Option<Uniform<Global>>,
    bind_groups: Option<Vec<wgpu::BindGroup>>,
    sprites: Sprites,
    texture_width: u32,
    texture_height: u32,
}

impl SpritePipeline {
    pub fn new(
        device: &wgpu::Device,
        texture_format: wgpu::TextureFormat,
        texture_width: u32,
        texture_height: u32,
    ) -> Self {
        // Shaders
        let vs_module =
            device.create_shader_module(&wgpu::include_spirv!("../../shaders/sprite.vert.spv"));
        let fs_module =
            device.create_shader_module(&wgpu::include_spirv!("../../shaders/sprite.frag.spv"));
        let shader_set = ShaderSet {
            vertex: vs_module,
            fragment: fs_module,
        };

        let mut sprites = Sprites::new();
        sprites.add(
            0,
            ultraviolet::Vec2::new(0.0, 0.0),
            ultraviolet::Vec2::new(90.0, 90.0),
            ultraviolet::Vec2::new(180.0, 180.0),
            0.0,
            ultraviolet::Vec2::new(1.0, 1.0),
            1.0,
            [1.0, 1.0, 1.0, 1.0],
            ultraviolet::Vec2::new(0.0, 0.0),
        );
        sprites.add(
            0,
            ultraviolet::Vec2::new(0.0, 0.0),
            ultraviolet::Vec2::new(90.0, 90.0),
            ultraviolet::Vec2::new(90.0, 90.0),
            0.0,
            ultraviolet::Vec2::new(1.0, 1.0),
            1.0,
            [1.0, 1.0, 1.0, 1.0],
            ultraviolet::Vec2::new(0.0, 0.0),
        );
        sprites.add(
            0,
            ultraviolet::Vec2::new(0.0, 0.0),
            ultraviolet::Vec2::new(90.0, 90.0),
            ultraviolet::Vec2::new(0.0, 0.0),
            0.0,
            ultraviolet::Vec2::new(1.0, 1.0),
            1.0,
            [1.0, 1.0, 1.0, 1.0],
            ultraviolet::Vec2::new(0.0, 0.0),
        );

        SpritePipeline {
            shaders: shader_set,
            texture_format,
            layout: None,
            pipeline: None,
            sprites,
            vertex_buffer: None,
            index_buffer: None,
            global_buffer: None,
            bind_groups: None,
            texture_width,
            texture_height,
        }
    }

    pub fn get_sprites_mut(&mut self) -> &mut Sprites {
        &mut self.sprites
    }

    pub fn vertices_indices(&self) -> (Vec<Vertex>, Vec<u16>) {
        self.sprites
            .vertices_indices(self.texture_width, self.texture_height)
    }
}

impl Pipeline for SpritePipeline {
    fn initialize<
        F0: Fn(&wgpu::Device, &wgpu::Queue) -> Vec<(wgpu::BindGroupLayout, wgpu::BindGroup)>,
    >(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        bind_group_builder: F0,
    ) {
        // Build vertex and index lists
        let (vertices, indices) = self.vertices_indices();

        // Vertex buffer
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsage::VERTEX | wgpu::BufferUsage::COPY_DST,
        });
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&indices),
            usage: wgpu::BufferUsage::INDEX | wgpu::BufferUsage::COPY_DST,
        });

        // Global buffer
        let global_buffer = Uniform::new(
            &device,
            Global {
                ortho: ultraviolet::Mat4::identity(),
                transform: ultraviolet::Mat4::identity(),
            },
        );

        // Bind groups
        let (global_bind_group_layout, global_bind_group) =
            build_bind_group(&device, wgpu::ShaderStage::VERTEX, vec![&global_buffer]);
        let (texture_bind_group_layouts, texture_bind_groups): (Vec<_>, Vec<_>) =
            bind_group_builder(&device, &queue).into_iter().unzip();
        let bind_group_layouts = vec![global_bind_group_layout]
            .into_iter()
            .chain(texture_bind_group_layouts)
            .collect::<Vec<_>>();
        let bind_groups = vec![global_bind_group]
            .into_iter()
            .chain(texture_bind_groups)
            .collect::<Vec<_>>();

        // Pipeline
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Sprite Pipeline Layout"),
                bind_group_layouts: &bind_group_layouts.iter().collect::<Vec<_>>(),
                push_constant_ranges: &[],
            });
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Sprite Pipeline"),
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
        self.global_buffer = Some(global_buffer);
        self.bind_groups = Some(bind_groups);
    }

    fn resize(&mut self, _device: &wgpu::Device, queue: &wgpu::Queue, width: u32, height: u32) {
        if let Some(_) = self.vertex_buffer {
            // Update the vertex buffers
            let (vertices, indices) = self.vertices_indices();

            // Vertex buffer
            queue.write_buffer(
                self.vertex_buffer.as_mut().unwrap(),
                0,
                bytemuck::cast_slice(&vertices),
            );
            queue.write_buffer(
                self.index_buffer.as_mut().unwrap(),
                0,
                bytemuck::cast_slice(&indices),
            );

            // Global uniform buffer
            self.global_buffer.as_ref().unwrap().set(
                &queue,
                Global {
                    ortho: ultraviolet::projection::rh_yup::orthographic_wgpu_dx(
                        0.0,
                        width as f32,
                        0.0,
                        height as f32,
                        -100.0,
                        100.0,
                    ),
                    transform: ultraviolet::Mat4::identity(),
                },
            );
        }
    }
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
        self.sprites.len() as u32 * 6
    }
    fn groups(&self) -> &Option<Vec<BindGroup>> {
        &self.bind_groups
    }
}
