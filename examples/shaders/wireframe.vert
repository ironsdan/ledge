#version 450

layout (location=0) in mediump vec3 position;
layout (location=1) in mediump vec3 barycenter;

layout(binding=1, set=0) uniform mvp { // Model-View-Projection matrices
    mat4 model;
    mat4 view;
    mat4 proj;
} u_camera;

layout (location=0) out highp vec3 vbc;

void main() {
    vbc = barycenter;
    vec4 mvPosition = u_camera.proj * u_camera.view * u_camera.model * vec4( position, 1.0 );
    mvPosition /= mvPosition.w;
    gl_Position =  mvPosition;
}