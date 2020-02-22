#version 450

layout(location = 0) in vec2 uv;

layout(location = 0) out vec4 fragment_color;

layout(set = 0, binding = 0) uniform texture2D texture_1;
layout(set = 0, binding = 1) uniform sampler texture_1_sampler;

layout(set = 1, binding = 0) uniform texture2D texture_2;
layout(set = 1, binding = 1) uniform sampler texture_2_sampler;

void main()
{
    fragment_color = mix(
        texture(sampler2D(texture_1, texture_1_sampler), uv),
        texture(sampler2D(texture_2, texture_2_sampler), vec2(1.0 - uv.x, uv.y)),
        0.3
    );
}
