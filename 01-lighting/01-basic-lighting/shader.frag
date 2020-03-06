#version 450

layout(location = 0) in vec3 translation;
layout(location = 1) in vec3 normal;

layout(location = 0) out vec4 fragment_color;

layout(set = 1, binding = 0) uniform Uniforms {
    vec3 object_color;
    vec3 light_color;
    vec3 light_translation;
    vec3 view_translation;
};

void main()
{
    float ambient_strength = 0.1;
    vec3 ambient = ambient_strength * light_color;

    vec3 normalized_normal = normalize(normal);
    vec3 light_direction = normalize(light_translation - translation);
    float diff = max(dot(normalized_normal, light_direction), 0.0);
    vec3 diffuse = diff * light_color;

    float specular_strength = 0.5;
    vec3 view_direction = normalize(view_translation - translation);
    vec3 reflect_direction = reflect(-light_direction, normalized_normal);
    float spec = pow(max(dot(view_direction, reflect_direction), 0.0), 32.0);
    vec3 specular = specular_strength * spec * light_color;

    vec3 result = (ambient + diffuse + specular) * object_color;
    fragment_color = vec4(result, 1.0);
}
