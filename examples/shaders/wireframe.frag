#version 450

layout (location=0) in highp vec3 vbc;

layout (location=0) out mediump vec4 f_color;

layout(binding=0,set=0) uniform color {
    mediump vec3 value;
} u_color;

float edgeFactor() {
    vec3 d = fwidth(vbc);
    vec3 f = smoothstep(vec3(0.0), d * 1.0, vbc);
    return min(min(f.x, f.y), f.z);
}

void main() {
    f_color = vec4(u_color.value, (1.0-edgeFactor())*0.95);
}