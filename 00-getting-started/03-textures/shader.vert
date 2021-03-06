#version 450

layout(location = 0) in vec3 translation;
layout(location = 1) in vec2 in_uv;

layout(location = 0) out vec2 fragment_uv;

void main()
{
    gl_Position = vec4(translation, 1.0);
    fragment_uv = in_uv;
}
