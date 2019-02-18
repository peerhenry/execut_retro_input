#version 450 core
layout (location = 0) out vec4 FragColor;
layout (location = 1) out vec4 BrightColor;
in vec2 TexCoords;

uniform float baseRand;
uniform sampler2D screenTexture;
uniform vec4 retroColorLeft;
uniform float linePosLeft;
float lineDelta = 0.002;

uniform vec4 retroColorRight;
uniform float linePosRight;

subroutine vec4 RenderPassType();
subroutine uniform RenderPassType RenderPass;

// psuedo random number generator
float rand(vec2 co)
{
  return fract(sin(dot(co.xy ,vec2(12.9898,78.233))) * 43758.5453 * baseRand);
}

vec4 getRetroColoring(float linePos, vec4 retroColor)
{
  float u = TexCoords.x;
  float v = TexCoords.y;
  float amplitude = u*v*0.05 + 0.05;
  float lineDiff = abs(linePos - v);
  float lineAmp = 0.0;
  if(lineDiff < lineDelta) lineAmp = (lineDelta - lineDiff)/lineDelta;
  float darkLightDelta = 0.05;
  float dark = (darkLightDelta/2)*(3-linePos);
  amplitude += dark;
  if(v < linePos) amplitude += darkLightDelta;
  return (amplitude + lineAmp)*retroColor;
}

vec4 getLeftSideFragment()
{
  return getRetroColoring(linePosLeft, retroColorLeft);
}

vec4 getRightSideFragment()
{
  return getRetroColoring(linePosRight, retroColorRight);
}

vec4 getColoring()
{
  if(TexCoords.x < 0.5) return getLeftSideFragment();
  else return getRightSideFragment();
}

subroutine (RenderPassType)
vec4 noise()
{
  float r = rand(TexCoords);
  vec4 noise = vec4(vec3(0.05*r), 1);
  vec4 coloring = getColoring();
  return texture(screenTexture, TexCoords) + noise + coloring;
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