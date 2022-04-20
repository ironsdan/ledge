#version 450

layout(location=0) in vec2 v_uv;
layout(location=1) in vec4 v_color;

layout(binding=0,set=1) uniform sampler2D t_tex;

layout(location=0) out vec4 f_color;

void main() {
    f_color = texture(t_tex, v_uv) * v_color;
}