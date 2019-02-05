#version 450

uniform sampler2D font_tex;

in vec2 f_tex_pos;
in vec4 f_color;

out vec4 out_color;

// psuedo random number generator
float rand(vec2 co){
  return fract(sin(dot(co.xy ,vec2(12.9898,78.233))) * 43758.5453);
}

void main() {
  float alpha = texture(font_tex, f_tex_pos).r;
  if (alpha <= 0.0) {
    discard;
  } else out_color = f_color * vec4(1, 1, 1, alpha);
}
