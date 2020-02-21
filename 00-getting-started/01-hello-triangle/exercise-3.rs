use shaderc::{Compiler, ShaderKind};
use ultraviolet::Vec3;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

#[rustfmt::skip]
const VERTICES_1: &[Vertex] = &[
    Vertex { translation: Vec3 { x: -0.75, y: -0.5,  z:  0.0  } },
    Vertex { translation: Vec3 { x: -0.75, y:  0.5,  z:  0.0  } },
    Vertex { translation: Vec3 { x:  0.25, y:  0.5,  z:  0.0  } },
    Vertex { translation: Vec3 { x:  0.25, y: -0.5,  z:  0.0  } },
];

#[rustfmt::skip]
const VERTICES_2: &[Vertex] = &[
    Vertex { translation: Vec3 { x: -0.25, y: -0.5,  z:  0.0  } },
    Vertex { translation: Vec3 { x: -0.25, y:  0.5,  z:  0.0  } },
    Vertex { translation: Vec3 { x:  0.75, y:  0.5,  z:  0.0  } },
    Vertex { translation: Vec3 { x:  0.75, y: -0.5,  z:  0.0  } },
];

#[rustfmt::skip]
const INDICES_1: &[u16] = &[
    0, 1, 2,
    // 1, 2, 3,
    // 2, 3 , 0
    // 3, 0, 1
];

#[rustfmt::skip]
const INDICES_2: &[u16] = &[
    // 0, 1, 2,
    // 1, 2, 3,
    2, 3 , 0
    // 3, 0, 1
];

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

    let mut shader_compiler = Compiler::new().expect("Could not initialize shader compiler");

    let vs_module = prepare_shader(
        &mut shader_compiler,
        include_str!("shader.vert"),
        ShaderKind::Vertex,
        "shader.vert",
        &device,
    );
    let fs_module_1 = prepare_shader(
        &mut shader_compiler,
        include_str!("shader.frag"),
        ShaderKind::Fragment,
        "shader.frag",
        &device,
    );
    let fs_module_2 = prepare_shader(
        &mut shader_compiler,
        include_str!("shader_2.frag"),
        ShaderKind::Fragment,
        "shader_2.frag",
        &device,
    );

    let vertex_buffer_1 = device
        .create_buffer_mapped(VERTICES_1.len(), wgpu::BufferUsage::VERTEX)
        .fill_from_slice(VERTICES_1);
    let vertex_buffer_2 = device
        .create_buffer_mapped(VERTICES_2.len(), wgpu::BufferUsage::VERTEX)
        .fill_from_slice(VERTICES_2);

    let index_buffer_1 = device
        .create_buffer_mapped(INDICES_1.len(), wgpu::BufferUsage::INDEX)
        .fill_from_slice(INDICES_1);
    let index_buffer_2 = device
        .create_buffer_mapped(INDICES_2.len(), wgpu::BufferUsage::INDEX)
        .fill_from_slice(INDICES_2);

    let mut swap_chain_descriptor = wgpu::SwapChainDescriptor {
        usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
        format: wgpu::TextureFormat::Bgra8Unorm,
        width: size.width,
        height: size.height,
        present_mode: wgpu::PresentMode::Vsync,
    };

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        bind_group_layouts: &[],
    });

    let pipeline_1 = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        layout: &pipeline_layout,
        vertex_stage: wgpu::ProgrammableStageDescriptor {
            module: &vs_module,
            entry_point: "main",
        },
        fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
            module: &fs_module_1,
            entry_point: "main",
        }),
        rasterization_state: Some(wgpu::RasterizationStateDescriptor {
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: wgpu::CullMode::Back,
            depth_bias: 0,
            depth_bias_slope_scale: 0.0,
            depth_bias_clamp: 0.0,
        }),
        primitive_topology: wgpu::PrimitiveTopology::TriangleList,
        color_states: &[wgpu::ColorStateDescriptor {
            format: swap_chain_descriptor.format,
            alpha_blend: wgpu::BlendDescriptor::REPLACE,
            color_blend: wgpu::BlendDescriptor::REPLACE,
            write_mask: wgpu::ColorWrite::ALL,
        }],
        depth_stencil_state: None,
        index_format: wgpu::IndexFormat::Uint16,
        vertex_buffers: &[wgpu::VertexBufferDescriptor {
            stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Vertex,
            attributes: &[wgpu::VertexAttributeDescriptor {
                format: wgpu::VertexFormat::Float3,
                offset: 0,
                shader_location: 0,
            }],
        }],
        sample_count: 1,
        sample_mask: !0,
        alpha_to_coverage_enabled: false,
    });

    let pipeline_2 = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        layout: &pipeline_layout,
        vertex_stage: wgpu::ProgrammableStageDescriptor {
            module: &vs_module,
            entry_point: "main",
        },
        fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
            module: &fs_module_2,
            entry_point: "main",
        }),
        rasterization_state: Some(wgpu::RasterizationStateDescriptor {
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: wgpu::CullMode::Back,
            depth_bias: 0,
            depth_bias_slope_scale: 0.0,
            depth_bias_clamp: 0.0,
        }),
        primitive_topology: wgpu::PrimitiveTopology::TriangleList,
        color_states: &[wgpu::ColorStateDescriptor {
            format: swap_chain_descriptor.format,
            alpha_blend: wgpu::BlendDescriptor::REPLACE,
            color_blend: wgpu::BlendDescriptor::REPLACE,
            write_mask: wgpu::ColorWrite::ALL,
        }],
        depth_stencil_state: None,
        index_format: wgpu::IndexFormat::Uint16,
        vertex_buffers: &[wgpu::VertexBufferDescriptor {
            stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Vertex,
            attributes: &[wgpu::VertexAttributeDescriptor {
                format: wgpu::VertexFormat::Float3,
                offset: 0,
                shader_location: 0,
            }],
        }],
        sample_count: 1,
        sample_mask: !0,
        alpha_to_coverage_enabled: false,
    });

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
                let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
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
                render_pass.set_pipeline(&pipeline_1);
                render_pass.set_vertex_buffers(0, &[(&vertex_buffer_1, 0)]);
                render_pass.set_index_buffer(&index_buffer_1, 0);
                render_pass.draw_indexed(0..INDICES_1.len() as u32, 0, 0..1);
                render_pass.set_pipeline(&pipeline_2);
                render_pass.set_vertex_buffers(0, &[(&vertex_buffer_2, 0)]);
                render_pass.set_index_buffer(&index_buffer_2, 0);
                render_pass.draw_indexed(0..INDICES_2.len() as u32, 0, 0..1);
            }

            queue.submit(&[encoder.finish()]);

            *control_flow = ControlFlow::Poll;
        }
        _ => {}
    })
}

pub fn prepare_shader(
    compiler: &mut Compiler,
    source_text: &str,
    shader_kind: ShaderKind,
    input_file_name: &str,
    device: &wgpu::Device,
) -> wgpu::ShaderModule {
    let artifact = compiler
        .compile_into_spirv(source_text, shader_kind, input_file_name, "main", None)
        .unwrap();
    device.create_shader_module(artifact.as_binary())
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Vertex {
    pub translation: Vec3,
}
