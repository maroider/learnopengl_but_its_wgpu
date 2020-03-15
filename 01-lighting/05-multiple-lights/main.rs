use amethyst_input::{InputHandler, StringBindings};
use shaderc::{Compiler, ShaderKind};
use ultraviolet::{Mat4, Vec2, Vec3, Vec4};
use winit::{
    event::{Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use learnopengl_but_its_wgpu::{Align16, Camera};

const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;

#[rustfmt::skip]
const VERTICES: &[Vertex] = &[
    Vertex { translation: Vec3 { x:  0.5, y: -0.5, z: -0.5, }, normal: Vec3 { x:  0.0, y:  0.0, z: -1.0 }, uv: Vec2 { x: 1.0, y: 0.0, } },
    Vertex { translation: Vec3 { x: -0.5, y: -0.5, z: -0.5, }, normal: Vec3 { x:  0.0, y:  0.0, z: -1.0 }, uv: Vec2 { x: 0.0, y: 0.0, } },
    Vertex { translation: Vec3 { x:  0.5, y:  0.5, z: -0.5, }, normal: Vec3 { x:  0.0, y:  0.0, z: -1.0 }, uv: Vec2 { x: 1.0, y: 1.0, } },
    Vertex { translation: Vec3 { x:  0.5, y:  0.5, z: -0.5, }, normal: Vec3 { x:  0.0, y:  0.0, z: -1.0 }, uv: Vec2 { x: 1.0, y: 1.0, } },
    Vertex { translation: Vec3 { x: -0.5, y: -0.5, z: -0.5, }, normal: Vec3 { x:  0.0, y:  0.0, z: -1.0 }, uv: Vec2 { x: 0.0, y: 0.0, } },
    Vertex { translation: Vec3 { x: -0.5, y:  0.5, z: -0.5, }, normal: Vec3 { x:  0.0, y:  0.0, z: -1.0 }, uv: Vec2 { x: 0.0, y: 1.0, } },

    Vertex { translation: Vec3 { x: -0.5, y: -0.5, z:  0.5, }, normal: Vec3 { x:  0.0, y:  0.0, z:  1.0 }, uv: Vec2 { x: 0.0, y: 0.0, } },
    Vertex { translation: Vec3 { x:  0.5, y: -0.5, z:  0.5, }, normal: Vec3 { x:  0.0, y:  0.0, z:  1.0 }, uv: Vec2 { x: 1.0, y: 0.0, } },
    Vertex { translation: Vec3 { x:  0.5, y:  0.5, z:  0.5, }, normal: Vec3 { x:  0.0, y:  0.0, z:  1.0 }, uv: Vec2 { x: 1.0, y: 1.0, } },
    Vertex { translation: Vec3 { x:  0.5, y:  0.5, z:  0.5, }, normal: Vec3 { x:  0.0, y:  0.0, z:  1.0 }, uv: Vec2 { x: 1.0, y: 1.0, } },
    Vertex { translation: Vec3 { x: -0.5, y:  0.5, z:  0.5, }, normal: Vec3 { x:  0.0, y:  0.0, z:  1.0 }, uv: Vec2 { x: 0.0, y: 1.0, } },
    Vertex { translation: Vec3 { x: -0.5, y: -0.5, z:  0.5, }, normal: Vec3 { x:  0.0, y:  0.0, z:  1.0 }, uv: Vec2 { x: 0.0, y: 0.0, } },

    Vertex { translation: Vec3 { x: -0.5, y:  0.5, z:  0.5, }, normal: Vec3 { x: -1.0, y:  0.0, z:  0.0 }, uv: Vec2 { x: 1.0, y: 0.0, } },
    Vertex { translation: Vec3 { x: -0.5, y:  0.5, z: -0.5, }, normal: Vec3 { x: -1.0, y:  0.0, z:  0.0 }, uv: Vec2 { x: 1.0, y: 1.0, } },
    Vertex { translation: Vec3 { x: -0.5, y: -0.5, z: -0.5, }, normal: Vec3 { x: -1.0, y:  0.0, z:  0.0 }, uv: Vec2 { x: 0.0, y: 1.0, } },
    Vertex { translation: Vec3 { x: -0.5, y: -0.5, z: -0.5, }, normal: Vec3 { x: -1.0, y:  0.0, z:  0.0 }, uv: Vec2 { x: 0.0, y: 1.0, } },
    Vertex { translation: Vec3 { x: -0.5, y: -0.5, z:  0.5, }, normal: Vec3 { x: -1.0, y:  0.0, z:  0.0 }, uv: Vec2 { x: 0.0, y: 0.0, } },
    Vertex { translation: Vec3 { x: -0.5, y:  0.5, z:  0.5, }, normal: Vec3 { x: -1.0, y:  0.0, z:  0.0 }, uv: Vec2 { x: 1.0, y: 0.0, } },

    Vertex { translation: Vec3 { x:  0.5, y:  0.5, z:  0.5, }, normal: Vec3 { x:  1.0, y:  0.0, z:  0.0 }, uv: Vec2 { x: 1.0, y: 0.0, } },
    Vertex { translation: Vec3 { x:  0.5, y: -0.5, z: -0.5, }, normal: Vec3 { x:  1.0, y:  0.0, z:  0.0 }, uv: Vec2 { x: 0.0, y: 1.0, } },
    Vertex { translation: Vec3 { x:  0.5, y:  0.5, z: -0.5, }, normal: Vec3 { x:  1.0, y:  0.0, z:  0.0 }, uv: Vec2 { x: 1.0, y: 1.0, } },
    Vertex { translation: Vec3 { x:  0.5, y: -0.5, z: -0.5, }, normal: Vec3 { x:  1.0, y:  0.0, z:  0.0 }, uv: Vec2 { x: 0.0, y: 1.0, } },
    Vertex { translation: Vec3 { x:  0.5, y:  0.5, z:  0.5, }, normal: Vec3 { x:  1.0, y:  0.0, z:  0.0 }, uv: Vec2 { x: 1.0, y: 0.0, } },
    Vertex { translation: Vec3 { x:  0.5, y: -0.5, z:  0.5, }, normal: Vec3 { x:  1.0, y:  0.0, z:  0.0 }, uv: Vec2 { x: 0.0, y: 0.0, } },

    Vertex { translation: Vec3 { x: -0.5, y: -0.5, z: -0.5, }, normal: Vec3 { x:  0.0, y: -1.0, z:  0.0 }, uv: Vec2 { x: 0.0, y: 1.0, } },
    Vertex { translation: Vec3 { x:  0.5, y: -0.5, z: -0.5, }, normal: Vec3 { x:  0.0, y: -1.0, z:  0.0 }, uv: Vec2 { x: 1.0, y: 1.0, } },
    Vertex { translation: Vec3 { x:  0.5, y: -0.5, z:  0.5, }, normal: Vec3 { x:  0.0, y: -1.0, z:  0.0 }, uv: Vec2 { x: 1.0, y: 0.0, } },
    Vertex { translation: Vec3 { x:  0.5, y: -0.5, z:  0.5, }, normal: Vec3 { x:  0.0, y: -1.0, z:  0.0 }, uv: Vec2 { x: 1.0, y: 0.0, } },
    Vertex { translation: Vec3 { x: -0.5, y: -0.5, z:  0.5, }, normal: Vec3 { x:  0.0, y: -1.0, z:  0.0 }, uv: Vec2 { x: 0.0, y: 0.0, } },
    Vertex { translation: Vec3 { x: -0.5, y: -0.5, z: -0.5, }, normal: Vec3 { x:  0.0, y: -1.0, z:  0.0 }, uv: Vec2 { x: 0.0, y: 1.0, } },

    Vertex { translation: Vec3 { x:  0.5, y:  0.5, z: -0.5, }, normal: Vec3 { x:  0.0, y:  1.0, z:  0.0 }, uv: Vec2 { x: 1.0, y: 1.0, } },
    Vertex { translation: Vec3 { x: -0.5, y:  0.5, z: -0.5, }, normal: Vec3 { x:  0.0, y:  1.0, z:  0.0 }, uv: Vec2 { x: 0.0, y: 1.0, } },
    Vertex { translation: Vec3 { x:  0.5, y:  0.5, z:  0.5, }, normal: Vec3 { x:  0.0, y:  1.0, z:  0.0 }, uv: Vec2 { x: 1.0, y: 0.0, } },
    Vertex { translation: Vec3 { x:  0.5, y:  0.5, z:  0.5, }, normal: Vec3 { x:  0.0, y:  1.0, z:  0.0 }, uv: Vec2 { x: 1.0, y: 0.0, } },
    Vertex { translation: Vec3 { x: -0.5, y:  0.5, z: -0.5, }, normal: Vec3 { x:  0.0, y:  1.0, z:  0.0 }, uv: Vec2 { x: 0.0, y: 1.0, } },
    Vertex { translation: Vec3 { x: -0.5, y:  0.5, z:  0.5, }, normal: Vec3 { x:  0.0, y:  1.0, z:  0.0 }, uv: Vec2 { x: 0.0, y: 0.0, } },

];

#[rustfmt::skip]
const CUBES: &[Instance] = &[
    Instance { model: Mat4 {
        cols: [
            Vec4 { x:   1.0, y:   0.0, z:   0.0, w:   0.0 },
            Vec4 { x:   0.0, y:   1.0, z:   0.0, w:   0.0 },
            Vec4 { x:   0.0, y:   0.0, z:   1.0, w:   0.0 },
            Vec4 { x:   0.0, y:   0.0, z:   0.0, w:   1.0 },
        ]
    }},
    Instance { model: Mat4 {
        cols: [
            Vec4 { x:   1.0, y:   0.0, z:   0.0, w:   0.0 },
            Vec4 { x:   0.0, y:   1.0, z:   0.0, w:   0.0 },
            Vec4 { x:   0.0, y:   0.0, z:   1.0, w:   0.0 },
            Vec4 { x:   2.0, y:   5.0, z: -15.0, w:   1.0 },
        ]
    }},
    Instance { model: Mat4 {
        cols: [
            Vec4 { x:   1.0, y:   0.0, z:   0.0, w:   0.0 },
            Vec4 { x:   0.0, y:   1.0, z:   0.0, w:   0.0 },
            Vec4 { x:   0.0, y:   0.0, z:   1.0, w:   0.0 },
            Vec4 { x:  -1.5, y:  -2.2, z:  -2.5, w:   1.0 },
        ]
    }},
    Instance { model: Mat4 {
        cols: [
            Vec4 { x:   1.0, y:   0.0, z:   0.0, w:   0.0 },
            Vec4 { x:   0.0, y:   1.0, z:   0.0, w:   0.0 },
            Vec4 { x:   0.0, y:   0.0, z:   1.0, w:   0.0 },
            Vec4 { x:  -3.8, y:  -2.0, z: -12.0, w:   1.0 },
        ]
    }},
    Instance { model: Mat4 {
        cols: [
            Vec4 { x:   1.0, y:   0.0, z:   0.0, w:   0.0 },
            Vec4 { x:   0.0, y:   1.0, z:   0.0, w:   0.0 },
            Vec4 { x:   0.0, y:   0.0, z:   1.0, w:   0.0 },
            Vec4 { x:   2.4, y:  -0.4, z:  -3.5, w:   1.0 },
        ]
    }},
    Instance { model: Mat4 {
        cols: [
            Vec4 { x:   1.0, y:   0.0, z:   0.0, w:   0.0 },
            Vec4 { x:   0.0, y:   1.0, z:   0.0, w:   0.0 },
            Vec4 { x:   0.0, y:   0.0, z:   1.0, w:   0.0 },
            Vec4 { x:  -1.7, y:   3.0, z:  -7.5, w:   1.0 },
        ]
    }},
    Instance { model: Mat4 {
        cols: [
            Vec4 { x:   1.0, y:   0.0, z:   0.0, w:   0.0 },
            Vec4 { x:   0.0, y:   1.0, z:   0.0, w:   0.0 },
            Vec4 { x:   0.0, y:   0.0, z:   1.0, w:   0.0 },
            Vec4 { x:   1.3, y:  -2.0, z:  -2.5, w:   1.0 },
        ]
    }},
    Instance { model: Mat4 {
        cols: [
            Vec4 { x:   1.0, y:   0.0, z:   0.0, w:   0.0 },
            Vec4 { x:   0.0, y:   1.0, z:   0.0, w:   0.0 },
            Vec4 { x:   0.0, y:   0.0, z:   1.0, w:   0.0 },
            Vec4 { x:   1.5, y:   2.0, z:  -2.5, w:   1.0 },
        ]
    }},
    Instance { model: Mat4 {
        cols: [
            Vec4 { x:   1.0, y:   0.0, z:   0.0, w:   0.0 },
            Vec4 { x:   0.0, y:   1.0, z:   0.0, w:   0.0 },
            Vec4 { x:   0.0, y:   0.0, z:   1.0, w:   0.0 },
            Vec4 { x:   1.5, y:   0.2, z:  -1.5, w:   1.0 },
        ]
    }},
    Instance { model: Mat4 {
        cols: [
            Vec4 { x:   1.0, y:   0.0, z:   0.0, w:   0.0 },
            Vec4 { x:   0.0, y:   1.0, z:   0.0, w:   0.0 },
            Vec4 { x:   0.0, y:   0.0, z:   1.0, w:   0.0 },
            Vec4 { x:  -1.3, y:   1.0, z:  -1.5, w:   1.0 },
        ]
    }},
];

#[rustfmt::skip]
const LIGHT_TRANSLATIONS: [Vec3; 4] = [
    Vec3 { x:   0.7, y:   0.2, z:   2.0 },
    Vec3 { x:   2.3, y:  -3.3, z:  -4.0 },
    Vec3 { x:  -4.0, y:   2.0, z: -12.0 },
    Vec3 { x:   0.0, y:   0.0, z:  -3.0 },
];

#[rustfmt::skip]
const LIGHT_SOURCES: [Instance; 4] = [
    Instance { model: Mat4 {
        cols: [
            Vec4 { x:  0.2,                     y: 0.0,                       z:  0.0,                    w:  0.0 },
            Vec4 { x:  0.0,                     y: 0.2,                       z:  0.0,                    w:  0.0 },
            Vec4 { x:  0.0,                     y: 0.0,                       z:  0.2,                    w:  0.0 },
            Vec4 { x:  LIGHT_TRANSLATIONS[0].x, y:  LIGHT_TRANSLATIONS[0].y,  z: LIGHT_TRANSLATIONS[0].z, w:  1.0 },
        ]
    }},
    Instance { model: Mat4 {
        cols: [
            Vec4 { x:  0.2,                     y: 0.0,                       z:  0.0,                    w:  0.0 },
            Vec4 { x:  0.0,                     y: 0.2,                       z:  0.0,                    w:  0.0 },
            Vec4 { x:  0.0,                     y: 0.0,                       z:  0.2,                    w:  0.0 },
            Vec4 { x:  LIGHT_TRANSLATIONS[1].x, y:  LIGHT_TRANSLATIONS[1].y,  z: LIGHT_TRANSLATIONS[1].z, w:  1.0 },
        ]
    }},
    Instance { model: Mat4 {
        cols: [
            Vec4 { x:  0.2,                     y: 0.0,                       z:  0.0,                    w:  0.0 },
            Vec4 { x:  0.0,                     y: 0.2,                       z:  0.0,                    w:  0.0 },
            Vec4 { x:  0.0,                     y: 0.0,                       z:  0.2,                    w:  0.0 },
            Vec4 { x:  LIGHT_TRANSLATIONS[2].x, y:  LIGHT_TRANSLATIONS[2].y,  z: LIGHT_TRANSLATIONS[2].z, w:  1.0 },
        ]
    }},
    Instance { model: Mat4 {
        cols: [
            Vec4 { x:  0.2,                     y: 0.0,                       z:  0.0,                    w:  0.0 },
            Vec4 { x:  0.0,                     y: 0.2,                       z:  0.0,                    w:  0.0 },
            Vec4 { x:  0.0,                     y: 0.0,                       z:  0.2,                    w:  0.0 },
            Vec4 { x:  LIGHT_TRANSLATIONS[3].x, y:  LIGHT_TRANSLATIONS[3].y,  z: LIGHT_TRANSLATIONS[3].z, w:  1.0 },
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
    let light_source_vs_module = prepare_shader(
        &mut shader_compiler,
        include_str!("light_source.vert"),
        ShaderKind::Vertex,
        "light_source.vert",
        &device,
    );
    let light_source_fs_module = prepare_shader(
        &mut shader_compiler,
        include_str!("light_source.frag"),
        ShaderKind::Fragment,
        "light_source.frag",
        &device,
    );

    let mut init_encoder =
        device.create_command_encoder(&wgpu::CommandEncoderDescriptor { todo: 0 });

    let vertex_buffer = device
        .create_buffer_mapped(VERTICES.len(), wgpu::BufferUsage::VERTEX)
        .fill_from_slice(VERTICES);

    let cube_instance_buffer = device
        .create_buffer_mapped(CUBES.len(), wgpu::BufferUsage::VERTEX)
        .fill_from_slice(CUBES);
    let light_source_instance_buffer = device
        .create_buffer_mapped(LIGHT_SOURCES.len(), wgpu::BufferUsage::VERTEX)
        .fill_from_slice(&LIGHT_SOURCES);

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

    let material_bind_group_layout =
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            bindings: &[
                wgpu::BindGroupLayoutBinding {
                    binding: 0,
                    visibility: wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::UniformBuffer { dynamic: false },
                },
                wgpu::BindGroupLayoutBinding {
                    binding: 1,
                    visibility: wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::SampledTexture {
                        multisampled: false,
                        dimension: wgpu::TextureViewDimension::D2,
                    },
                },
                wgpu::BindGroupLayoutBinding {
                    binding: 2,
                    visibility: wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::Sampler,
                },
                wgpu::BindGroupLayoutBinding {
                    binding: 3,
                    visibility: wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::SampledTexture {
                        multisampled: false,
                        dimension: wgpu::TextureViewDimension::D2,
                    },
                },
                wgpu::BindGroupLayoutBinding {
                    binding: 4,
                    visibility: wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::Sampler,
                },
            ],
        });

    let mut event_channel = shrev::EventChannel::new();
    let event_reader = event_channel.register_reader();
    let mut input_handler = InputHandler::<StringBindings>::new();

    let mut camera = Camera {
        translation: Vec3::new(0.0, 0.0, 3.0),
        pitch: 0.0,
        yaw: std::f32::consts::PI,
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
        .create_buffer_mapped(1, wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST)
        .fill_from_slice(&[FragmentUniforms {
            view_translation: camera.translation.into(),
            directional_light: DirectionalLight {
                direction: Vec3::new(-0.2, -1.0, -0.3).into(),
                ambient: Vec3::new(0.05, 0.05, 0.05).into(),
                diffuse: Vec3::new(0.1, 0.1, 0.1).into(),
                specular: Vec3::new(0.1, 0.1, 0.1),
            },
            point_lights: [
                PointLight {
                    translation: LIGHT_TRANSLATIONS[0].into(),
                    ambient: Vec3::new(0.05, 0.05, 0.05).into(),
                    diffuse: Vec3::new(0.1, 0.1, 0.1).into(),
                    specular: Vec3::new(0.3, 0.3, 0.3),
                    constant: 1.0,
                    linear: 0.09,
                    quadratic: 0.032,
                },
                PointLight {
                    translation: LIGHT_TRANSLATIONS[1].into(),
                    ambient: Vec3::new(0.05, 0.05, 0.05).into(),
                    diffuse: Vec3::new(0.1, 0.1, 0.1).into(),
                    specular: Vec3::new(0.3, 0.3, 0.3),
                    constant: 1.0,
                    linear: 0.09,
                    quadratic: 0.032,
                },
                PointLight {
                    translation: LIGHT_TRANSLATIONS[2].into(),
                    ambient: Vec3::new(0.05, 0.05, 0.05).into(),
                    diffuse: Vec3::new(0.1, 0.1, 0.1).into(),
                    specular: Vec3::new(0.3, 0.3, 0.3),
                    constant: 1.0,
                    linear: 0.09,
                    quadratic: 0.032,
                },
                PointLight {
                    translation: LIGHT_TRANSLATIONS[3].into(),
                    ambient: Vec3::new(0.05, 0.05, 0.05).into(),
                    diffuse: Vec3::new(0.1, 0.1, 0.1).into(),
                    specular: Vec3::new(0.3, 0.3, 0.3),
                    constant: 1.0,
                    linear: 0.09,
                    quadratic: 0.032,
                },
            ],
            spot_light: SpotLight {
                translation: camera.translation.into(),
                direction: camera.get_direction_vector(),
                cutoff: 12.5f32.to_radians().cos(),
                outer_cutoff: 15.0f32.to_radians().cos(),
                constant: 1.0,
                linear: 0.045,
                quadratic: 0.032,
                ambient: Vec3::new(0.0, 0.0, 0.0).into(),
                diffuse: Vec3::new(0.7, 0.7, 0.7).into(),
                specular: Vec3::new(1.0, 1.0, 1.0),
            },
        }]);

    let fragment_uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: &fragment_uniform_bind_group_layout,
        bindings: &[wgpu::Binding {
            binding: 0,
            resource: wgpu::BindingResource::Buffer {
                buffer: &fragment_uniform_buffer,
                range: 0..std::mem::size_of::<FragmentUniforms>() as wgpu::BufferAddress,
            },
        }],
    });

    let material_uniform_buffer = device
        .create_buffer_mapped(1, wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST)
        .fill_from_slice(&[Material { shininess: 32.0 }]);

    let material_diffuse_texture_source = include_bytes!("container2.png");
    let material_diffuse_texture_image = image::load_from_memory(material_diffuse_texture_source)
        .unwrap()
        .to_rgba();
    let (width, height) = material_diffuse_texture_image.dimensions();
    let material_diffuse_texture_raw = material_diffuse_texture_image.to_vec();
    let (_, material_diffuse_texture_view, material_diffuse_texture_sampler) =
        create_sampled_texture2d(
            &device,
            &mut init_encoder,
            &material_diffuse_texture_raw,
            width,
            height,
        );

    let material_specular_texture_source = include_bytes!("container2_specular.png");
    let material_specular_texture_image = image::load_from_memory(material_specular_texture_source)
        .unwrap()
        .to_rgba();
    let (width, height) = material_specular_texture_image.dimensions();
    let material_specular_texture_raw = material_specular_texture_image.to_vec();
    let (_, material_specular_texture_view, material_specular_texture_sampler) =
        create_sampled_texture2d(
            &device,
            &mut init_encoder,
            &material_specular_texture_raw,
            width,
            height,
        );

    let material_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: &material_bind_group_layout,
        bindings: &[
            wgpu::Binding {
                binding: 0,
                resource: wgpu::BindingResource::Buffer {
                    buffer: &material_uniform_buffer,
                    range: 0..std::mem::size_of::<Material>() as wgpu::BufferAddress,
                },
            },
            wgpu::Binding {
                binding: 1,
                resource: wgpu::BindingResource::TextureView(&material_diffuse_texture_view),
            },
            wgpu::Binding {
                binding: 2,
                resource: wgpu::BindingResource::Sampler(&material_diffuse_texture_sampler),
            },
            wgpu::Binding {
                binding: 3,
                resource: wgpu::BindingResource::TextureView(&material_specular_texture_view),
            },
            wgpu::Binding {
                binding: 4,
                resource: wgpu::BindingResource::Sampler(&material_specular_texture_sampler),
            },
        ],
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
            &material_bind_group_layout,
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
                        format: wgpu::VertexFormat::Float3,
                        shader_location: 1,
                    },
                    wgpu::VertexAttributeDescriptor {
                        offset: (std::mem::size_of::<Vec3>() * 2) as wgpu::BufferAddress,
                        format: wgpu::VertexFormat::Float2,
                        shader_location: 2,
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
                        shader_location: 3,
                    },
                    wgpu::VertexAttributeDescriptor {
                        offset: std::mem::size_of::<Vec4>() as wgpu::BufferAddress,
                        format: wgpu::VertexFormat::Float4,
                        shader_location: 4,
                    },
                    wgpu::VertexAttributeDescriptor {
                        offset: std::mem::size_of::<Vec4>() as wgpu::BufferAddress * 2,
                        format: wgpu::VertexFormat::Float4,
                        shader_location: 5,
                    },
                    wgpu::VertexAttributeDescriptor {
                        offset: std::mem::size_of::<Vec4>() as wgpu::BufferAddress * 3,
                        format: wgpu::VertexFormat::Float4,
                        shader_location: 6,
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
    light_source_pipeline_descriptor.vertex_stage = wgpu::ProgrammableStageDescriptor {
        module: &light_source_vs_module,
        entry_point: "main",
    };
    light_source_pipeline_descriptor.fragment_stage = Some(wgpu::ProgrammableStageDescriptor {
        module: &light_source_fs_module,
        entry_point: "main",
    });
    let light_source_pipeline = device.create_render_pipeline(&light_source_pipeline_descriptor);

    let mut swap_chain = device.create_swap_chain(&surface, &swap_chain_descriptor);

    queue.submit(&[init_encoder.finish()]);

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
                        .create_buffer_mapped(6, wgpu::BufferUsage::COPY_SRC)
                        .fill_from_slice(&[
                            view_projection[0],
                            view_projection[1],
                            view_projection[2],
                            view_projection[3],
                            // TODO: Figure out why I need to invert this
                            -camera.translation.into_homogeneous_vector(),
                            camera.get_direction_vector().into_homogeneous_vector(),
                        ]);
                    encoder.copy_buffer_to_buffer(
                        &staging_buffer,
                        0,
                        &vertex_uniform_buffer,
                        0,
                        std::mem::size_of::<Mat4>() as wgpu::BufferAddress,
                    );
                    encoder.copy_buffer_to_buffer(
                        &staging_buffer,
                        std::mem::size_of::<Mat4>() as wgpu::BufferAddress,
                        &fragment_uniform_buffer,
                        0,
                        std::mem::size_of::<Vec4>() as wgpu::BufferAddress,
                    );
                    encoder.copy_buffer_to_buffer(
                        &staging_buffer,
                        std::mem::size_of::<Mat4>() as wgpu::BufferAddress,
                        &fragment_uniform_buffer,
                        400,
                        std::mem::size_of::<Vec4>() as wgpu::BufferAddress,
                    );
                    encoder.copy_buffer_to_buffer(
                        &staging_buffer,
                        (std::mem::size_of::<Mat4>() + std::mem::size_of::<Vec4>())
                            as wgpu::BufferAddress,
                        &fragment_uniform_buffer,
                        400 + std::mem::size_of::<Vec4>() as wgpu::BufferAddress,
                        std::mem::size_of::<Vec3>() as wgpu::BufferAddress,
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
                    render_pass.set_bind_group(2, &material_bind_group, &[]);
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

pub fn create_sampled_texture2d(
    device: &wgpu::Device,
    encoder: &mut wgpu::CommandEncoder,
    raw_texture: &[u8],
    width: u32,
    height: u32,
) -> (wgpu::Texture, wgpu::TextureView, wgpu::Sampler) {
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

    (texture, texture_view, texture_sampler)
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Vertex {
    pub translation: Vec3,
    pub normal: Vec3,
    pub uv: Vec2,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Instance {
    pub model: Mat4,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct FragmentUniforms {
    pub view_translation: Vec3,
    pub directional_light: DirectionalLight,
    pub point_lights: [PointLight; 4],
    pub spot_light: SpotLight,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Material {
    pub shininess: f32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct DirectionalLight {
    pub direction: Align16<Vec3>,

    pub ambient: Align16<Vec3>,
    pub diffuse: Align16<Vec3>,
    pub specular: Vec3,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct PointLight {
    pub translation: Align16<Vec3>,

    pub ambient: Align16<Vec3>,
    pub diffuse: Align16<Vec3>,
    pub specular: Vec3,

    pub constant: f32,
    pub linear: f32,
    pub quadratic: f32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct SpotLight {
    pub translation: Align16<Vec3>,
    pub direction: Vec3,
    pub cutoff: f32,
    pub outer_cutoff: f32,

    pub constant: f32,
    pub linear: f32,
    pub quadratic: f32,

    pub ambient: Align16<Vec3>,
    pub diffuse: Align16<Vec3>,
    pub specular: Vec3,
}
