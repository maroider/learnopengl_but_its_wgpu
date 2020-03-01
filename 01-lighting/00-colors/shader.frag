#version 450

layout(location = 0) out vec4 fragment_color;

layout(set = 1, binding = 0) uniform Uniforms {
    vec3 object_color;
    vec3 light_color;
};

void main()
{
    fragment_color = vec4(light_color * object_color, 1.0);
}
