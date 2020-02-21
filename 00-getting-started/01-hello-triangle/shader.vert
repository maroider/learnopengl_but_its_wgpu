#version 450

layout(location = 0) in vec3 translation;

void main()
{
    gl_Position = vec4(translation, 1.0);
}
