#version 450

#define POINT_LIGHT_COUNT 4

layout(location = 0) in vec3 translation;
layout(location = 1) in vec3 normal;
layout(location = 2) in vec2 uv;

layout(location = 0) out vec4 fragment_color;

struct DirectionalLight {
    vec3 direction;

    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
};

struct PointLight {
    vec3 translation;

    vec3 ambient;
    vec3 diffuse;
    vec3 specular;

    float constant;
    float linear;
    float quadratic;
};

struct SpotLight {
    vec3 translation;
    vec3 direction;
    float cutoff;
    float outer_cutoff;

    float constant;
    float linear;
    float quadratic;

    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
};

layout(set = 1, binding = 0) uniform Uniforms {
    vec3 view_translation;
    DirectionalLight directional_light;
    PointLight point_lights[POINT_LIGHT_COUNT];
    SpotLight spot_light;
};

layout(set = 2, binding = 0) uniform Material {
    float shininess;
} material;
layout(set = 2, binding = 1) uniform texture2D material_diffuse_texture;
layout(set = 2, binding = 2) uniform sampler material_diffuse_sampler;
layout(set = 2, binding = 3) uniform texture2D material_specular_texture;
layout(set = 2, binding = 4) uniform sampler material_specular_sampler;

vec3 calculate_directional_light(DirectionalLight light, vec3 normal, vec3 view_direction);
vec3 calculate_point_light(PointLight light, vec3 normal, vec3 frag_translation, vec3 view_direction);
vec3 calculate_spot_light(SpotLight light, vec3 normal, vec3 frag_translation, vec3 view_direction);

void main()
{
    vec3 normalized_normal = normalize(normal);
    vec3 view_direction = normalize(view_translation - translation);

    vec3 result = calculate_directional_light(directional_light, normalized_normal, view_direction);
    for (int i = 0; i < POINT_LIGHT_COUNT; i++) {
        result += calculate_point_light(point_lights[i], normalized_normal, translation, view_direction);
    }
    result += calculate_spot_light(spot_light, normalized_normal, translation, view_direction);

    fragment_color = vec4(result, 1.0);
}

vec3 calculate_directional_light(DirectionalLight light, vec3 normal, vec3 view_direction)
{
    vec3 light_direction = normalize(-light.direction);

    float diff = max(dot(normal, light_direction), 0.0);

    vec3 reflect_direction = reflect(-light_direction, normal);
    float spec = pow(max(dot(view_direction, reflect_direction), 0.0), material.shininess);

    vec3 ambient = light.ambient * vec3(texture(sampler2D(material_diffuse_texture, material_diffuse_sampler), uv));
    vec3 diffuse = light.diffuse * diff * vec3(texture(sampler2D(material_diffuse_texture, material_diffuse_sampler), uv));
    vec3 specular = light.specular * spec * vec3(texture(sampler2D(material_specular_texture, material_specular_sampler), uv));
    return (ambient + diffuse + specular);
}

vec3 calculate_point_light(PointLight light, vec3 normal, vec3 frag_translation, vec3 view_direction)
{
    vec3 light_direction = normalize(light.translation - frag_translation);

    float diff = max(dot(normal, light_direction), 0.0);

    vec3 reflect_direction = reflect(-light_direction, normal);
    float spec = pow(max(dot(view_direction, reflect_direction), 0.0), material.shininess);

    float distance = length(light.translation - frag_translation);
    float attenuation = 1.0 / (light.constant + light.linear * distance + light.quadratic * (distance * distance));

    vec3 ambient = light.ambient * vec3(texture(sampler2D(material_diffuse_texture, material_diffuse_sampler), uv));
    vec3 diffuse = light.diffuse * vec3(texture(sampler2D(material_diffuse_texture, material_diffuse_sampler), uv));
    vec3 specular = light.specular * spec * vec3(texture(sampler2D(material_specular_texture, material_specular_sampler), uv));
    ambient *= attenuation;
    diffuse *= attenuation;
    specular *= attenuation;
    return (ambient + diffuse + specular);
}

vec3 calculate_spot_light(SpotLight light, vec3 normal, vec3 frag_translation, vec3 view_direction)
{
    vec3 light_direction = normalize(light.translation - frag_translation);

    float diff = max(dot(normal, light_direction), 0.0);

    vec3 reflect_direction = reflect(-light_direction, normal);
    float spec = pow(max(dot(view_direction, reflect_direction), 0.0), material.shininess);

    float distance = length(light.translation - frag_translation);
    float attenuation = 1.0 / (light.constant + light.linear * distance + light.quadratic * (distance * distance));

    float theta = dot(light_direction, normalize(-light.direction));
    float epsilon = light.cutoff - light.outer_cutoff;
    float intensity = clamp((theta - light.outer_cutoff) / epsilon, 0.0, 1.0);

    vec3 ambient = light.ambient * vec3(texture(sampler2D(material_diffuse_texture, material_diffuse_sampler), uv));
    vec3 diffuse = light.diffuse * diff * vec3(texture(sampler2D(material_diffuse_texture, material_diffuse_sampler), uv));
    vec3 specular = light.specular * spec * vec3(texture(sampler2D(material_specular_texture, material_specular_sampler), uv));
    ambient *= attenuation * intensity;
    diffuse *= attenuation * intensity;
    specular *= attenuation * intensity;
    return (ambient + diffuse + specular);
}
