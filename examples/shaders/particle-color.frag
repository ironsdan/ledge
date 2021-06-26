#version 450

layout (location=0) in mediump vec3 v_color;
layout (location=0) out mediump vec4 f_color;

void main() {
    if ( length( gl_PointCoord - vec2( 0.5, 0.5 ) ) > 0.475 ) {
        discard;
    }
    f_color = vec4( v_color, 1.0 );
}