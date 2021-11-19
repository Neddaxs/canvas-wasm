#version 300 es

// an attribute will receive data from a buffer
in vec4 aPosition;
uniform mat4 uTransform;

// all shaders have a main function
void main() {

  gl_Position = uTransform * aPosition;
}
