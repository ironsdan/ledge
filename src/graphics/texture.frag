#version 450

layout(location = 0) in vec2 v_Uv;
layout(location = 1) in vec4 v_Color;

layout(set = 0, binding = 0) uniform sampler2D t_Tex;

layout(location = 0) out vec4 f_Color;

void main() {
    f_Color = texture(t_Tex, v_Uv) * v_Color;
}