#version 330 core

in vec2 vTexCoord;

uniform usampler2D spriteData;
uniform sampler1D palette;

out vec4 color;

void main() {
    uint index = texture(spriteData, vTexCoord).r;
    color = texelFetch(palette, int(index), 0);
}