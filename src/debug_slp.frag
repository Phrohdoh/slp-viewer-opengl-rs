#version 330 core

in vec2 vTexCoord;

const uint CMD_SKIP = uint(0);
const uint CMD_COLOR = uint(1);
const uint CMD_REMAP = uint(2);
const uint CMD_SHADOW = uint(3);

const vec4 C_COLOR = vec4(0.0, 0.0, 1.0, 1.0);
const vec4 C_SHADOW = vec4(0.0, 1.0, 0.0, 1.0);
const vec4 C_REMAP = vec4(1.0, 0.0, 0.0, 1.0);

uniform usampler2D spriteCmds;

uniform sampler1D palette;

out vec4 color;

void main() {
    uint cmd = texture(spriteCmds, vTexCoord).r;

    if (cmd == CMD_SKIP) {
        discard;
    } else if (cmd == CMD_REMAP) {
        color = C_REMAP;
    } else if (cmd == CMD_SHADOW) {
        color = C_SHADOW;
    } else {
        color = C_COLOR;
    }
}
