#version 450 core

layout(location = 0) in vec2 a_pos; // The position of the vertex.

// Instance specific
layout(location = 1) in vec2 a_uv; // Texture coordinates.
layout(location = 2) in vec4 a_color; // Color value.
// layout(location = 3) in vec4 a_src; // Need to figure this one out.

layout(binding=0,set=1) uniform mvp { // Model-View-Projection matrices
    mat4 model;
    mat4 view;
    mat4 proj;
} camera;

layout(binding=0,set=2) uniform instance_data {
    mat4 transform;
} instance;

layout(location = 0) out vec2 v_uv;
layout(location = 1) out vec4 v_color;

void main() {
    v_uv = a_uv;
    v_color = a_color;
    vec4 position = vec4(a_pos, 0.0, 1.0);
    gl_Position = camera.proj * camera.view * camera.model * position;
}