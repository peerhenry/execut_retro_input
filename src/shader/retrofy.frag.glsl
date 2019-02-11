#version 450 core
layout (location = 0) out vec4 FragColor;
layout (location = 1) out vec4 BrightColor;
in vec2 TexCoords;

uniform float baseRand;
uniform sampler2D screenTexture;
uniform vec4 retroColor;
uniform float linePos;
float lineDelta = 0.002;

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
  vec4 noise = vec4(vec3(0.05*r), 1);
  float amplitude = TexCoords.x*TexCoords.y*0.05 + 0.05;
  float lineDiff = abs(linePos - TexCoords.y);
  float lineAmp = 0.0;
  if(lineDiff < lineDelta) lineAmp = (lineDelta - lineDiff)/lineDelta;
  
  float darkLightDelta = 0.05;
  float dark = (darkLightDelta/2)*(3-linePos);
  amplitude += dark;
  if(TexCoords.y < linePos) amplitude += darkLightDelta;

  /*vec3 texAverage = vec3(0.0);
  for(int i=-3;i<4;i++)
  {
    for(int j=-3;j<4;j++)
    {
      texAverage += (texture(screenTexture, TexCoords + vec2(i,j)).xyz)/49.0;
    }
  }
  vec4 texCol = vec4(texAverage, 1);
  return texCol + noise + (amplitude + lineAmp)*retroColor; //*/
  return texture(screenTexture, TexCoords) + noise + (amplitude + lineAmp)*retroColor;
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