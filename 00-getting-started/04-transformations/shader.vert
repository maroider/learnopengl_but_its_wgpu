#version 450

layout(location = 0) in vec3 translation;
layout(location = 1) in vec2 in_uv;

layout(location = 0) out vec2 fragment_uv;

layout(set = 0, binding = 0) uniform Uniforms {
    mat4 transform;
};

void main()
{
    gl_Position = transform * vec4(translation, 1.0);
    fragment_uv = in_uv;
}
