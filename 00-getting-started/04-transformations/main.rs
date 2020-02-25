use std::time::Instant;

use shaderc::{Compiler, ShaderKind};
use ultraviolet::{Bivec3, Mat4, Rotor3, Vec2, Vec3};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

#[rustfmt::skip]
const VERTICES: &[Vertex] = &[
    Vertex { translation: Vec3 { x: -0.5, y: -0.5, z:  0.0 }, uv: Vec2 { x: 0.0, y: 0.0 } },
    Vertex { translation: Vec3 { x: -0.5, y:  0.5, z:  0.0 }, uv: Vec2 { x: 0.0, y: 1.0 } },
    Vertex { translation: Vec3 { x:  0.5, y:  0.5, z:  0.0 }, uv: Vec2 { x: 1.0, y: 1.0 } },
    Vertex { translation: Vec3 { x:  0.5, y: -0.5, z:  0.0 }, uv: Vec2 { x: 1.0, y: 0.0 } },
];

#[rustfmt::skip]
const INDICES: &[u16] = &[
    0, 1, 2,
    2, 3, 0,
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

    let mut init_encoder =
        device.create_command_encoder(&wgpu::CommandEncoderDescriptor { todo: 0 });

    let mut shader_compiler = Compiler::new().expect("Could not initialize shader compiler");

    let vs_module = prepare_shader(
        &mut shader_compiler,
        include_str!("shader.vert"),
        ShaderKind::Vertex,
        "shader.vert",
        &device,
    );
    let fs_module = prepare_shader(
        &mut shader_compiler,
        include_str!("shader.frag"),
        ShaderKind::Fragment,
        "shader.frag",
        &device,
    );

    let vertex_buffer = device
        .create_buffer_mapped(VERTICES.len(), wgpu::BufferUsage::VERTEX)
        .fill_from_slice(VERTICES);
    let index_buffer = device
        .create_buffer_mapped(INDICES.len(), wgpu::BufferUsage::INDEX)
        .fill_from_slice(INDICES);

    let uniform_bind_group_layout =
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            bindings: &[wgpu::BindGroupLayoutBinding {
                binding: 0,
                visibility: wgpu::ShaderStage::VERTEX,
                ty: wgpu::BindingType::UniformBuffer { dynamic: false },
            }],
        });

    let texture_bind_group_layout_descriptor = wgpu::BindGroupLayoutDescriptor {
        bindings: &[
            wgpu::BindGroupLayoutBinding {
                binding: 0,
                visibility: wgpu::ShaderStage::FRAGMENT,
                ty: wgpu::BindingType::SampledTexture {
                    multisampled: false,
                    dimension: wgpu::TextureViewDimension::D2,
                },
            },
            wgpu::BindGroupLayoutBinding {
                binding: 1,
                visibility: wgpu::ShaderStage::FRAGMENT,
                ty: wgpu::BindingType::Sampler,
            },
        ],
    };
    let texture_bind_group_layout =
        device.create_bind_group_layout(&texture_bind_group_layout_descriptor);

    let texture_1_source = include_bytes!("container.jpg");
    let texture_1_image = image::load_from_memory(texture_1_source).unwrap().to_rgba();
    let (width, height) = texture_1_image.dimensions();
    let raw_texture_1 = texture_1_image.to_vec();
    let (_, _, _, texture_1_bind_group) = create_sampled_texture2d(
        &device,
        &mut init_encoder,
        &texture_bind_group_layout,
        &raw_texture_1,
        width,
        height,
    );

    let texture_2_source = include_bytes!("awesomeface.png");
    let texture_2_image = image::load_from_memory(texture_2_source).unwrap().to_rgba();
    let (width, height) = texture_2_image.dimensions();
    let raw_texture_2 = texture_2_image.to_vec();
    let (_, _, _, texture_2_bind_group) = create_sampled_texture2d(
        &device,
        &mut init_encoder,
        &texture_bind_group_layout,
        &raw_texture_2,
        width,
        height,
    );

    let uniform_buffer = device
        .create_buffer_mapped(1, wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST)
        .fill_from_slice(&[Mat4::identity() * Mat4::from_translation(Vec3::new(0.5, 0.5, 0.0))]);

    let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: &uniform_bind_group_layout,
        bindings: &[wgpu::Binding {
            binding: 0,
            resource: wgpu::BindingResource::Buffer {
                buffer: &uniform_buffer,
                range: 0..1,
            },
        }],
    });

    let mut swap_chain_descriptor = wgpu::SwapChainDescriptor {
        usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
        format: wgpu::TextureFormat::Bgra8Unorm,
        width: size.width,
        height: size.height,
        present_mode: wgpu::PresentMode::Vsync,
    };

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        bind_group_layouts: &[
            &uniform_bind_group_layout,
            &texture_bind_group_layout,
            &texture_bind_group_layout,
        ],
    });

    let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        layout: &pipeline_layout,
        vertex_stage: wgpu::ProgrammableStageDescriptor {
            module: &vs_module,
            entry_point: "main",
        },
        fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
            module: &fs_module,
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
            attributes: &[
                wgpu::VertexAttributeDescriptor {
                    offset: 0,
                    format: wgpu::VertexFormat::Float3,
                    shader_location: 0,
                },
                wgpu::VertexAttributeDescriptor {
                    offset: std::mem::size_of::<Vec3>() as wgpu::BufferAddress,
                    format: wgpu::VertexFormat::Float2,
                    shader_location: 1,
                },
            ],
        }],
        sample_count: 1,
        sample_mask: !0,
        alpha_to_coverage_enabled: false,
    });

    let mut swap_chain = device.create_swap_chain(&surface, &swap_chain_descriptor);

    queue.submit(&[init_encoder.finish()]);

    let start_time = Instant::now();

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
            {
                let staging_buffer = device
                    .create_buffer_mapped(1, wgpu::BufferUsage::COPY_SRC)
                    .fill_from_slice(&[Mat4::identity()
                        * Mat4::from_translation(Vec3::new(0.5, 0.5, 0.0))
                        * Rotor3::from_angle_plane(
                            -start_time.elapsed().as_secs_f32(),
                            Bivec3::from_normalized_axis(Vec3::unit_z()),
                        )
                        .into_matrix()
                        .into_homogeneous()]);
                let mut encoder =
                    device.create_command_encoder(&wgpu::CommandEncoderDescriptor { todo: 0 });
                encoder.copy_buffer_to_buffer(
                    &staging_buffer,
                    0,
                    &uniform_buffer,
                    0,
                    std::mem::size_of::<Mat4>() as wgpu::BufferAddress,
                );

                queue.submit(&[encoder.finish()]);
            }

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
                render_pass.set_pipeline(&pipeline);
                render_pass.set_bind_group(0, &uniform_bind_group, &[]);
                render_pass.set_bind_group(1, &texture_1_bind_group, &[]);
                render_pass.set_bind_group(2, &texture_2_bind_group, &[]);
                render_pass.set_vertex_buffers(0, &[(&vertex_buffer, 0)]);
                render_pass.set_index_buffer(&index_buffer, 0);
                render_pass.draw_indexed(0..INDICES.len() as u32, 0, 0..1);
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

