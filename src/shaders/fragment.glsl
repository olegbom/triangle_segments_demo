#version 100

uniform int bitfield;

void main() {
    if( bitfield > 2 )
    {
        gl_FragColor = vec4(1.0,0.0,0.0,1.0);
    }
    else
    {
        gl_FragColor = vec4(0.0,0.0,1.0,1.0);
    }
}
