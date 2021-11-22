#version 300 es

precision mediump float;

in vec2 fragTexCoord;
out vec4 myOutputColor;
uniform sampler2D sampler;

void main() {
  myOutputColor = texture(sampler, fragTexCoord);
}