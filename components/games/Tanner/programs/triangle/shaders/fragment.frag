#version 300 es

precision mediump float;

in vec3 fragColor;
out vec4 myOutputColor;

void main() {
  myOutputColor = vec4(fragColor, 1.0);
}