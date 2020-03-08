#version 450

layout(location = 0) in vec3 translation;
layout(location = 1) in vec3 normal;
layout(location = 2) in vec2 uv;

layout(location = 3) in mat4 model;

layout(location = 0) out vec3 frag_translation;
layout(location = 1) out vec3 frag_normal;
layout(location = 2) out vec2 frag_uv;

layout(set = 0, binding = 0) uniform Uniforms {
    mat4 view_projection;
};

void main()
{
    gl_Position = view_projection * model * vec4(translation, 1.0);
    frag_translation = vec3(model * vec4(translation, 1.0));
    frag_normal = mat3(transpose(inverse(model))) * normal;
    frag_uv = uv;
}
