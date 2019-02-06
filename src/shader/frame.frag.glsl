#version 450 core
out vec4 FragColor;
in vec2 TexCoords;
uniform sampler2D screenTexture;

uniform float baseRand;

// psuedo random number generator
float rand(vec2 co)
{
  return fract(sin(dot(co.xy ,vec2(12.9898,78.233))) * 43758.5453 * baseRand);
}

void main()
{
  float r = rand(TexCoords);
  FragColor = texture(screenTexture, TexCoords) + vec4(vec3(0.2*r), 1);
}