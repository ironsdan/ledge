#version 450

layout (location=0) out mediump vec4 f_color;

layout(binding=0,set=0) uniform color {
    mediump vec3 value;
} v_color;

void main() {
    if ( length( gl_PointCoord - vec2( 0.5, 0.5 ) ) > 0.475 || length( gl_PointCoord - vec2( 0.5, 0.5 ) ) < 0.375 ) {
        discard;
    }
    f_color = vec4( v_color.value, 1.0 );
}