#version 450

layout (location = 0) in mediump vec3 position;
layout (location = 1) in mediump float scale;
layout (location = 2) in mediump vec3 color;

layout(binding=0, set=0) uniform mvp { // Model-View-Projection matrices
    mat4 model;
    mat4 view;
    mat4 proj;
} u_camera;

layout (location = 0) out mediump vec3 v_color;

void main() {
    vec4 mvPosition =  u_camera.view * u_camera.model * vec4( position, 1.0 );
    float distance_adjust = mvPosition.z;
    if(distance_adjust == 0) {
        distance_adjust = 1.0;
    }
    gl_PointSize = scale * ( 300 / distance_adjust);
    vec4 test = u_camera.proj * mvPosition;
    test /= test.w;

    v_color = color;

    gl_Position =  test;
}