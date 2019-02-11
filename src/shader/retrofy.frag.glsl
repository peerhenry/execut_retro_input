#version 450 core
layout (location = 0) out vec4 FragColor;
layout (location = 1) out vec4 BrightColor;
in vec2 TexCoords;

uniform float baseRand;
uniform sampler2D screenTexture;
// todo: uniform float linePos;

subroutine vec4 RenderPassType();
subroutine uniform RenderPassType RenderPass;

// psuedo random number generator
float rand(vec2 co)
{
  return fract(sin(dot(co.xy ,vec2(12.9898,78.233))) * 43758.5453 * baseRand);
}

subroutine (RenderPassType)
vec4 noise()
{
  float r = rand(TexCoords);
  vec4 noise = vec4(vec3(0.2*r), 1);
  return texture(screenTexture, TexCoords) + noise;
  // float brightness = dot(FragColor.rgb, vec3(0.2126, 0.7151, 0.0722));
  // if(brightness > 1.0) BrightColor = vec4(FragColor.rgb, 1.0);
  // else BrightColor = vec4(0.0, 0.0, 0.0, 1.0);
}

subroutine (RenderPassType)
vec4 extractBright()
{
  // todo
  // float brightness = dot(FragColor.rgb, vec3(0.2126, 0.7151, 0.0722));
  // if(brightness > 1.0) BrightColor = vec4(FragColor.rgb, 1.0);
  // else BrightColor = vec4(0.0, 0.0, 0.0, 1.0);
  return texture(screenTexture, TexCoords);
}

subroutine (RenderPassType)
vec4 blurVertically()
{
  // todo
  return vec4(0.0, 1.0, 0.0, 1.0);
}

subroutine (RenderPassType)
vec4 blurHorizontallyAndJoin()
{
  // todo
  return vec4(0.0, 0.0, 1.0, 1.0);
}

void main()
{
  FragColor = RenderPass();
}