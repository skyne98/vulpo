use crate::backend::pipeline::Pipeline;
use crate::backend::renderer::Renderer;
use futures::executor::block_on;
use winit::event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

pub struct Window {}

impl Window {
    pub fn new<
        P: Pipeline + 'static,
        F0: Fn(&wgpu::Device, &wgpu::Queue) -> Vec<(wgpu::BindGroupLayout, wgpu::BindGroup)>,
        F1: Fn(&wgpu::Device, wgpu::TextureFormat) -> P,
    >(
        bind_group_builder: F0,
        pipeline_builder: F1,
    ) -> Self {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new().build(&event_loop).unwrap();

        let mut renderer = block_on(Renderer::new::<F0, F1>(
            &window,
            bind_group_builder,
            pipeline_builder,
        ));

        event_loop.run(move |event, _, control_flow| {
            match event {
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == window.id() => {
                    if !renderer.input(event) {
                        // UPDATED!
                        match event {
                            WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                            WindowEvent::KeyboardInput { input, .. } => match input {
                                KeyboardInput {
                                    state: ElementState::Pressed,
                                    virtual_keycode: Some(VirtualKeyCode::Escape),
                                    ..
                                } => *control_flow = ControlFlow::Exit,
                                _ => {}
                            },
                            WindowEvent::Resized(physical_size) => {
                                renderer.resize(physical_size.width, physical_size.height);
                            }
                            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                                renderer.resize(new_inner_size.width, new_inner_size.height);
                            }
                            _ => {}
                        }
                    }
                }
                Event::RedrawRequested(_) => {
                    renderer.update();
                    match renderer.render() {
                        Ok(_) => {}
                        // Recreate the swap_chain if lost
                        Err(wgpu::SwapChainError::Lost) => {
                            renderer.resize(renderer.width, renderer.height)
                        }
                        // The system is out of memory, we should probably quit
                        Err(wgpu::SwapChainError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                        // All other errors (Outdated, Timeout) should be resolved by the next frame
                        Err(e) => eprintln!("{:?}", e),
                    }
                }
                Event::MainEventsCleared => {
                    // RedrawRequested will only trigger once, unless we manually
                    // request it.
                    window.request_redraw();
                }
                _ => {}
            }
        });

        Self {}
    }
}
