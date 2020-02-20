use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn main() {
    let (event_loop, window, mut size) = {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_title("LearnOpenGL but it's WGPU-rs")
            .build(&event_loop)
            .unwrap();
        let size = window.inner_size();
        (event_loop, window, size)
    };

    let surface = wgpu::Surface::create(&window);

    let (device, mut queue) = wgpu::Adapter::request(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::HighPerformance,
        backends: wgpu::BackendBit::all(),
    })
    .unwrap()
    .request_device(&wgpu::DeviceDescriptor {
        extensions: wgpu::Extensions {
            anisotropic_filtering: false,
        },
        ..Default::default()
    });

    let mut swap_chain_descriptor = wgpu::SwapChainDescriptor {
        usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
        format: wgpu::TextureFormat::Bgra8Unorm,
        width: size.width,
        height: size.height,
        present_mode: wgpu::PresentMode::Vsync,
    };

    let mut swap_chain = device.create_swap_chain(&surface, &swap_chain_descriptor);

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } => *control_flow = ControlFlow::Exit,
        Event::WindowEvent {
            event: WindowEvent::Resized(new_size),
            ..
        } => {
            size = new_size;
            swap_chain_descriptor.width = size.width;
            swap_chain_descriptor.height = size.height;
            swap_chain = device.create_swap_chain(&surface, &swap_chain_descriptor);
            *control_flow = ControlFlow::Poll;
        }
        Event::MainEventsCleared => {
            window.request_redraw();
            *control_flow = ControlFlow::Poll;
        }
        Event::RedrawRequested(_) => {
            let frame = swap_chain.get_next_texture();
            let mut encoder =
                device.create_command_encoder(&wgpu::CommandEncoderDescriptor { todo: 0 });
            {
                let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                        attachment: &frame.view,
                        resolve_target: None,
                        load_op: wgpu::LoadOp::Clear,
                        store_op: wgpu::StoreOp::Store,
                        clear_color: wgpu::Color {
                            r: 0.2,
                            g: 0.3,
                            b: 0.3,
                            a: 1.0,
                        },
                    }],
                    depth_stencil_attachment: None,
                });
            }

            queue.submit(&[encoder.finish()]);
        }
        _ => {}
    })
}
