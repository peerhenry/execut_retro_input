#version 450
out vec4 FragColor;
in vec2 TexCoords;
uniform sampler2D screenTexture;

void main()
{
    FragColor = texture(screenTexture, TexCoords) + vec4(0, 0.5, 0.5, 1);
}