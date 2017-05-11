#version 330 core

in vec2 vTexCoord;

uniform usampler2D spriteData;
uniform sampler1D palette;

out vec4 color;

void main() {
    uint index = texture(spriteData, vTexCoord).r;
    int i = int(index);

    if (i == 0) {
        discard;
    } else if (i == 1) {
        color = vec4(0.0, 0.0, 0.0, 0.75);
    } else {
        color = texelFetch(palette, i, 0);
    }
}