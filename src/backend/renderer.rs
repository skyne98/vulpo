use crate::backend::pipeline::texture::TexturePipeline;
use crate::backend::pipeline::Pipeline;
use crate::backend::resource::sampler::Sampler;
use crate::backend::resource::{build_bind_group, texture};
use crate::backend::swapchain::SwapChain;
use crate::backend::vertex::Vertex;
use anyhow::Result;
use bytemuck::__core::marker::PhantomData;
use wgpu::util::DeviceExt;
use winit::event::WindowEvent;
use winit::window::Window;

pub struct Renderer<P: Pipeline> {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    swap_chain: SwapChain,
    pub size: winit::dpi::PhysicalSize<u32>,
    pipeline: P,
    bind_groups: Vec<wgpu::BindGroup>,
}

impl<P: Pipeline> Renderer<P> {
    // Creating some of the wgpu types requires async code
    pub async fn new<
        F0: Fn(&wgpu::Device, &wgpu::Queue) -> Vec<(wgpu::BindGroupLayout, wgpu::BindGroup)>,
        F1: Fn(&wgpu::Device, wgpu::TextureFormat) -> P,
    >(
        window: &Window,
        bind_group_builder: F0,
        pipeline_builder: F1,
    ) -> Self {
        let size = window.inner_size();

        // Instance is our GPU handle
        // BackendBit::PRIMARY => Vulkan + Metal + DX12 + Browser WebGPU
        let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::Default,
                compatible_surface: Some(&surface),
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                    shader_validation: true,
                },
                None,
            )
            .await
            .unwrap();

        let swap_descriptor = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Mailbox,
        };
        let swap_chain = SwapChain::new(&device, &surface, swap_descriptor);

        // Bind groups
        let (bind_group_layouts, bind_groups): (Vec<_>, Vec<_>) =
            bind_group_builder(&device, &queue).into_iter().unzip();

        // Pipeline
        let mut pipeline = pipeline_builder(&device, swap_chain.descriptor.format);
        pipeline.resize(&device, &queue, size.width, size.height);
        pipeline.initialize(&device, &bind_group_layouts.iter().collect::<Vec<_>>());

        Self {
            surface,
            device,
            queue,
            swap_chain,
            size,
            pipeline,
            bind_groups,
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;
        self.swap_chain
            .resize(&self.device, &self.surface, new_size.width, new_size.height);
        self.pipeline
            .resize(&self.device, &self.queue, new_size.width, new_size.height);
    }

    pub fn input(&mut self, event: &WindowEvent) -> bool {
        false
    }

    pub fn update(&mut self) {}

    pub fn render(&mut self) -> Result<(), wgpu::SwapChainError> {
        let frame = self.swap_chain.get_current_frame()?;
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                    attachment: frame.get_view(),
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.0,
                            g: 0.0,
                            b: 0.0,
                            a: 1.0,
                        }),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });

            let vertex_buffer = self.pipeline.vertex_buffer().as_ref().unwrap();
            let index_buffer = self.pipeline.index_buffer().as_ref().unwrap();
            let number_of_indecies = self.pipeline.index_number();

            render_pass.set_pipeline(&self.pipeline.pipeline().as_ref().unwrap());
            for (i, bind_group) in self.bind_groups.iter().enumerate() {
                render_pass.set_bind_group(i as u32, bind_group, &[]);
            }
            render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
            render_pass.set_index_buffer(index_buffer.slice(..));
            render_pass.draw_indexed(0..number_of_indecies, 0, 0..1);
        }

        // submit will accept anything that implements IntoIter
        self.queue.submit(std::iter::once(encoder.finish()));

        Ok(())
    }
}
