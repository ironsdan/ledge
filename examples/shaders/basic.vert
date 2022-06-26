#version 450 core

layout(location = 0) in vec3 pos; // The position of the vertex.
layout(location = 1) in vec2 uv; // Texture coordinates.
layout(location = 2) in vec4 vert_color; // Color value.

layout(location = 3) in vec4 src; // Chooses the texture to use in the texture array.
layout(location = 4) in vec4 color;
layout(location = 5) in mat4 transform;

layout(binding=0,set=0) uniform mvp {
    mat4 mvp;
} camera;


layout(location = 0) out vec2 v_uv;
layout(location = 1) out vec4 v_color;

void main() {
    v_uv = vec2(uv * src.zw + src.xy);
    v_color = vert_color * color;
    vec4 position = transform * vec4(pos, 1.0);
    // gl_Position = camera.model * camera.view * camera.projection * position;
    gl_Position = camera.mvp * position;
}