pub fn create_sampled_texture2d(
    device: &wgpu::Device,
    encoder: &mut wgpu::CommandEncoder,
    bind_group_layout: &wgpu::BindGroupLayout,
    raw_texture: &[u8],
    width: u32,
    height: u32,
) -> (
    wgpu::Texture,
    wgpu::TextureView,
    wgpu::Sampler,
    wgpu::BindGroup,
) {
    let texture_extent = wgpu::Extent3d {
        width,
        height,
        depth: 1,
    };

    let texture_buffer = device
        .create_buffer_mapped(raw_texture.len(), wgpu::BufferUsage::COPY_SRC)
        .fill_from_slice(&raw_texture);
    let texture = device.create_texture(&wgpu::TextureDescriptor {
        size: texture_extent,
        array_layer_count: 1,
        mip_level_count: 0,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8Unorm,
        usage: wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::COPY_DST,
    });

    encoder.copy_buffer_to_texture(
        wgpu::BufferCopyView {
            buffer: &texture_buffer,
            offset: 0,
            row_pitch: 4 * width,
            image_height: height,
        },
        wgpu::TextureCopyView {
            texture: &texture,
            mip_level: 0,
            array_layer: 0,
            origin: wgpu::Origin3d {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        },
        texture_extent,
    );

    let texture_view = texture.create_default_view();
    let texture_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
        address_mode_u: wgpu::AddressMode::ClampToEdge,
        address_mode_v: wgpu::AddressMode::ClampToEdge,
        address_mode_w: wgpu::AddressMode::ClampToEdge,
        mag_filter: wgpu::FilterMode::Nearest,
        min_filter: wgpu::FilterMode::Linear,
        mipmap_filter: wgpu::FilterMode::Nearest,
        lod_min_clamp: 0.0,
        lod_max_clamp: 0.0,
        compare_function: wgpu::CompareFunction::Never,
    });
    let texture_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: &bind_group_layout,
        bindings: &[
            wgpu::Binding {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&texture_view),
            },
            wgpu::Binding {
                binding: 1,
                resource: wgpu::BindingResource::Sampler(&texture_sampler),
            },
        ],
    });

    (texture, texture_view, texture_sampler, texture_bind_group)
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Vertex {
    pub translation: Vec3,
    pub uv: Vec2,
}
