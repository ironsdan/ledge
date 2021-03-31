#version 450 core

layout(location = 0) in vec2 a_pos; // The position of the vertex.
layout(location = 1) in vec2 a_uv; // Texture coordinates.
layout(location = 2) in vec4 a_vert_color; // Color value.

layout(location = 3) in vec4 a_src; // Chooses the texture to use in the texture array.
layout(location = 4) in vec4 a_color;
layout(location = 5) in mat4 a_transform;

layout(binding=0,set=0) uniform mvp { // Model-View-Projection matrices
    mat4 model;
    mat4 view;
    mat4 projection;
} u_camera;

layout(location = 0) out vec2 v_uv;
layout(location = 1) out vec4 v_color;

void main() {
    v_uv = a_uv * a_src.zw + a_src.xy;
    v_color = a_vert_color * a_color;
    vec4 position = a_transform * vec4(a_pos, 0.0, 1.0);
    gl_Position = u_camera.model * u_camera.view * u_camera.projection * position;
}