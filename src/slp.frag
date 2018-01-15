#version 330 core

in vec2 vTexCoord;

const uint CMD_SKIP = uint(0);
const uint CMD_COLOR = uint(1);
const uint CMD_REMAP = uint(2);
const uint CMD_SHADOW = uint(3);

uniform usampler2D spriteCmds;
uniform usampler2D spriteData;

uniform sampler1D palette;

out vec4 color;

void main() {
    uint index = texture(spriteData, vTexCoord).r;
    uint cmd = texture(spriteCmds, vTexCoord).r;

    if (cmd == CMD_SKIP) {
        discard;
    } else if (cmd == CMD_SHADOW) {
        color = vec4(0.0, 0.0, 1.0, 1.0);
    } else {
        color = texelFetch(palette, int(index), 0);
    }
}