#version 100
attribute vec2 pos;

uniform vec2 offset;
uniform float aspect;

void main() {
    vec2 p = pos + offset;
    p.y *= aspect;
    gl_Position = vec4(p, 0, 1);
}
