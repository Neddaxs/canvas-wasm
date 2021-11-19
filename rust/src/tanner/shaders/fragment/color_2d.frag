#version 300 es

precision mediump float;
uniform vec4 uColor;
uniform float uOpacity;
out vec4 myOutputColor;
    
void main() {
    myOutputColor = vec4(uColor.r, uColor.g, uColor.b, uColor.a * uOpacity);
}
