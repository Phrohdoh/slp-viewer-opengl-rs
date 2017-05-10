#version 330 core

in vec2 vTexCoord;
out vec4 fColor;

uniform usampler2D spriteData;
uniform sampler1D palette;

void main() {
    uint index = texture(spriteData, vTexCoord).r;
    fColor = texelFetch(palette, int(index), 0);

    // An attempt at debugging.
    if (fColor.w < 1.0) {
        fColor = vec4(0.0, 1.0, 0.0, 1.0);
    }
}