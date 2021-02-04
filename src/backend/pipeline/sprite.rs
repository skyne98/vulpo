use crate::backend::pipeline::Pipeline;
use crate::backend::shader::ShaderSet;
use crate::backend::sprite::Sprites;
use crate::backend::vertex::Vertex;
use futures::StreamExt;
use rayon::prelude::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};
use wgpu::util::DeviceExt;
use wgpu::{Buffer, PipelineLayout, RenderPipeline};

// TEXTURE RANGE: (0.0) - (1.0)
// FRAME RANGE: (-1.0) - (1.0)

const BASELINE_INDICES: &[u16] = &[0, 1, 3, 3, 1, 2];

pub struct SpritePipeline {
    shaders: ShaderSet,
    texture_format: wgpu::TextureFormat,
    layout: Option<wgpu::PipelineLayout>,
    pipeline: Option<wgpu::RenderPipeline>,
    vertex_buffer: Option<wgpu::Buffer>,
    index_buffer: Option<wgpu::Buffer>,
    sprites: Sprites,
    surface_width: u32,
    surface_height: u32,
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
            device.create_shader_module(wgpu::include_spirv!("../../shaders/sprite.vert.spv"));
        let fs_module =
            device.create_shader_module(wgpu::include_spirv!("../../shaders/sprite.frag.spv"));
        let shader_set = ShaderSet {
            vertex: vs_module,
            fragment: fs_module,
        };

        let mut sprites = Sprites::new();
        sprites.add(
            0,
            ultraviolet::Vec2::new(0.0, 0.0),
            ultraviolet::Vec2::new(90.0, 90.0),
            ultraviolet::Vec2::new(0.0, 0.0),
            45.0,
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
            sprites: sprites,
            vertex_buffer: None,
            index_buffer: None,
            surface_width: 0,
            surface_height: 0,
            texture_width,
            texture_height,
        }
    }

    pub fn get_sprites_mut(&mut self) -> &mut Sprites {
        &mut self.sprites
    }

    pub fn vertices_indices(&self) -> (Vec<Vertex>, Vec<u16>) {
        let (vertices, indices): (Vec<_>, Vec<_>) = (0..self.sprites.len())
            .map(|index| (index, self.sprites.get(index).unwrap()))
            .collect::<Vec<_>>()
            .par_iter()
            .map(|sprite| {
                let (
                    index,
                    (
                        texture_index,
                        source_position,
                        source_size,
                        position,
                        angle,
                        scale,
                        depth,
                        color,
                        origin,
                    ),
                ) = sprite;

                // Relative texture coordinates
                let src_relative_min_x: f32 = source_position.x / self.texture_width as f32;
                let src_relative_min_y: f32 = source_position.y / self.texture_height as f32;
                let src_relative_max_x: f32 =
                    source_position.x + source_size.x / self.texture_width as f32;
                let src_relative_max_y: f32 =
                    source_position.y + source_size.y / self.texture_height as f32;

                // Transform matrix
                let scale_mat =
                    ultraviolet::Mat4::from_nonuniform_scale(ultraviolet::Vec3::new(1.0, 1.0, 1.0));
                let origin_translation =
                    ultraviolet::Mat4::from_translation(ultraviolet::Vec3::new(
                        -source_size.x * origin.x * scale.x,
                        -source_size.y * origin.y * scale.y,
                        0.0,
                    ));
                let rotation =
                    ultraviolet::Mat4::from_rotation_z(*angle * std::f32::consts::PI / 180.0);
                let translation = ultraviolet::Mat4::from_translation(ultraviolet::Vec3::new(
                    (*position).x,
                    (*position).y,
                    0.0,
                ));
                let transformation = translation * rotation * origin_translation * scale_mat;

                // Calculate the position vectors
                let vec_a = ultraviolet::Vec3::new(0.0, 1.0, 1.0);
                let vec_a = transformation.transform_vec3(vec_a);
                let vec_b = ultraviolet::Vec3::new(0.0, 0.0, 1.0);
                let vec_b = transformation.transform_vec3(vec_b);
                let vec_c = ultraviolet::Vec3::new(1.0, 0.0, 1.0);
                let vec_c = transformation.transform_vec3(vec_c);
                let vec_d = ultraviolet::Vec3::new(1.0, 1.0, 1.0);
                let vec_d = transformation.transform_vec3(vec_d);

                // Create the UV arrays
                let uv_a = ultraviolet::Vec2::new(0.0, 0.0);
                let uv_b = ultraviolet::Vec2::new(0.0, 1.0);
                let uv_c = ultraviolet::Vec2::new(1.0, 1.0);
                let uv_d = ultraviolet::Vec2::new(1.0, 0.0);

                // Calculate the indices
                let indices = BASELINE_INDICES
                    .iter()
                    .map(|i| *i * (*index as u16 + 1))
                    .collect::<Vec<_>>();

                // Generate the vertices
                let vertices = vec![
                    Vertex {
                        position: [vec_a.x, vec_a.y, vec_a.z],
                        tex_coords: [uv_a.x, uv_a.y],
                    }, // A
                    Vertex {
                        position: [vec_b.x, vec_b.y, vec_b.z],
                        tex_coords: [uv_b.x, uv_b.y],
                    }, // B
                    Vertex {
                        position: [vec_c.x, vec_c.y, vec_c.z],
                        tex_coords: [uv_c.x, uv_c.y],
                    }, // C
                    Vertex {
                        position: [vec_d.x, vec_d.y, vec_d.z],
                        tex_coords: [uv_d.x, uv_d.y],
                    }, // D
                ];
                (vertices, indices)
            })
            .unzip();

        (
            vertices.into_iter().flatten().collect(),
            indices.into_iter().flatten().collect(),
        )
    }
}

impl Pipeline for SpritePipeline {
    fn initialize(&mut self, device: &wgpu::Device, bind_group_layouts: &[&wgpu::BindGroupLayout]) {
        // Pipeline
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Texture Pipeline Layout"),
                bind_group_layouts,
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

        self.layout = Some(render_pipeline_layout);
        self.pipeline = Some(render_pipeline);
        self.vertex_buffer = Some(vertex_buffer);
        self.index_buffer = Some(index_buffer);
    }

    fn resize(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, width: u32, height: u32) {
        self.surface_width = width;
        self.surface_height = height;

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
}
