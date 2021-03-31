#version 450

layout (location = 0) in mediump vec3 position;
layout (location = 1) in uint scale;

layout(binding=0, set=0) uniform mvp { // Model-View-Projection matrices
    mat4 mvp;
} u_camera;

void main() {
    vec4 mvPosition = u_camera.mvp * vec4( position, 1.0 );
    float distance_adjust = mvPosition.z;
    if(distance_adjust == 0) {
        distance_adjust = 1.0;
    }
    gl_PointSize = scale * ( 200 / distance_adjust);
    gl_Position = mvPosition;

    // gl_PointSize = scale;
    // gl_Position = vec4(position, 1.0);
}