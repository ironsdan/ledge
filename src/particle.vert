#version 450

layout (location = 0) in mediump vec3 position;
layout (location = 1) in mediump float scale;

layout(binding=1, set=0) uniform mvp { // Model-View-Projection matrices
    mat4 model;
    mat4 view;
    mat4 projection;
} u_camera;

void main() {
    vec4 mvPosition =  u_camera.view * u_camera.model * vec4( position, 1.0 );
    float distance_adjust = mvPosition.z;
    if(distance_adjust == 0) {
        distance_adjust = 1.0;
    }
    gl_PointSize = scale * ( 300 / distance_adjust);
    vec4 test = u_camera.projection * mvPosition;
    test /= test.w;
    gl_Position =  test;
}