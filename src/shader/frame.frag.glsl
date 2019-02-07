#version 450 core
layout (location = 0) out vec4 FragColor;
layout (location = 1) out vec4 BrightColor;
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
  float brightness = dot(FragColor.rgb, vec3(0.2126, 0.7151, 0.0722));
  if(brightness > 1.0) BrightColor = vec4(FragColor.rgb, 1.0);
  else BrightColor = vec4(0.0, 0.0, 0.0, 1.0);
}