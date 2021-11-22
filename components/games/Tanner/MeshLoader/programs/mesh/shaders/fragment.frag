#version 300 es

precision mediump float;

struct DirectionalLight {
  vec3 direction;
  vec3 color;
};

in vec2 fragTexCoord;
in vec3 fragNormal;

uniform vec3 ambientLightIntensity;
uniform sampler2D sampler;
uniform DirectionalLight sun;

out vec4 myOutputColor;


void main() {
  vec3 surfaceNormal = normalize(fragNormal);
  vec3 sunDirNormal = normalize(sun.direction);

  vec4 texel = texture(sampler, fragTexCoord);

  vec3 lightIntensity = ambientLightIntensity + 
    (sun.color * 
    max(dot(surfaceNormal, sunDirNormal), 0.0));

  myOutputColor = vec4(texel.rgb * lightIntensity, texel.a);
}