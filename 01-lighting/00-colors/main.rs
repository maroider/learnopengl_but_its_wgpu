use amethyst_input::{Button, InputEvent, InputHandler, ScrollDirection, StringBindings};
use shaderc::{Compiler, ShaderKind};
use shrev::{EventChannel, ReaderId};
use ultraviolet::{Mat4, Rotor3, Vec2, Vec3, Vec4};
use winit::{
    event::{Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;

#[rustfmt::skip]
const VERTICES: &[Vertex] = &[
   Vertex { translation: Vec3 { x:  0.5, y: -0.5, z: -0.5, }, uv: Vec2 { x: 0.0, y: 0.0, } },
   Vertex { translation: Vec3 { x: -0.5, y: -0.5, z: -0.5, }, uv: Vec2 { x: 0.0, y: 0.0, } },
   Vertex { translation: Vec3 { x:  0.5, y:  0.5, z: -0.5, }, uv: Vec2 { x: 1.0, y: 1.0, } },
   Vertex { translation: Vec3 { x:  0.5, y:  0.5, z: -0.5, }, uv: Vec2 { x: 1.0, y: 1.0, } },
   Vertex { translation: Vec3 { x: -0.5, y: -0.5, z: -0.5, }, uv: Vec2 { x: 1.0, y: 0.0, } },
   Vertex { translation: Vec3 { x: -0.5, y:  0.5, z: -0.5, }, uv: Vec2 { x: 0.0, y: 1.0, } },

   Vertex { translation: Vec3 { x: -0.5, y: -0.5, z:  0.5, }, uv: Vec2 { x: 0.0, y: 0.0, } },
   Vertex { translation: Vec3 { x:  0.5, y: -0.5, z:  0.5, }, uv: Vec2 { x: 1.0, y: 0.0, } },
   Vertex { translation: Vec3 { x:  0.5, y:  0.5, z:  0.5, }, uv: Vec2 { x: 1.0, y: 1.0, } },
   Vertex { translation: Vec3 { x:  0.5, y:  0.5, z:  0.5, }, uv: Vec2 { x: 1.0, y: 1.0, } },
   Vertex { translation: Vec3 { x: -0.5, y:  0.5, z:  0.5, }, uv: Vec2 { x: 0.0, y: 1.0, } },
   Vertex { translation: Vec3 { x: -0.5, y: -0.5, z:  0.5, }, uv: Vec2 { x: 0.0, y: 0.0, } },

   Vertex { translation: Vec3 { x: -0.5, y:  0.5, z:  0.5, }, uv: Vec2 { x: 1.0, y: 0.0, } },
   Vertex { translation: Vec3 { x: -0.5, y:  0.5, z: -0.5, }, uv: Vec2 { x: 1.0, y: 1.0, } },
   Vertex { translation: Vec3 { x: -0.5, y: -0.5, z: -0.5, }, uv: Vec2 { x: 0.0, y: 1.0, } },
   Vertex { translation: Vec3 { x: -0.5, y: -0.5, z: -0.5, }, uv: Vec2 { x: 0.0, y: 1.0, } },
   Vertex { translation: Vec3 { x: -0.5, y: -0.5, z:  0.5, }, uv: Vec2 { x: 0.0, y: 0.0, } },
   Vertex { translation: Vec3 { x: -0.5, y:  0.5, z:  0.5, }, uv: Vec2 { x: 1.0, y: 0.0, } },

   Vertex { translation: Vec3 { x:  0.5, y:  0.5, z:  0.5, }, uv: Vec2 { x: 1.0, y: 0.0, } },
   Vertex { translation: Vec3 { x:  0.5, y: -0.5, z: -0.5, }, uv: Vec2 { x: 0.0, y: 1.0, } },
   Vertex { translation: Vec3 { x:  0.5, y:  0.5, z: -0.5, }, uv: Vec2 { x: 1.0, y: 1.0, } },
   Vertex { translation: Vec3 { x:  0.5, y: -0.5, z: -0.5, }, uv: Vec2 { x: 0.0, y: 1.0, } },
   Vertex { translation: Vec3 { x:  0.5, y:  0.5, z:  0.5, }, uv: Vec2 { x: 1.0, y: 0.0, } },
   Vertex { translation: Vec3 { x:  0.5, y: -0.5, z:  0.5, }, uv: Vec2 { x: 0.0, y: 0.0, } },

   Vertex { translation: Vec3 { x: -0.5, y: -0.5, z: -0.5, }, uv: Vec2 { x: 0.0, y: 1.0, } },
   Vertex { translation: Vec3 { x:  0.5, y: -0.5, z: -0.5, }, uv: Vec2 { x: 1.0, y: 1.0, } },
   Vertex { translation: Vec3 { x:  0.5, y: -0.5, z:  0.5, }, uv: Vec2 { x: 1.0, y: 0.0, } },
   Vertex { translation: Vec3 { x:  0.5, y: -0.5, z:  0.5, }, uv: Vec2 { x: 1.0, y: 0.0, } },
   Vertex { translation: Vec3 { x: -0.5, y: -0.5, z:  0.5, }, uv: Vec2 { x: 0.0, y: 0.0, } },
   Vertex { translation: Vec3 { x: -0.5, y: -0.5, z: -0.5, }, uv: Vec2 { x: 0.0, y: 1.0, } },

   Vertex { translation: Vec3 { x:  0.5, y:  0.5, z: -0.5, }, uv: Vec2 { x: 1.0, y: 1.0, } },
   Vertex { translation: Vec3 { x: -0.5, y:  0.5, z: -0.5, }, uv: Vec2 { x: 0.0, y: 1.0, } },
   Vertex { translation: Vec3 { x:  0.5, y:  0.5, z:  0.5, }, uv: Vec2 { x: 1.0, y: 0.0, } },
   Vertex { translation: Vec3 { x:  0.5, y:  0.5, z:  0.5, }, uv: Vec2 { x: 1.0, y: 0.0, } },
   Vertex { translation: Vec3 { x: -0.5, y:  0.5, z: -0.5, }, uv: Vec2 { x: 0.0, y: 1.0, } },
   Vertex { translation: Vec3 { x: -0.5, y:  0.5, z:  0.5, }, uv: Vec2 { x: 0.0, y: 0.0, } },
];

#[rustfmt::skip]
const CUBES: &[Instance] = &[
    Instance { model: Mat4 {
        cols: [
            Vec4 { x:  1.0, y:  0.0, z:  0.0, w:  0.0 },
            Vec4 { x:  0.0, y:  1.0, z:  0.0, w:  0.0 },
            Vec4 { x:  0.0, y:  0.0, z:  1.0, w:  0.0 },
            Vec4 { x:  0.0, y:  0.0, z:  0.0, w:  1.0 },
        ]
    }},
];

#[rustfmt::skip]
const LIGHT_SOURCES: &[Instance] = &[
    Instance { model: Mat4 {
        cols: [
            Vec4 { x:  0.2, y:  0.0,  z:  0.0, w:  0.0 },
            Vec4 { x:  0.0, y:  0.2,  z:  0.0, w:  0.0 },
            Vec4 { x:  0.0, y:  0.0,  z:  0.2, w:  0.0 },
            Vec4 { x:  1.2, y:  1.0,  z: -2.0, w:  1.0 },
        ]
    }},
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
    let fs_module = prepare_shader(
        &mut shader_compiler,
        include_str!("shader.frag"),
        ShaderKind::Fragment,
        "shader.frag",
        &device,
    );
    let light_source_fs_module = prepare_shader(
        &mut shader_compiler,
        include_str!("light_source.frag"),
        ShaderKind::Fragment,
        "light_source.frag",
        &device,
    );

    let vertex_buffer = device
        .create_buffer_mapped(VERTICES.len(), wgpu::BufferUsage::VERTEX)
        .fill_from_slice(VERTICES);

    let cube_instance_buffer = device
        .create_buffer_mapped(CUBES.len(), wgpu::BufferUsage::VERTEX)
        .fill_from_slice(CUBES);
    let light_source_instance_buffer = device
        .create_buffer_mapped(LIGHT_SOURCES.len(), wgpu::BufferUsage::VERTEX)
        .fill_from_slice(LIGHT_SOURCES);

    let vertex_uniform_bind_group_layout =
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            bindings: &[wgpu::BindGroupLayoutBinding {
                binding: 0,
                visibility: wgpu::ShaderStage::VERTEX,
                ty: wgpu::BindingType::UniformBuffer { dynamic: false },
            }],
        });

    let fragment_uniform_bind_group_layout =
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            bindings: &[wgpu::BindGroupLayoutBinding {
                binding: 0,
                visibility: wgpu::ShaderStage::FRAGMENT,
                ty: wgpu::BindingType::UniformBuffer { dynamic: false },
            }],
        });

    let mut event_channel = shrev::EventChannel::new();
    let event_reader = event_channel.register_reader();
    let mut input_handler = InputHandler::<StringBindings>::new();

    let mut camera = Camera {
        translation: Vec3::new(0.0, 0.0, -3.0),
        pitch: 0.0,
        yaw: 0.0,
        roll: 0.0,
        movement_speed: 0.1,
        mouse_sensitivity: 0.0025,
        zoom: std::f32::consts::FRAC_PI_3,
        is_controlled: false,
        event_reader,
    };

    let vertex_uniform_buffer = device
        .create_buffer_mapped(1, wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST)
        .fill_from_slice(&[camera.get_view_projection_matrix(
            size.width as f32,
            size.height as f32,
            0.1,
            100.0,
        )]);

    let vertex_uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: &vertex_uniform_bind_group_layout,
        bindings: &[wgpu::Binding {
            binding: 0,
            resource: wgpu::BindingResource::Buffer {
                buffer: &vertex_uniform_buffer,
                range: 0..std::mem::size_of::<Mat4>() as wgpu::BufferAddress,
            },
        }],
    });

    let fragment_uniform_buffer = device
        .create_buffer_mapped(2, wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST)
        .fill_from_slice(&[Vec3::new(1.0, 0.5, 0.31), Vec3::new(1.0, 1.0, 1.0)]);

    let fragment_uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: &fragment_uniform_bind_group_layout,
        bindings: &[wgpu::Binding {
            binding: 0,
            resource: wgpu::BindingResource::Buffer {
                buffer: &fragment_uniform_buffer,
                range: 0..std::mem::size_of::<[Vec3; 2]>() as wgpu::BufferAddress,
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

    let (mut depth_texture, mut depth_texture_view) =
        create_depth_texture(&device, &swap_chain_descriptor);

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        bind_group_layouts: &[
            &vertex_uniform_bind_group_layout,
            &fragment_uniform_bind_group_layout,
        ],
    });

    let render_pipeline_descriptor = wgpu::RenderPipelineDescriptor {
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
        depth_stencil_state: Some(wgpu::DepthStencilStateDescriptor {
            format: DEPTH_FORMAT,
            depth_write_enabled: true,
            depth_compare: wgpu::CompareFunction::Less,
            stencil_front: wgpu::StencilStateFaceDescriptor::IGNORE,
            stencil_back: wgpu::StencilStateFaceDescriptor::IGNORE,
            stencil_read_mask: 0,
            stencil_write_mask: 0,
        }),
        index_format: wgpu::IndexFormat::Uint16,
        vertex_buffers: &[
            wgpu::VertexBufferDescriptor {
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
            },
            wgpu::VertexBufferDescriptor {
                stride: std::mem::size_of::<Instance>() as wgpu::BufferAddress,
                step_mode: wgpu::InputStepMode::Instance,
                attributes: &[
                    wgpu::VertexAttributeDescriptor {
                        offset: 0,
                        format: wgpu::VertexFormat::Float4,
                        shader_location: 2,
                    },
                    wgpu::VertexAttributeDescriptor {
                        offset: std::mem::size_of::<Vec4>() as wgpu::BufferAddress,
                        format: wgpu::VertexFormat::Float4,
                        shader_location: 3,
                    },
                    wgpu::VertexAttributeDescriptor {
                        offset: std::mem::size_of::<Vec4>() as wgpu::BufferAddress * 2,
                        format: wgpu::VertexFormat::Float4,
                        shader_location: 4,
                    },
                    wgpu::VertexAttributeDescriptor {
                        offset: std::mem::size_of::<Vec4>() as wgpu::BufferAddress * 3,
                        format: wgpu::VertexFormat::Float4,
                        shader_location: 5,
                    },
                ],
            },
        ],
        sample_count: 1,
        sample_mask: !0,
        alpha_to_coverage_enabled: false,
    };
    let pipeline = device.create_render_pipeline(&render_pipeline_descriptor);

    let light_source_pipeline_layout =
        device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            bind_group_layouts: &[&vertex_uniform_bind_group_layout],
        });

    let mut light_source_pipeline_descriptor = render_pipeline_descriptor.clone();
    light_source_pipeline_descriptor.layout = &light_source_pipeline_layout;
    light_source_pipeline_descriptor.fragment_stage = Some(wgpu::ProgrammableStageDescriptor {
        module: &light_source_fs_module,
        entry_point: "main",
    });
    let light_source_pipeline = device.create_render_pipeline(&light_source_pipeline_descriptor);

    let mut swap_chain = device.create_swap_chain(&surface, &swap_chain_descriptor);

    event_loop.run(move |event, _, control_flow| {
        input_handler.send_event(&event, &mut event_channel);

        camera.on_event(&event_channel);

        match event {
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
                let dt = create_depth_texture(&device, &swap_chain_descriptor);
                depth_texture = dt.0;
                depth_texture_view = dt.1;
                *control_flow = ControlFlow::Poll;
            }
            Event::WindowEvent {
                event:
                    WindowEvent::MouseInput {
                        button: winit::event::MouseButton::Left,
                        state: winit::event::ElementState::Pressed,
                        ..
                    },
                ..
            } => {
                camera.is_controlled = true;
                window.set_cursor_grab(true).ok();
                window.set_cursor_visible(false);
                *control_flow = ControlFlow::Poll;
            }
            Event::WindowEvent {
                event:
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                state: winit::event::ElementState::Pressed,
                                ..
                            },
                        ..
                    },
                ..
            } => {
                camera.is_controlled = false;
                window.set_cursor_grab(false).ok();
                window.set_cursor_visible(true);
                *control_flow = ControlFlow::Poll;
            }
            Event::MainEventsCleared => {
                window.request_redraw();
                *control_flow = ControlFlow::Poll;
            }
            Event::RedrawRequested(_) => {
                input_handler.send_frame_begin();
                camera.update(&input_handler);

                let frame = swap_chain.get_next_texture();
                let mut encoder =
                    device.create_command_encoder(&wgpu::CommandEncoderDescriptor { todo: 0 });
                {
                    let view_projection = camera.get_view_projection_matrix(
                        size.width as f32,
                        size.height as f32,
                        0.1,
                        100.0,
                    );
                    let staging_buffer = device
                        .create_buffer_mapped(1, wgpu::BufferUsage::COPY_SRC)
                        .fill_from_slice(&[view_projection]);
                    encoder.copy_buffer_to_buffer(
                        &staging_buffer,
                        0,
                        &vertex_uniform_buffer,
                        0,
                        std::mem::size_of::<Mat4>() as wgpu::BufferAddress,
                    );

                    let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                            attachment: &frame.view,
                            resolve_target: None,
                            load_op: wgpu::LoadOp::Clear,
                            store_op: wgpu::StoreOp::Store,
                            clear_color: wgpu::Color {
                                r: 0.1,
                                g: 0.1,
                                b: 0.1,
                                a: 1.0,
                            },
                        }],
                        depth_stencil_attachment: Some(
                            wgpu::RenderPassDepthStencilAttachmentDescriptor {
                                attachment: &depth_texture_view,
                                depth_load_op: wgpu::LoadOp::Clear,
                                depth_store_op: wgpu::StoreOp::Store,
                                clear_depth: 1.0,
                                stencil_load_op: wgpu::LoadOp::Clear,
                                stencil_store_op: wgpu::StoreOp::Store,
                                clear_stencil: 0,
                            },
                        ),
                    });
                    render_pass.set_pipeline(&pipeline);
                    render_pass.set_bind_group(0, &vertex_uniform_bind_group, &[]);
                    render_pass.set_bind_group(1, &fragment_uniform_bind_group, &[]);
                    render_pass
                        .set_vertex_buffers(0, &[(&vertex_buffer, 0), (&cube_instance_buffer, 0)]);
                    render_pass.draw(0..VERTICES.len() as u32, 0..CUBES.len() as u32);

                    render_pass.set_pipeline(&light_source_pipeline);
                    render_pass.set_bind_group(0, &vertex_uniform_bind_group, &[]);
                    render_pass.set_vertex_buffers(
                        0,
                        &[(&vertex_buffer, 0), (&light_source_instance_buffer, 0)],
                    );
                    render_pass.draw(0..VERTICES.len() as u32, 0..LIGHT_SOURCES.len() as u32);
                }

                queue.submit(&[encoder.finish()]);

                *control_flow = ControlFlow::Poll;
            }
            _ => {}
        }
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

