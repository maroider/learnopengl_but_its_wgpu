#version 450

layout(location = 0) in vec3 translation;

layout(location = 2) in mat4 model;

layout(set = 0, binding = 0) uniform Uniforms {
    mat4 view_projection;
};

void main()
{
    gl_Position = view_projection * model * vec4(translation, 1.0);
}
