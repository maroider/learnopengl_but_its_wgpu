#version 450

layout(location = 0) in vec3 translation;
layout(location = 1) in vec3 normal;

layout(location = 0) out vec4 fragment_color;

layout(set = 1, binding = 0) uniform Uniforms {
    vec3 view_translation;
};

layout(set = 2, binding = 0) uniform Material {
    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
    float shininess;
} material;

layout(set = 3, binding = 0) uniform Light {
    vec3 translation;

    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
} light;

void main()
{
    vec3 ambient = light.ambient * material.ambient;

    vec3 normalized_normal = normalize(normal);
    vec3 light_direction = normalize(light.translation - translation);
    float diff = max(dot(normalized_normal, light_direction), 0.0);
    vec3 diffuse = light.diffuse * (diff * material.diffuse);

    vec3 view_direction = normalize(view_translation - translation);
    vec3 reflect_direction = reflect(-light_direction, normalized_normal);
    float spec = pow(max(dot(view_direction, reflect_direction), 0.0), material.shininess);
    vec3 specular = light.specular * (spec * material.specular);

    vec3 result = ambient + diffuse + specular;
    fragment_color = vec4(result, 1.0);
}