pub fn create_depth_texture(
    device: &wgpu::Device,
    swap_chain_desc: &wgpu::SwapChainDescriptor,
) -> (wgpu::Texture, wgpu::TextureView) {
    let desc = wgpu::TextureDescriptor {
        format: DEPTH_FORMAT,
        usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
        ..swap_chain_desc.to_texture_desc()
    };
    let texture = device.create_texture(&desc);
    let view = texture.create_default_view();
    (texture, view)
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Vertex {
    pub translation: Vec3,
    pub uv: Vec2,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Instance {
    pub model: Mat4,
}

pub struct Camera {
    pub translation: Vec3,
    pub pitch: f32,
    pub yaw: f32,
    pub roll: f32,
    pub movement_speed: f32,
    pub mouse_sensitivity: f32,
    pub zoom: f32,
    pub is_controlled: bool,

    pub event_reader: ReaderId<InputEvent<StringBindings>>,
}

impl Camera {
    pub fn get_view_projection_matrix(&self, width: f32, height: f32, near: f32, far: f32) -> Mat4 {
        ultraviolet::projection::perspective_vk(self.zoom, width / height, near, far)
            * (Rotor3::from_rotation_xy(self.roll)
                * Rotor3::from_rotation_yz(self.pitch)
                * Rotor3::from_rotation_xz(self.yaw))
            .into_matrix()
            .into_homogeneous()
            * Mat4::from_translation(self.translation)
    }

    pub fn on_event(&mut self, event_channel: &EventChannel<InputEvent<StringBindings>>) {
        if !self.is_controlled {
            return;
        }

        for event in event_channel.read(&mut self.event_reader) {
            match event {
                InputEvent::MouseMoved { delta_x, delta_y } => {
                    self.pitch += delta_y * self.mouse_sensitivity;
                    self.yaw += -(delta_x * self.mouse_sensitivity);

                    if self.pitch > std::f32::consts::FRAC_PI_2 {
                        self.pitch = std::f32::consts::FRAC_PI_2;
                    } else if self.pitch < -std::f32::consts::FRAC_PI_2 {
                        self.pitch = -std::f32::consts::FRAC_PI_2;
                    }
                    if self.yaw > 2.0 * std::f32::consts::PI {
                        self.yaw -= 2.0 * std::f32::consts::PI;
                    } else if self.yaw < 0.0 {
                        self.yaw = 2.0 * std::f32::consts::PI - self.yaw;
                    }
                }
                InputEvent::MouseWheelMoved(direction) => match direction {
                    ScrollDirection::ScrollUp => {
                        if self.zoom > 0.2 {
                            self.zoom -= 0.1;
                        } else {
                            self.zoom = 0.1;
                        }
                    }
                    ScrollDirection::ScrollDown => {
                        if self.zoom < std::f32::consts::PI - 0.1 {
                            self.zoom += 0.1;
                        } else {
                            self.zoom = std::f32::consts::PI - 0.1;
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }

    pub fn update(&mut self, input_handler: &InputHandler<StringBindings>) {
        let forward_displacement = (if input_handler.button_is_down(Button::ScanCode(0x11)) {
            -1.0
        } else {
            0.0
        } + if input_handler.button_is_down(Button::ScanCode(0x1F)) {
            1.0
        } else {
            0.0
        }) * self.movement_speed;
        let sideways_displacement = (if input_handler.button_is_down(Button::ScanCode(0x1E)) {
            -1.0
        } else {
            0.0
        } + if input_handler.button_is_down(Button::ScanCode(0x20)) {
            1.0
        } else {
            0.0
        }) * self.movement_speed;
        let vertical_displacement = (if input_handler.button_is_down(Button::ScanCode(0x39)) {
            -1.0
        } else {
            0.0
        } + if input_handler.button_is_down(Button::ScanCode(0x2e)) {
            1.0
        } else {
            0.0
        }) * self.movement_speed;

        let yaw = self.yaw - std::f32::consts::FRAC_PI_2;

        let x_displacement = -yaw.cos() * forward_displacement + yaw.sin() * sideways_displacement;
        let y_displacement = -self.pitch.sin() * forward_displacement + vertical_displacement;
        let z_displacement = yaw.sin() * forward_displacement + yaw.cos() * sideways_displacement;

        self.translation += Vec3::new(x_displacement, y_displacement, z_displacement);
    }
}
